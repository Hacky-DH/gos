//! GOS Decompiler - converts JSON data back to GOS source code
//! 
//! This module provides functionality to decompile GOS JSON format back to GOS source code.
//! It supports various formatting options including indentation, line wrapping, and string escaping.

use std::fs;
use std::path::Path;
use serde_json::Value;
use regex::Regex;
use std::cell::RefCell;

/// Options for decompilation process
#[derive(Debug, Clone)]
pub struct DecompileOptions {
    pub indent: usize,
    pub max_col: usize,
    pub unescape: bool,
    pub keep_order: bool,
}

impl Default for DecompileOptions {
    fn default() -> Self {
        Self {
            indent: 4,
            max_col: 100,
            unescape: false,
            keep_order: false,
        }
    }
}

/// Result of decompilation process
#[derive(Debug, Clone)]
pub enum DecompileResult {
    Text(String),
    Structured {
        grl: String,
        std: Value,
        source_json_kind: String,
    },
}

thread_local! {
    /// Thread-local options for the decompilation process
    static OPTIONS: RefCell<DecompileOptions> = RefCell::new(DecompileOptions::default());
}

/// Valid identifier pattern (extended from Python version)
static VALID_IDENTIFIER: &str = r"^[a-zA-Z_\-$%@][a-zA-Z_\-$%@\.0-9]*$";
static VALID_VERSION: &str = r"^[0-9]+\.[0-9]+\.[0-9]+$";

/// Decompile from JSON data
pub fn decompile_from_data(
    content: Value,
    options: Option<DecompileOptions>,
) -> Result<DecompileResult, String> {
    let mut content = content;
    let options = options.unwrap_or_default();
    
    // Set thread-local options
    OPTIONS.with(|opts| {
        *opts.borrow_mut() = options.clone();
    });
    
    // Handle unescaping if requested
    if options.unescape {
        content = unescape_dfs(&content);
    }
    
    // For now, assume standard JSON format
    // TODO: Add plugin detection and conversion logic
    let grl_text = decompile_std(&content)?;
    
    Ok(DecompileResult::Text(grl_text))
}

/// Decompile from file
pub fn decompile(
    filename: &str,
    options: Option<DecompileOptions>,
) -> Result<DecompileResult, String> {
    let path = Path::new(filename);
    if !path.exists() {
        return Err(format!("File {} not found", filename));
    }
    
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;
    
    let json_value: Value = serde_json::from_str(&content)
        .map_err(|e| format!("File {} is not valid JSON: {}", filename, e))?;
    
    decompile_from_data(json_value, options)
}

/// Recursively unescape strings in JSON data
fn unescape_dfs(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (key, val) in map {
                new_map.insert(key.clone(), unescape_dfs(val));
            }
            Value::Object(new_map)
        }
        Value::Array(arr) => {
            Value::Array(arr.iter().map(unescape_dfs).collect())
        }
        Value::String(s) => {
            // Simple unescape - replace common escape sequences
            let unescaped = s.replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\r", "\r")
                .replace("\\\\", "\\")
                .replace("\\\"", "\"")
                .replace("\\'", "'");
            Value::String(unescaped)
        }
        _ => value.clone(),
    }
}

/// Main decompilation function for standard JSON format
fn decompile_std(std_data: &Value) -> Result<String, String> {
    if !std_data.is_object() {
        return Err("Decompile input must be a JSON object".to_string());
    }
    
    let mut buffer = String::new();
    
    // Handle graphs
    if let Some(graphs) = std_data.get("graphs") {
        if let Some(graphs_array) = graphs.as_array() {
            for (index, graph) in graphs_array.iter().enumerate() {
                decompile_graph(&mut buffer, graph)?;
                if index < graphs_array.len() - 1 {
                    buffer.push_str("\n\n");
                }
            }
        } else {
            return Err("Graphs must be an array".to_string());
        }
    }
    
    // Handle operations
    if let Some(ops) = std_data.get("ops") {
        if let Some(ops_array) = ops.as_array() {
            for (index, op) in ops_array.iter().enumerate() {
                decompile_op(&mut buffer, op)?;
                if index < ops_array.len() - 1 {
                    buffer.push_str("\n\n");
                }
            }
        }
    }
    
    // Handle nodes
    if let Some(nodes) = std_data.get("nodes") {
        if let Some(nodes_obj) = nodes.as_object() {
            for (node_as, node) in nodes_obj {
                let decompiler = NodeDecompiler::new(node_as, node);
                decompiler.decompile(&mut buffer)?;
            }
        }
    }
    
    Ok(buffer)
}

