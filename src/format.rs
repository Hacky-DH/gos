//! GOS Code Formatter
//! 
//! This module provides formatting functionality for GOS (Graph Representation Language) code.
//! It corresponds to the Python implementation in gos/format.py, maintaining the same
//! structure and formatting behavior.
#![allow(dead_code)]

use crate::ast::*;
use crate::parser::parse_gos;
use crate::ParseOptions;
use std::fs;
use std::path::Path;

/// GOS code formatting tool
/// 
/// # Arguments
/// * `content` - GOS content string
/// * `indent` - Indentation size (default: 4)
/// * `max_col` - Maximum column width (default: 100)
/// 
/// # Returns
/// Formatted GOS text string
pub fn format_from_data(content: &str, indent: usize, max_col: usize) -> Result<String, Box<dyn std::error::Error>> {
    let options = ParseOptions {
        ast: true,
        tracking: true,
        ..Default::default()
    };
    
    let parsed = parse_gos(content, options)?;
    let formatter = Formatter::new(indent, max_col);
    Ok(formatter.format(&parsed, 0))
}

/// GOS code formatting tool for files
/// 
/// # Arguments
/// * `filename` - Path to GOS file
/// * `indent` - Indentation size (default: 4)  
/// * `max_col` - Maximum column width (default: 100)
/// 
/// # Returns
/// Formatted GOS text string
pub fn format(filename: &str, indent: usize, max_col: usize) -> Result<String, Box<dyn std::error::Error>> {
    if filename.is_empty() {
        return Err("Filename cannot be empty".into());
    }
    
    let path = Path::new(filename);
    if !path.exists() {
        return Err(format!("File {} not found", filename).into());
    }
    
    let content = fs::read_to_string(path)?;
    format_from_data(&content, indent, max_col)
}

/// Indent buffer for managing indented output
/// 
/// This corresponds to the Python IndentBuffer class, providing
/// methods for writing indented content with proper formatting.
#[derive(Debug)]
pub struct IndentBuffer {
    buffer: String,
    indent_size: usize,
    current_indent: usize,
}

impl IndentBuffer {
    /// Create a new IndentBuffer
    /// 
    /// # Arguments
    /// * `indent_size` - Size of each indentation level
    /// * `begin_indent` - Initial indentation level
    pub fn new(indent_size: usize, begin_indent: usize) -> Self {
        Self {
            buffer: String::new(),
            indent_size,
            current_indent: begin_indent,
        }
    }

    /// Write multiple arguments as strings
    pub fn writes(&mut self, args: &[&str]) -> usize {
        let mut len = 0;
        for arg in args {
            len += self.write(arg);
        }
        len
    }

    /// Write a single string
    pub fn write(&mut self, s: &str) -> usize {
        self.buffer.push_str(s);
        s.len()
    }

    /// Write with indentation first
    pub fn write_indent(&mut self, args: &[&str]) -> usize {
        let mut len = 0;
        if self.indent_size > 0 && self.current_indent > 0 {
            let indent_str = " ".repeat(self.current_indent);
            len += self.write(&indent_str);
        }
        len += self.writes(args);
        len
    }

    /// Write with newline
    pub fn writeln(&mut self, args: &[&str]) -> usize {
        let len = self.writes(args);
        if self.indent_size > 0 {
            self.buffer.push('\n');
            len + 1
        } else {
            len
        }
    }

    /// Write with indentation and newline
    pub fn writeln_indent(&mut self, args: &[&str]) -> usize {
        let len = self.write_indent(args);
        if self.indent_size > 0 {
            self.buffer.push('\n');
            len + 1
        } else {
            len
        }
    }

    /// Increase indentation level
    pub fn indent(&mut self) -> usize {
        self.current_indent += self.indent_size;
        self.current_indent
    }

    /// Decrease indentation level
    pub fn dedent(&mut self) -> usize {
        if self.current_indent >= self.indent_size {
            self.current_indent -= self.indent_size;
        }
        self.current_indent
    }