/// Decompile a single graph
fn decompile_graph(buffer: &mut String, graph: &Value) -> Result<(), String> {
    if !graph.is_object() {
        return Err("Graph must be a JSON object".to_string());
    }
    
    let template_graph = graph.get("template_graph").and_then(|v| v.as_str());
    
    if let Some(tpl) = template_graph {
        let checked_tpl = check_id(tpl)?;
        buffer.push_str(&format!("graph : {}", checked_tpl));
        
        if let Some(tpl_version) = graph.get("template_version").and_then(|v| v.as_str()) {
            let checked_version = check_version(tpl_version)?;
            buffer.push_str(&format!(".version('{}')", checked_version));
        }
        buffer.push_str(" {");
    } else {
        buffer.push_str("graph {");
    }
    
    let options = OPTIONS.with(|opts| opts.borrow().clone());
    
    // Handle properties
    if let Some(props) = graph.get("property") {
        indent(buffer, options.indent);
        let mut param_formatter = ParamFormatter::new(props, ',');
        param_formatter.format(buffer, options.indent)?;
        buffer.push(';');
    }
    
    // Handle nodes
    if let Some(nodes) = graph.get("nodes") {
        if let Some(nodes_obj) = nodes.as_object() {
            for (node_as, node) in nodes_obj {
                let decompiler = NodeDecompiler::new(node_as, node);
                decompiler.decompile(buffer)?;
            }
        }
    }
    
    if options.indent > 0 {
        buffer.push('\n');
    }
    buffer.push('}');
    
    // Handle alias and version
    if let Some(graph_as) = graph.get("as").and_then(|v| v.as_str()) {
        let checked_as = check_id(graph_as)?;
        buffer.push_str(&format!(" as {}", checked_as));
        
        if let Some(graph_version) = graph.get("version").and_then(|v| v.as_str()) {
            let checked_version = check_version(graph_version)?;
            buffer.push_str(&format!(".version('{}')", checked_version));
        }
    }
    
    buffer.push(';');
    Ok(())
}

/// Node decompiler - handles individual node decompilation
struct NodeDecompiler<'a> {
    node_as: &'a str,
    node: &'a Value,
}

impl<'a> NodeDecompiler<'a> {
    fn new(node_as: &'a str, node: &'a Value) -> Self {
        Self { node_as, node }
    }
    
    fn decompile(&self, buffer: &mut String) -> Result<(), String> {
        let options = OPTIONS.with(|opts| opts.borrow().clone());
        
        // Check for outputs
        let outputs = self.node.get("output")
            .and_then(|v| v.as_array())
            .ok_or_else(|| format!("Node {} has no output", self.node_as))?;
        
        indent(buffer, options.indent);
        
        let output_key = outputs.iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join(",");
        
        let has_as = output_key != self.node_as;
        
        // Handle outputs
        if has_as {
            let simplified_outputs: Vec<&str> = outputs.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.split('.').last().unwrap_or(s))
                .collect();
            
            let _col = self.indent_list(&simplified_outputs, options.indent, ",", buffer);
            buffer.push_str(" = ");
        } else {
            buffer.push_str(&output_key);
            buffer.push_str(" = ");
        }
        
        // Handle for loop
        if let Some(for_loop) = self.node.get("for_loop").and_then(|v| v.as_object()) {
            if !for_loop.is_empty() 
                && for_loop.get("inputs").is_some() 
                && for_loop.get("outputs").is_some() {
                return self.for_loop(for_loop, buffer);
            }
        }
        
        // Handle condition node
        if let Some(op_name) = self.node.get("op_name").and_then(|v| v.as_str()) {
            if op_name == "builtin.conditions.str" {
                return self.condition_node(buffer);
            }
        }
        
        // Regular node block
        self.node_block(buffer, has_as)?;
        buffer.push(';');
        Ok(())
    }
    
    fn for_loop(&self, for_loop: &serde_json::Map<String, Value>, buffer: &mut String) -> Result<(), String> {
        buffer.push('[');
        self.node_block(buffer, true)?; // has_as is true for for loops
        
        let for_inputs = for_loop.get("inputs").and_then(|v| v.as_str()).unwrap_or("");
        let for_outputs = for_loop.get("outputs");
        
        let for_outputs_str = if let Some(outputs) = for_outputs {
            if let Some(arr) = outputs.as_array() {
                arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", ")
            } else if let Some(s) = outputs.as_str() {
                s.to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        
        let options = OPTIONS.with(|opts| opts.borrow().clone());
        let indent_ = options.indent * 2;
        indent(buffer, indent_);
        buffer.push_str(&format!("for {} in {}", for_outputs_str, for_inputs));
        
        if let Some(for_condition) = for_loop.get("condition").and_then(|v| v.as_str()) {
            let indent_ = options.indent * 2;
            indent(buffer, indent_);
            buffer.push_str(&format!("if {}", for_condition));
        }
        
        buffer.push_str("];");
        Ok(())
    }
    
    fn condition_node(&self, buffer: &mut String) -> Result<(), String> {
        let condition = self.node.get("condition")
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("Condition node {} must have string condition", self.node_as))?;
        
        buffer.push_str(&format!("{} ? ", condition));
        
        let true_branch = self.node.get("true_branch")
            .ok_or_else(|| format!("Condition node {} must have true branch", self.node_as))?;
        
        self.node_block_from_value(true_branch, buffer, false, self.node_as)?;
        
        buffer.push_str(" : ");
        
        let false_branch = self.node.get("false_branch")
            .ok_or_else(|| format!("Condition node {} must have false branch", self.node_as))?;
        
        self.node_block_from_value(false_branch, buffer, false, self.node_as)?;
        
        buffer.push(';');
        Ok(())
    }
    
    fn node_block(&self, buffer: &mut String, has_as: bool) -> Result<(), String> {
        self.node_block_from_value(self.node, buffer, has_as, self.node_as)
    }
    
    fn node_block_from_value(&self, node: &Value, buffer: &mut String, has_as: bool, node_as: &str) -> Result<(), String> {
        let options = OPTIONS.with(|opts| opts.borrow().clone());
        
        let name = if let Some(ref_graph) = node.get("ref_graph").and_then(|v| v.as_str()) {
            buffer.push_str("ref(");
            ref_graph
        } else if let Some(op_name) = node.get("op_name").and_then(|v| v.as_str()) {
            op_name
        } else {
            return Err(format!("Node {} has no op_name or ref_graph", node_as));
        };
        
        let checked_name = check_id(name)?;
        buffer.push_str(&format!("{}(", checked_name));
        
        // Handle inputs
        if let Some(inputs) = node.get("input") {
            if let Some(inputs_array) = inputs.as_array() {
                // Handle array inputs
                let input_strings: Vec<String> = inputs_array.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                let input_refs: Vec<&str> = input_strings.iter().map(|s| s.as_str()).collect();
                let _col = self.indent_inputs(&input_refs, options.indent * 2, ",", buffer);
            } else if let Some(inputs_obj) = inputs.as_object() {
                // Handle key-value inputs
                let mut input_strings = Vec::new();
                for (k, v) in inputs_obj {
                    input_strings.push(format!("{}={}", k, input_str(v)));
                }
                let input_refs: Vec<&str> = input_strings.iter().map(|s| s.as_str()).collect();
                let _col = self.indent_inputs(&input_refs, options.indent * 2, ",", buffer);
            }
        }
        
        buffer.push(')');
        
        if node.get("ref_graph").is_some() {
            buffer.push(')');
        }
        
        // Handle attributes
        if let Some(attrs) = node.get("attrs").and_then(|v| v.as_array()) {
            for attr in attrs {
                if let Some(attr_obj) = attr.as_object() {
                    if let (Some(key), Some(value)) = (attr_obj.get("key"), attr_obj.get("value")) {
                        if let (Some(key_str), Some(value_str)) = (key.as_str(), value.as_str()) {
                            self.indent_str(buffer, &format!(".{}({})", key_str, value_str), 0);
                        }
                    }
                }
            }
        }
        
        // Handle version
        if let Some(version) = node.get("version").and_then(|v| v.as_str()) {
            self.indent_str(buffer, &format!(".version('{}')", version), 0);
        }
        
        // Handle alias
        if has_as {
            let checked_as = check_id(node_as)?;
            self.indent_str(buffer, &format!(".as({})", checked_as), 0);
        }
        
        // Handle start/end markers
        if node.get("start").is_some() {
            self.indent_str(buffer, ".as(start)", 0);
        }
        if node.get("end").is_some() {
            self.indent_str(buffer, ".as(end)", 0);
        }
        
        // Handle dependencies
        if let Some(depends) = node.get("depend").and_then(|v| v.as_array()) {
            let depends_str = depends.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(",");
            self.indent_str(buffer, &format!(".depend({})", depends_str), 0);
        }
        
        // Handle override
        if let Some(override_val) = node.get("override") {
            let override_str = match override_val {
                Value::Bool(b) => b.to_string(),
                Value::Null => String::new(),
                _ => override_val.to_string(),
            };
            self.indent_str(buffer, &format!(".override({})", override_str), 0);
        }
        
        // Handle other properties
        let param_map = [
            ("property", "property"),
            ("with", "with"),
            ("log", "log"),
            ("metrics", "metrics"),
            ("funnel", "funnel"),
        ];
        
        for (key, prefix) in param_map {
            if let Some(value) = node.get(key) {
                let options = OPTIONS.with(|opts| opts.borrow().clone());
                let indent_ = options.indent * 2;
                indent(buffer, indent_);
                buffer.push_str(&format!(".{}(", prefix));
                let mut param_formatter = ParamFormatter::new(value, ',');
                param_formatter.format(buffer, indent_ + prefix.len() + 1)?;
                buffer.push(')');
            }
        }
        
        Ok(())
    }
    
    fn indent_list(&self, inputs: &[&str], col: usize, delimiter: &str, buffer: &mut String) -> usize {
        let options = OPTIONS.with(|opts| opts.borrow().clone());
        let candidate = inputs.join(delimiter);
        
        if col + candidate.len() > options.max_col && options.indent > 0 {
            let mut current_col = col;
            for (i, item) in inputs.iter().enumerate() {
                current_col += options.indent * 2 + item.len() + 1;
                if current_col > options.max_col {
                    indent(buffer, options.indent * 2);
                    current_col = options.indent * 2;
                }
                buffer.push_str(item);
                if i < inputs.len() - 1 {
                    buffer.push_str(delimiter);
                }
            }
            current_col
        } else {
            buffer.push_str(&candidate);
            col + candidate.len()
        }
    }
    
    fn indent_inputs(&self, inputs: &[&str], col: usize, delimiter: &str, buffer: &mut String) -> usize {
        let options = OPTIONS.with(|opts| opts.borrow().clone());
        let candidate: String = inputs.iter()
            .map(|&item| self.str_input(item))
            .collect::<Vec<_>>()
            .join(delimiter);
        
        if col + candidate.len() > options.max_col && options.indent > 0 {
            let mut current_col = col;
            for (i, item) in inputs.iter().enumerate() {
                current_col += options.indent * 2 + item.len() + 1;
                if current_col > options.max_col {
                    indent(buffer, options.indent * 2);
                    current_col = options.indent * 2;
                }
                buffer.push_str(&self.str_input(item));
                if i < inputs.len() - 1 {
                    buffer.push_str(delimiter);
                }
            }
            current_col
        } else {
            buffer.push_str(&candidate);
            col + candidate.len()
        }
    }
    
    fn str_input(&self, data: &str) -> String {
        // Simple implementation - in real version would handle dict parsing
        data.to_string()
    }
    
    fn indent_str(&self, buffer: &mut String, input: &str, col: usize) -> usize {
        let options = OPTIONS.with(|opts| opts.borrow().clone());
        
        if col + input.len() > options.max_col && options.indent > 0 {
            let indent_ = options.indent * 2;
            indent(buffer, indent_);
            buffer.push_str(input);
            indent_ + input.len()
        } else {
            buffer.push_str(input);
            col + input.len()
        }
    }
}