    /// Get the buffer content
    pub fn get_value(&self) -> &str {
        &self.buffer
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

/// Main formatter struct
/// 
/// This corresponds to the Python Format class, providing
/// comprehensive formatting functionality for all AST node types.
#[derive(Debug)]
pub struct Formatter {
    indent: usize,
    max_col: usize,
    cur_col: usize,
}

impl Formatter {
    /// Create a new formatter
    pub fn new(indent: usize, max_col: usize) -> Self {
        Self {
            indent,
            max_col,
            cur_col: 0,
        }
    }

    /// Format an AST node
    pub fn format(&self, ast: &AstNodeEnum, begin_indent: usize) -> String {
        let mut formatter = Self::new(self.indent, self.max_col);
        formatter.format_node(ast, begin_indent)
    }

    /// Format a specific AST node type
    fn format_node(&mut self, ast: &AstNodeEnum, begin_indent: usize) -> String {
        match ast {
            AstNodeEnum::Module(node) => self.format_module(node, begin_indent),
            AstNodeEnum::Comment(node) => self.format_comment(node, begin_indent),
            AstNodeEnum::Symbol(node) => node.name.clone(),
            AstNodeEnum::StringLiteral(node) => node.value.clone(),
            AstNodeEnum::MultiLineStringLiteral(node) => node.value.clone(),
            AstNodeEnum::NumberLiteral(node) => node.raw.clone(),
            AstNodeEnum::FloatLiteral(node) => node.raw.clone(),
            AstNodeEnum::BoolLiteral(node) => node.raw.clone(),
            AstNodeEnum::DateTimeLiteral(node) => node.raw.clone(),
            AstNodeEnum::DateLiteral(node) => node.value.clone(),
            AstNodeEnum::NullLiteral(_) => "null".to_string(),
            AstNodeEnum::Import(node) => self.format_import(node, begin_indent),
            AstNodeEnum::AttrDef(node) => self.format_attr_def(node, begin_indent),
            AstNodeEnum::RefDef(node) => self.format_ref_def(node, begin_indent),
            AstNodeEnum::VarDef(node) => self.format_var_def(node, begin_indent),
            AstNodeEnum::GraphDef(node) => self.format_graph_def(node, begin_indent),
            AstNodeEnum::NodeDef(node) => self.format_node_def(node, begin_indent),
            AstNodeEnum::OpDef(node) => self.format_op_def(node, begin_indent),
            AstNodeEnum::OpMeta(node) => self.format_op_meta(node, begin_indent),
            AstNodeEnum::OpInput(node) => self.format_op_input(node, begin_indent),
            AstNodeEnum::OpOutput(node) => self.format_op_output(node, begin_indent),
            AstNodeEnum::OpConfig(node) => self.format_op_config(node, begin_indent),
            AstNodeEnum::OpSpec(node) => self.format_op_spec(node, begin_indent),
            AstNodeEnum::DictStatement(node) => self.format_dict_statement(node, begin_indent),
            AstNodeEnum::ListStatement(node) => self.format_list_statement(node, begin_indent),
            AstNodeEnum::TupleStatement(node) => self.format_tuple_statement(node, begin_indent),
            AstNodeEnum::SetStatement(node) => self.format_set_statement(node, begin_indent),
            AstNodeEnum::ClosedInterval(node) => self.format_closed_interval(node, begin_indent),
            AstNodeEnum::MixInterval(node) => self.format_mix_interval(node, begin_indent),
            AstNodeEnum::NodeBlock(node) => self.format_node_block(node, begin_indent),
            AstNodeEnum::ConditionBlock(node) => self.format_condition_block(node, begin_indent),
            AstNodeEnum::ConditionStatement(node) => self.format_condition_statement(node, begin_indent),
            _ => String::new(), // Handle other node types as needed
        }
    }

    /// Format module node
    fn format_module(&mut self, module: &Module, begin_indent: usize) -> String {
        self.format_list_with_comment(&module.children, begin_indent)
    }

    /// Format comment node
    fn format_comment(&mut self, comment: &Comment, begin_indent: usize) -> String {
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        buffer.write_indent(&[&comment.value, "\n"]);
        self.cur_col = 0;
        buffer.get_value().to_string()
    }

    /// Format import statement
    fn format_import(&mut self, import: &Import, begin_indent: usize) -> String {
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        buffer.write_indent(&["import "]);
        
        for (index, item) in import.items.iter().enumerate() {
            buffer.write(&item.path.name);
            if let Some(alias) = &item.alias {
                buffer.write(&format!(" as {}", alias.name));
            }
            if index + 1 < import.items.len() {
                buffer.write(", ");
            }
        }
        buffer.write(";");
        buffer.get_value().to_string()
    }

    /// Format attribute definition
    fn format_attr_def(&mut self, attr: &AttrDef, begin_indent: usize) -> String {
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        self.cur_col += buffer.write_indent(&[&attr.name.name, " = "]);
        let value_str = self.format_value(&attr.value, begin_indent);
        buffer.write(&format!("{};", value_str));
        self.cur_col += 1;
        buffer.get_value().to_string()
    }

    /// Format reference definition
    fn format_ref_def(&mut self, ref_def: &RefDef, begin_indent: usize) -> String {
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        self.cur_col += buffer.write_indent(&[&ref_def.name.name, " = ", &ref_def.value.name, ";"]);
        buffer.get_value().to_string()
    }

    /// Format variable definition
    fn format_var_def(&mut self, var: &VarDef, begin_indent: usize) -> String {
        let body = self.format_brace("var", &var.children, begin_indent, var.position.line == 1);
        let result = if let Some(alias) = &var.alias {
            format!("{} as {};", body, alias.name)
        } else {
            format!("{};", body)
        };
        self.cur_col = result.len();
        result
    }

    /// Format graph definition  
    fn format_graph_def(&mut self, graph: &GraphDef, begin_indent: usize) -> String {
        let body = self.format_brace("graph", &graph.children, begin_indent, graph.position.line == 1);
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        self.cur_col += buffer.write(&body);
        
        if let Some(alias) = &graph.alias {
            self.cur_col += buffer.writes(&[" as ", &alias.name]);
            if let Some(version) = &graph.version {
                let version_str = self.format_value(version, begin_indent);
                self.cur_col += buffer.writes(&[".version(", &version_str, ")"]);
            }
        }
        self.cur_col += buffer.write(";");
        buffer.get_value().to_string()
    }

    /// Format node definition
    fn format_node_def(&mut self, node: &NodeDef, begin_indent: usize) -> String {
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        
        for (index, output) in node.outputs.iter().enumerate() {
            if index == 0 {
                buffer.write_indent(&[&output.name]);
            } else {
                buffer.write(&output.name);
            }
            if index + 1 < node.outputs.len() {
                buffer.write(", ");
            }
        }
        buffer.write(" = ");
        
        let value_str = self.format_node_block(&node.value, begin_indent);
        buffer.write(&format!("{};", value_str));
        buffer.get_value().to_string()
    }

    /// Format node block
    fn format_node_block(&mut self, node: &NodeBlock, begin_indent: usize) -> String {
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        
        // Check if this is a reference or direct node call
        if node.name_or_ref.kind == SymbolKind::NodeName {
            buffer.writes(&[&node.name_or_ref.name, "("]);
            if let Some(inputs) = &node.inputs {
                buffer.write(&self.format_node_inputs(inputs));
            }
            buffer.write(")");
        } else {
            buffer.writes(&["ref(", &node.name_or_ref.name, "("]);
            if let Some(inputs) = &node.inputs {
                buffer.write(&self.format_node_inputs(inputs));
            }
            buffer.write("))");
        }
        
        // Format attributes
        if let Some(attrs) = &node.attrs {
            for attr in attrs {
                buffer.writes(&[".", &attr.name.name, "("]);
                let attr_value = self.format_node_attr_value(&attr.value, begin_indent);
                buffer.writes(&[&attr_value, ")"]);
            }
        }
        
        buffer.get_value().to_string()
    }

    /// Format node inputs
    fn format_node_inputs(&mut self, inputs: &NodeInputDef) -> String {
        let mut buffer = IndentBuffer::new(0, 0);
        
        match inputs {
            NodeInputDef::Tuple(tuple) => {
                for (index, item) in tuple.items.iter().enumerate() {
                    buffer.write(&item.name);
                    if index + 1 < tuple.items.len() {
                        buffer.write(", ");
                    }
                }
            }
            NodeInputDef::KeyValue(key_def) => {
                for (index, item) in key_def.items.iter().enumerate() {
                    if item.value.items.len() == 1 {
                        buffer.writes(&[&item.key.name, "=", &item.value.items[0].name]);
                    } else {
                        buffer.writes(&[&item.key.name, "=("]);
                        for (idx, val) in item.value.items.iter().enumerate() {
                            buffer.write(&val.name);
                            if idx + 1 < item.value.items.len() {
                                buffer.write(", ");
                            }
                        }
                        buffer.write(")");
                    }
                    if index + 1 < key_def.items.len() {
                        buffer.write(", ");
                    }
                }
            }
        }
        
        buffer.get_value().to_string()
    }

    /// Format node attribute value
    fn format_node_attr_value(&mut self, value: &NodeAttrValue, begin_indent: usize) -> String {
        match value {
            NodeAttrValue::Symbol(sym) => sym.name.clone(),
            NodeAttrValue::String(str_lit) => str_lit.value.clone(),
            NodeAttrValue::List(items) => {
                let mut buffer = IndentBuffer::new(0, 0);
                buffer.write("[");
                for (index, item) in items.iter().enumerate() {
                    buffer.write(&self.format_value(item, begin_indent));
                    if index + 1 < items.len() {
                        buffer.write(", ");
                    }
                }
                buffer.write("]");
                buffer.get_value().to_string()
            }
        }
    }

    /// Format condition block
    fn format_condition_block(&mut self, cond: &ConditionBlock, begin_indent: usize) -> String {
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        
        match &*cond.condition {
            ConditionExpr::Statement(stmt) => {
                let left = self.format_value(&stmt.left_operand, begin_indent);
                let right = self.format_value(&stmt.right_operand, begin_indent);
                buffer.writes(&[&left, " ", &stmt.operator, " ", &right]);
            }
            ConditionExpr::Block(block) => {
                buffer.write(&self.format_node_block(block, begin_indent));
            }
        }
        
        buffer.write(" ? ");
        
        match cond.true_branch.as_ref() {
            AstNodeEnum::ConditionBlock(cb) => {
                buffer.write(&self.format_condition_block(cb, begin_indent));
            }
            AstNodeEnum::NodeBlock(nb) => {
                buffer.write(&self.format_node_block(nb, begin_indent));
            }
            _ => {
                buffer.write(&self.format_value(cond.true_branch.as_ref(), begin_indent));
            }
        }
        
        buffer.write(" : ");
        
        match cond.false_branch.as_ref() {
            AstNodeEnum::ConditionBlock(cb) => {
                buffer.write(&self.format_condition_block(cb, begin_indent));
            }
            AstNodeEnum::NodeBlock(nb) => {
                buffer.write(&self.format_node_block(nb, begin_indent));
            }
            _ => {
                buffer.write(&self.format_value(cond.false_branch.as_ref(), begin_indent));
            }
        }
        
        buffer.get_value().to_string()
    }

    /// Format condition statement
    fn format_condition_statement(&mut self, stmt: &ConditionStatement, begin_indent: usize) -> String {
        let left = self.format_value(&stmt.left_operand, begin_indent);
        let right = self.format_value(&stmt.right_operand, begin_indent);
        format!("{} {} {}", left, stmt.operator, right)
    }

    /// Format operation definition
    fn format_op_def(&mut self, op: &OpDef, begin_indent: usize) -> String {
        self.format_brace_as_version(op, "op", begin_indent)
    }

    /// Format operation meta section
    fn format_op_meta(&mut self, meta: &OpMeta, begin_indent: usize) -> String {
        // Convert AttrDef vector to AstNodeEnum vector
        let children: Vec<AstNodeEnum> = meta.children.iter()
            .map(|attr| AstNodeEnum::AttrDef(attr.clone()))
            .collect();
        self.format_brace_end("meta", &children, begin_indent, true)
    }

    /// Format operation input section
    fn format_op_input(&mut self, input: &OpInput, begin_indent: usize) -> String {
        self.format_brace_end("input", &input.children, begin_indent, true)
    }

    /// Format operation output section
    fn format_op_output(&mut self, output: &OpOutput, begin_indent: usize) -> String {
        self.format_brace_end("output", &output.children, begin_indent, true)
    }

    /// Format operation config section
    fn format_op_config(&mut self, config: &OpConfig, begin_indent: usize) -> String {
        self.format_brace_end("config", &config.children, begin_indent, true)
    }

    /// Format operation spec
    fn format_op_spec(&mut self, spec: &OpSpec, begin_indent: usize) -> String {
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        buffer.write_indent(&[&spec.name.name, ": "]);
        
        if let Some(items) = &spec.items {
            if items.len() == 1 {
                let value_str = self.format_value(&items[0].value, begin_indent);
                buffer.write(&format!("{};", value_str));
            } else {
                buffer.write("(");
                for (index, item) in items.iter().enumerate() {
                    let value_str = self.format_value(&item.value, begin_indent);
                    buffer.write(&format!("{}={}", item.name, value_str));
                    if index + 1 < items.len() {
                        buffer.write(", ");
                    }
                }
                buffer.write(");");
            }
        }
        buffer.get_value().to_string()
    }

    /// Format dictionary statement
    fn format_dict_statement(&mut self, dict: &DictStatement, begin_indent: usize) -> String {
        self.format_sequence("{", "}", &dict.items, begin_indent, true)
    }

    /// Format list statement
    fn format_list_statement(&mut self, list: &ListStatement, begin_indent: usize) -> String {
        self.format_sequence("[", "]", &list.items, begin_indent, false)
    }

    /// Format tuple statement
    fn format_tuple_statement(&mut self, tuple: &TupleStatement, begin_indent: usize) -> String {
        self.format_sequence("(", ")", &tuple.items, begin_indent, false)
    }

    /// Format set statement
    fn format_set_statement(&mut self, set: &SetStatement, begin_indent: usize) -> String {
        self.format_sequence("{", "}", &set.items, begin_indent, false)
    }

    /// Format closed interval
    fn format_closed_interval(&mut self, interval: &ClosedInterval, _begin_indent: usize) -> String {
        let mut parts = Vec::new();
        
        if let Some(ge) = &interval.ge {
            parts.push(format!("[{}", ge.raw));
        }
        if let Some(le) = &interval.le {
            parts.push(format!("{}]", le.raw));
        }
        
        parts.join(", ")
    }

    /// Format mixed interval
    fn format_mix_interval(&mut self, interval: &MixInterval, _begin_indent: usize) -> String {
        let left = if let Some(ge) = &interval.ge {
            format!("[{}", ge.raw)
        } else if let Some(gt) = &interval.gt {
            format!("({}", gt.raw)
        } else {
            "(".to_string()
        };
        
        let right = if let Some(le) = &interval.le {
            format!("{}]", le.raw)
        } else if let Some(lt) = &interval.lt {
            format!("{})", lt.raw)
        } else {
            ")".to_string()
        };
        
        format!("{}, {}", left, right)
    }

    /// Helper method to format values
    fn format_value(&mut self, ast: &AstNodeEnum, begin_indent: usize) -> String {
        match ast {
            AstNodeEnum::NumberLiteral(n) => n.raw.clone(),
            AstNodeEnum::FloatLiteral(n) => n.raw.clone(),
            AstNodeEnum::BoolLiteral(n) => n.raw.clone(),
            AstNodeEnum::DateLiteral(n) => n.value.clone(),
            AstNodeEnum::StringLiteral(n) => n.value.clone(),
            AstNodeEnum::MultiLineStringLiteral(n) => n.value.clone(),
            AstNodeEnum::Symbol(n) => n.name.clone(),
            AstNodeEnum::NullLiteral(_) => "null".to_string(),
            AstNodeEnum::DictStatement(n) => self.format_dict_statement(n, begin_indent),
            AstNodeEnum::ListStatement(n) => self.format_list_statement(n, begin_indent),
            AstNodeEnum::TupleStatement(n) => self.format_tuple_statement(n, begin_indent),
            AstNodeEnum::SetStatement(n) => self.format_set_statement(n, begin_indent),
            _ => self.format_node(ast, begin_indent),
        }
    }

    /// Format sequences with delimiters
    fn format_sequence(&mut self, start: &str, end: &str, items: &[impl FormatItem], begin_indent: usize, is_dict: bool) -> String {
        if items.is_empty() {
            self.cur_col += 2;
            return format!("{}{}", start, end);
        }
        
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        let new_line = self.need_line_for_items(items);
        
        if new_line {
            buffer.writeln(&[start]);
            self.cur_col = 0;
        } else {
            self.cur_col += buffer.write(start);
        }
        
        buffer.indent();
        let mut next_new_line = new_line;
        
        for (index, item) in items.iter().enumerate() {
            let item_str = if is_dict {
                item.format_as_dict_item(self, begin_indent)
            } else {
                item.format_as_item(self, begin_indent)
            };
            
            if next_new_line {
                self.cur_col += buffer.write_indent(&[&item_str]);
            } else {
                self.cur_col += buffer.write(&item_str);
            }
            
            if index + 1 < items.len() {
                next_new_line = new_line;
                if next_new_line {
                    buffer.writeln(&[","]);
                    self.cur_col = 0;
                } else {
                    self.cur_col += buffer.write(", ");
                }
            }
        }
        
        buffer.dedent();
        if new_line {
            buffer.writeln(&[""]);
            self.cur_col += buffer.write_indent(&[end]);
        } else {
            self.cur_col += buffer.write(end);
        }
        
        buffer.get_value().to_string()
    }

    /// Format brace-enclosed sections
    fn format_brace(&mut self, name: &str, children: &[AstNodeEnum], begin_indent: usize, is_first_line: bool) -> String {
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        
        if !is_first_line {
            buffer.writeln(&[""]);
        }
        buffer.writeln_indent(&[name, " {"]);
        self.cur_col = 0;
        
        if !children.is_empty() {
            buffer.indent();
            let body = self.format_list_with_comment(children, buffer.current_indent);
            if body.ends_with('\n') {
                buffer.write(&body);
            } else {
                buffer.writeln(&[&body]);
            }
            self.cur_col = 0;
            buffer.dedent();
        }
        
        self.cur_col += buffer.write_indent(&["}"]);
        buffer.get_value().to_string()
    }

    /// Format brace sections with version support
    fn format_brace_as_version(&mut self, node: &OpDef, name: &str, begin_indent: usize) -> String {
        let body = self.format_brace(name, &node.children, begin_indent, node.position.line == 1);
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        self.cur_col += buffer.write(&body);
        
        if let Some(alias) = &node.alias {
            self.cur_col += buffer.writes(&[" as ", &alias.name]);
            if let Some(version) = &node.version {
                self.cur_col += buffer.writes(&[".version(", version, ")"]);
            }
        }
        self.cur_col += buffer.write(";");
        buffer.get_value().to_string()
    }

    /// Format brace sections with semicolon
    fn format_brace_end(&mut self, name: &str, children: &[AstNodeEnum], begin_indent: usize, is_first_line: bool) -> String {
        let body = self.format_brace(name, children, begin_indent, is_first_line);
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        self.cur_col += buffer.writes(&[&body, ";"]);
        buffer.get_value().to_string()
    }

    /// Format list with comments
    fn format_list_with_comment(&mut self, children: &[AstNodeEnum], begin_indent: usize) -> String {
        let mut buffer = IndentBuffer::new(self.indent, begin_indent);
        let mut next_comment = false;
        
        for (index, child) in children.iter().enumerate() {
            if next_comment {
                next_comment = false;
                continue;
            }
            
            let cur_end = child.position().end_line;
            let child_str = self.format_node(child, begin_indent);
            buffer.write(&child_str);
            
            // Check for inline comment
            if let Some(comment) = self.get_inline_comment(index, cur_end, children) {
                buffer.writes(&[" ", &comment, "\n"]);
                self.cur_col = 0;
                next_comment = true;
                continue;
            }
            
            if index + 1 < children.len() && !matches!(child, AstNodeEnum::Comment(_)) {
                buffer.writeln(&[""]);
                self.cur_col = 0;
            }
        }
        
        buffer.get_value().to_string()
    }

    /// Check if inline comment exists
    fn get_inline_comment(&self, index: usize, cur_end: usize, children: &[AstNodeEnum]) -> Option<String> {
        if index + 1 < children.len() {
            if let AstNodeEnum::Comment(comment) = &children[index + 1] {
                if comment.position.line == cur_end {
                    return Some(comment.value.clone());
                }
            }
        }
        None
    }

    /// Calculate value length for line breaking decisions
    fn value_length(&self, ast: &AstNodeEnum) -> usize {
        match ast {
            AstNodeEnum::NumberLiteral(n) => n.raw.len(),
            AstNodeEnum::FloatLiteral(n) => n.raw.len(),
            AstNodeEnum::BoolLiteral(n) => n.raw.len(),
            AstNodeEnum::StringLiteral(n) => n.value.len(),
            AstNodeEnum::MultiLineStringLiteral(n) => n.value.len(),
            AstNodeEnum::Symbol(n) => n.name.len(),
            AstNodeEnum::NullLiteral(_) => 4,
            AstNodeEnum::DateLiteral(n) => n.value.len(),
            _ => 0, // Simplified - would need full implementation
        }
    }

    /// Check if line break is needed
    fn need_line(&self, ast: &AstNodeEnum) -> bool {
        (self.cur_col + self.value_length(ast) > self.max_col) && self.indent > 0
    }

    /// Check if line break is needed for items
    fn need_line_for_items<T>(&self, items: &[T]) -> bool {
        // Simplified logic - would need proper implementation
        items.len() > 3
    }
}

/// Trait for formatting different item types
trait FormatItem {
    fn format_as_item(&self, formatter: &mut Formatter, begin_indent: usize) -> String;
    fn format_as_dict_item(&self, formatter: &mut Formatter, begin_indent: usize) -> String;
}

impl FormatItem for AstNodeEnum {
    fn format_as_item(&self, formatter: &mut Formatter, begin_indent: usize) -> String {
        formatter.format_value(self, begin_indent)
    }

    fn format_as_dict_item(&self, formatter: &mut Formatter, begin_indent: usize) -> String {
        formatter.format_value(self, begin_indent)
    }
}

impl FormatItem for DictItem {
    fn format_as_item(&self, formatter: &mut Formatter, begin_indent: usize) -> String {
        let key = formatter.format_value(&self.key, begin_indent);
        let value = formatter.format_value(&self.value, begin_indent);
        format!("{}: {}", key, value)
    }

    fn format_as_dict_item(&self, formatter: &mut Formatter, begin_indent: usize) -> String {
        self.format_as_item(formatter, begin_indent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent_buffer() {
        let mut buffer = IndentBuffer::new(4, 0);
        buffer.write_indent(&["test"]);
        assert_eq!(buffer.get_value(), "test");
        
        buffer.indent();
        buffer.clear();
        buffer.write_indent(&["indented"]);
        assert_eq!(buffer.get_value(), "    indented");
    }

    #[test]
    fn test_format_from_data() {
        let content = r#"var { name = "test"; };"#;
        let result = format_from_data(content, 4, 100);
        assert!(result.is_ok());
    }
}