/// Parameter formatter for handling complex parameter structures
struct ParamFormatter<'a> {
    inputs: &'a Value,
    delimiter: char,
}

impl<'a> ParamFormatter<'a> {
    fn new(inputs: &'a Value, delimiter: char) -> Self {
        Self { inputs, delimiter }
    }
    
    fn format(&mut self, buffer: &mut String, col: usize) -> Result<usize, String> {
        if let Some(obj) = self.inputs.as_object() {
            let mut strings = Vec::new();
            for (k, v) in obj {
                strings.push(format!("{}={}", k, self.format_value(v)));
            }
            
            let candidate = strings.iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(&self.delimiter.to_string());
            
            let options = OPTIONS.with(|opts| opts.borrow().clone());
            
            if col + candidate.len() > options.max_col && options.indent > 0 {
                let mut current_col = col;
                for (i, (k, v)) in obj.iter().enumerate() {
                    current_col += strings[i].len() + 1;
                    if current_col > options.max_col {
                        let key = format!("{}=", k);
                        buffer.push_str(&key);
                        current_col = self.dfs(buffer, v, col + key.len(), 0)?;
                    } else {
                        buffer.push_str(&strings[i]);
                    }
                    if i < obj.len() - 1 {
                        buffer.push(self.delimiter);
                        indent(buffer, col);
                    }
                }
                Ok(current_col)
            } else {
                buffer.push_str(&candidate);
                Ok(col + candidate.len())
            }
        } else {
            Ok(col)
        }
    }
    
    fn dfs(&mut self, buffer: &mut String, input: &Value, col: usize, deep: usize) -> Result<usize, String> {
        match input {
            Value::Object(obj) => self.dict(buffer, obj, col, deep + 1),
            Value::Array(arr) => self.list(buffer, arr, col, deep + 1),
            _ => {
                let formatted = self.format_value(input);
                buffer.push_str(&formatted);
                Ok(col + formatted.len())
            }
        }
    }
    
    fn dict(&mut self, buffer: &mut String, inputs: &serde_json::Map<String, Value>, col: usize, deep: usize) -> Result<usize, String> {
        let strings: Vec<String> = inputs.iter()
            .map(|(k, v)| format!("{}: {}", k, self.format_value(v)))
            .collect();
        
        let candidate = strings.join(",");
        buffer.push('{');
        let mut current_col = col + 1;
        
        let options = OPTIONS.with(|opts| opts.borrow().clone());
        
        if current_col + candidate.len() > options.max_col && options.indent > 0 {
            for (i, (k, v)) in inputs.iter().enumerate() {
                current_col = col + options.indent;
                indent(buffer, current_col);
                current_col += strings[i].len() + 1;
                
                if current_col > options.max_col {
                    let key = format!("{}: ", k);
                    buffer.push_str(&key);
                    current_col = self.dfs(buffer, v, col + options.indent + key.len(), deep + 1)?;
                } else {
                    buffer.push_str(&strings[i]);
                }
                
                if i < inputs.len() - 1 {
                    buffer.push(',');
                }
            }
            indent(buffer, col - 1);
        } else {
            buffer.push_str(&candidate);
            current_col = col + candidate.len();
        }
        
        buffer.push('}');
        Ok(current_col + 1)
    }
    
    fn list(&mut self, buffer: &mut String, inputs: &[Value], col: usize, deep: usize) -> Result<usize, String> {
        let strings: Vec<String> = inputs.iter()
            .map(|v| self.format_value(v))
            .collect();
        
        let candidate = strings.join(",");
        buffer.push('[');
        let mut current_col = col + 1;
        
        let options = OPTIONS.with(|opts| opts.borrow().clone());
        
        if current_col + candidate.len() > options.max_col && options.indent > 0 {
            for (i, item) in inputs.iter().enumerate() {
                current_col += strings[i].len() + 1;
                if current_col > options.max_col {
                    indent(buffer, col);
                    current_col = self.dfs(buffer, item, col, deep + 1)?;
                } else {
                    buffer.push_str(&strings[i]);
                }
                if i < inputs.len() - 1 {
                    buffer.push(',');
                }
            }
        } else {
            buffer.push_str(&candidate);
            current_col = col + candidate.len();
        }
        
        buffer.push(']');
        Ok(current_col + 1)
    }
    
    fn format_value(&self, value: &Value) -> String {
        match value {
            Value::String(s) => format!("'{}'", s.replace('\'', "\\'")),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            _ => value.to_string(),
        }
    }
}

/// Decompile an operation definition
fn decompile_op(buffer: &mut String, op: &Value) -> Result<(), String> {
    if !op.is_object() {
        return Err("Operation must be a JSON object".to_string());
    }
    
    let options = OPTIONS.with(|opts| opts.borrow().clone());
    
    let default_meta = serde_json::Map::new();
    let metas = op.get("metas").and_then(|v| v.as_object()).unwrap_or(&default_meta);
    let mut copy_meta = metas.clone();
    
    // Remove as and version from meta
    let op_as = copy_meta.remove("as").and_then(|v| v.as_str().map(String::from));
    let op_version = copy_meta.remove("version").and_then(|v| v.as_str().map(String::from));
    
    buffer.push_str("op {");
    
    // Handle meta
    if !copy_meta.is_empty() {
        indent(buffer, options.indent);
        buffer.push_str("meta {");
        
        let meta_value = Value::Object(copy_meta);
        let mut param_formatter = ParamFormatter::new(&meta_value, ',');
        param_formatter.format(buffer, options.indent * 2)?;
        
        if options.indent > 0 {
            buffer.push('\n');
            for _ in 0..options.indent {
                buffer.push(' ');
            }
        }
        buffer.push_str("};");
    }
    
    // Handle inputs
    if let Some(inputs_obj) = op.get("inputs").and_then(|v| v.as_object()) {
        let inputs = inputs_obj.clone(); // Create owned copy
        indent(buffer, options.indent);
        buffer.push_str("input {");
        op_spec_format(&inputs, buffer, options.indent * 2)?;
        if options.indent > 0 {
            buffer.push('\n');
            for _ in 0..options.indent {
                buffer.push(' ');
            }
        }
        buffer.push_str("};");
    }
    
    // Handle outputs
    if let Some(outputs_obj) = op.get("outputs").and_then(|v| v.as_object()) {
        let outputs = outputs_obj.clone(); // Create owned copy
        indent(buffer, options.indent);
        buffer.push_str("output {");
        op_spec_format(&outputs, buffer, options.indent * 2)?;
        if options.indent > 0 {
            buffer.push('\n');
            for _ in 0..options.indent {
                buffer.push(' ');
            }
        }
        buffer.push_str("};");
    }
    
    // Handle configs
    if let Some(configs_obj) = op.get("configs").and_then(|v| v.as_object()) {
        let configs = configs_obj.clone(); // Create owned copy
        indent(buffer, options.indent);
        buffer.push_str("config {");
        op_spec_format(&configs, buffer, options.indent * 2)?;
        if options.indent > 0 {
            buffer.push('\n');
            for _ in 0..options.indent {
                buffer.push(' ');
            }
        }
        buffer.push_str("};");
    }
    
    // Handle graph
    if let Some(graph) = op.get("graph") {
        decompile_graph(buffer, graph)?;
    }
    
    if options.indent > 0 {
        buffer.push('\n');
    }
    buffer.push('}');
    
    // Handle alias and version
    if let Some(as_name) = op_as {
        let checked_as = check_id(&as_name)?;
        buffer.push_str(&format!(" as {}", checked_as));
        
        if let Some(version) = op_version {
            let checked_version = check_version(&version)?;
            buffer.push_str(&format!(".version('{}')", checked_version));
        }
    }
    
    buffer.push(';');
    Ok(())
}

/// Format operation specification
fn op_spec_format(inputs: &serde_json::Map<String, Value>, buffer: &mut String, col: usize) -> Result<(), String> {
    let options = OPTIONS.with(|opts| opts.borrow().clone());
    
    for (i, (name, spec)) in inputs.iter().enumerate() {
        buffer.push_str(name);
        buffer.push_str(":(");
        
        if let Some(spec_obj) = spec.as_object() {
            for (j, (k, v)) in spec_obj.iter().enumerate() {
                let value = match k.as_str() {
                    "dtype" => {
                        v.as_str().unwrap_or(&v.to_string()).to_string()
                    },
                    "length" | "range" => {
                        op_length_range_format(v)
                    },
                    "choice" => {
                        if let Some(choices) = v.as_array() {
                            let choices_str: Vec<String> = choices.iter()
                                .map(|c| format!("'{}'", c.as_str().unwrap_or(&c.to_string())))
                                .collect();
                            format!("({})", choices_str.join(","))
                        } else {
                            format!("'{}'", v.as_str().unwrap_or(&v.to_string()))
                        }
                    },
                    _ => {
                        format!("'{}'", v.as_str().unwrap_or(&v.to_string()))
                    }
                };
                
                buffer.push_str(&format!("{}={}", k, value));
                if j < spec_obj.len() - 1 {
                    buffer.push(',');
                }
            }
        }
        
        buffer.push_str(");");
        if i < inputs.len() - 1 && options.indent > 0 {
            indent(buffer, col);
        }
    }
    
    Ok(())
}

/// Format length/range specification
fn op_length_range_format(inputs: &Value) -> String {
    if let Some(eq) = inputs.get("eq").and_then(|v| v.as_i64()) {
        return eq.to_string();
    }
    
    let mut result = String::new();
    
    // Handle lower bound
    if let Some(ge) = inputs.get("ge").and_then(|v| v.as_i64()) {
        result.push_str(&format!("[{}", ge));
    } else if let Some(gt) = inputs.get("gt").and_then(|v| v.as_i64()) {
        result.push_str(&format!("({}", gt));
    } else {
        result.push('[');
    }
    
    result.push(',');
    
    // Handle upper bound
    if let Some(le) = inputs.get("le").and_then(|v| v.as_i64()) {
        result.push_str(&format!("{}]", le));
    } else if let Some(lt) = inputs.get("lt").and_then(|v| v.as_i64()) {
        result.push_str(&format!("{})", lt));
    } else {
        result.push(']');
    }
    
    result
}

/// Helper function to format input strings
fn input_str(inputs: &Value) -> String {
    match inputs {
        Value::Array(arr) => {
            if arr.len() == 1 {
                arr[0].to_string()
            } else {
                format!("({})", arr.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","))
            }
        },
        _ => inputs.to_string(),
    }
}

/// Check if identifier is valid
fn check_id(value: &str) -> Result<String, String> {
    let re = Regex::new(VALID_IDENTIFIER).unwrap();
    if re.is_match(value) {
        Ok(value.to_string())
    } else {
        Err(format!("Invalid identifier: {}", value))
    }
}

/// Check if version string is valid
fn check_version(value: &str) -> Result<String, String> {
    let re = Regex::new(VALID_VERSION).unwrap();
    if re.is_match(value) {
        Ok(value.to_string())
    } else {
        Ok(value.to_string()) // For now, allow any version format
    }
}

/// Add indentation to buffer
fn indent(buffer: &mut String, spaces: usize) {
    if spaces > 0 {
        buffer.push('\n');
        for _ in 0..spaces {
            buffer.push(' ');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_basic_decompile() {
        let data = json!({
            "graphs": [{
                "as": "main",
                "nodes": {
                    "node1": {
                        "output": ["node1"],
                        "op_name": "test.op",
                        "input": ["input1", "input2"]
                    }
                }
            }]
        });
        
        let result = decompile_from_data(data, None).unwrap();
        match result {
            DecompileResult::Text(text) => {
                assert!(text.contains("graph {"));
                assert!(text.contains("node1 = test.op(input1,input2);"));
                assert!(text.contains("} as main;"));
            },
            _ => panic!("Expected text result"),
        }
    }
    
    #[test]
    fn test_check_id() {
        assert!(check_id("valid_id").is_ok());
        assert!(check_id("valid-id").is_ok());
        assert!(check_id("valid$id").is_ok());
        assert!(check_id("123invalid").is_err());
    }
}