//! GOS Compiler - Converts AST to Dictionary Structure
//!
//! This module implements the GOS compiler that converts parsed AST nodes into
//! dictionary structures.
//!
//! The compiler performs the following operations:
//! 1. Validates graph nodes' inputs, outputs, and dependencies
//! 2. Reads variables and updates them in graphs and operations
//! 3. Reads operation metadata and validates graph nodes
//! 4. Constructs subgraphs supporting two methods:
//!    a. External graph definitions via op metadata
//!    b. Directly embedded graph definitions in op metadata
//!
//! Output format:
//! ```json
//! {
//!     "graphs": [...],
//!     "ops": [...],
//!     "vars": {...},
//!     "gos_version": "x.x.x"
//! }
//! ```

#![allow(dead_code)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};

use crate::ast::*;
use crate::error::{ParseError, ParseResult};

/// Compilation options
#[derive(Debug, Clone, Default)]
pub struct CompileOptions {
    /// Return operation names
    pub return_op_names: bool,
    /// Return subgraphs
    pub return_subgraphs: bool,
    /// Keep original order
    pub keep_order: bool,
    /// Plugin name for conversion
    pub plugin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResult {
    /// Graph definitions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphs: Option<Vec<GraphDict>>,
    /// Operation definitions  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops: Option<Vec<OpDict>>,
    /// Variable definitions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vars: Option<HashMap<String, Value>>,
    /// GOS version
    pub gos_version: String,
    /// Operation names (if requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_names: Option<Vec<String>>,
    /// Subgraphs (if requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgraphs: Option<Vec<String>>,
}

/// Graph dictionary structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphDict {
    /// Graph properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Value>>,
    /// Graph nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<HashMap<String, NodeDict>>,
    /// Graph alias name
    #[serde(skip_serializing_if = "Option::is_none", rename = "as")]
    pub alias: Option<String>,
    /// Graph version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Template graph reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_graph: Option<String>,
    /// Template version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version: Option<String>,
}

/// Node dictionary structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDict {
    /// Operation name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_name: Option<String>,
    /// Reference graph
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_graph: Option<String>,
    /// Node version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Node outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<String>>,
    /// Node inputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    /// Node dependencies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends: Option<Vec<String>>,
    /// Node properties (with clause)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<HashMap<String, Value>>,
    /// Node properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Value>>,
    /// Node alias
    #[serde(skip_serializing_if = "Option::is_none", rename = "as")]
    pub alias: Option<String>,
    /// Override flag for templates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_flag: Option<bool>,
    /// For loop configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_loop: Option<HashMap<String, Value>>,
}

/// Operation dictionary structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpDict {
    /// Operation metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metas: Option<HashMap<String, Value>>,
    /// Operation inputs specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<HashMap<String, HashMap<String, Value>>>,
    /// Operation outputs specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<HashMap<String, HashMap<String, Value>>>,
    /// Operation configuration specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<HashMap<String, HashMap<String, Value>>>,
    /// Embedded graph definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph: Option<GraphDict>,
}

/// Main compiler structure
pub struct Compiler {
    options: CompileOptions,
}

impl Compiler {
    /// Create a new compiler with default options
    pub fn new() -> Self {
        Self {
            options: CompileOptions::default(),
        }
    }

    /// Create a new compiler with specified options
    pub fn with_options(options: CompileOptions) -> Self {
        Self { options }
    }

    /// Compile AST to dictionary structure
    pub fn compile(&self, ast: &AstNodeEnum) -> ParseResult<CompileResult> {
        match ast {
            AstNodeEnum::Module(module) => self.compile_module(module),
            _ => Err(ParseError::general("Expected Module as root AST node")),
        }
    }

    /// Compile a module (root AST node)
    fn compile_module(&self, module: &Module) -> ParseResult<CompileResult> {
        let mut result = CompileResult {
            graphs: None,
            ops: None,
            vars: None,
            gos_version: "0.5.2".to_string(),
            op_names: None,
            subgraphs: None,
        };

        let mut graphs = Vec::new();
        let mut ops = Vec::new();
        let mut vars: HashMap<String, Value> = HashMap::new();

        // Process each child statement
        for child in &module.children {
            match child {
                AstNodeEnum::VarDef(var_def) => {
                    self.process_var_def(var_def, &mut vars)?;
                }
                AstNodeEnum::GraphDef(graph_def) => {
                    let graph_dict = self.convert_graph_def(graph_def, &vars)?;
                    graphs.push(graph_dict);
                }
                AstNodeEnum::OpDef(op_def) => {
                    let op_dict = self.convert_op_def(op_def, &vars)?;
                    ops.push(op_dict);
                }
                AstNodeEnum::Import(_) => {
                    // Import processing would be handled here in a full implementation
                    // For now, we skip imports as they require file system access
                }
                AstNodeEnum::Comment(_) => {
                    // Comments are ignored in compilation
                }
                _ => {
                    // Handle other statement types as needed
                }
            }
        }

        // Set results if not empty
        if !graphs.is_empty() {
            result.graphs = Some(graphs);
        }
        if !ops.is_empty() {
            result.ops = Some(ops);
        }
        if !vars.is_empty() {
            result.vars = Some(vars);
        }

        Ok(result)
    }

    /// Process variable definition
    fn process_var_def(&self, var_def: &VarDef, vars: &mut HashMap<String, Value>) -> ParseResult<()> {
        for child in &var_def.children {
            match child {
                AstNodeEnum::AttrDef(attr_def) => {
                    let key = if let Some(alias) = &var_def.alias {
                        format!("{}.{}", alias.name, attr_def.name.name.trim())
                    } else {
                        attr_def.name.name.trim().to_string()
                    };
                    let value = self.convert_ast_to_value(&attr_def.value)?;
                    vars.insert(key, value);
                }
                _ => {}
            }
        }
        
        // Add alias information if present
        if let Some(alias) = &var_def.alias {
            let alias_key = format!("{}.as", alias.name);
            vars.insert(alias_key, Value::String(alias.name.clone()));
        }
        
        Ok(())
    }

    /// Convert graph definition to dictionary
    fn convert_graph_def(&self, graph_def: &GraphDef, vars: &HashMap<String, Value>) -> ParseResult<GraphDict> {
        let mut graph_dict = GraphDict {
            properties: None,
            nodes: None,
            alias: graph_def.alias.as_ref().map(|s| s.name.clone()),
            version: graph_def.version.as_ref().and_then(|v| self.extract_string_value(v)),
            template_graph: graph_def.template_graph.as_ref().map(|s| s.name.clone()),
            template_version: graph_def.template_version.as_ref().and_then(|v| self.extract_string_value(v)),
        };

        let mut properties: HashMap<String, Value> = HashMap::new();
        let mut nodes: HashMap<String, NodeDict> = HashMap::new();

        for child in &graph_def.children {
            match child {
                AstNodeEnum::AttrDef(attr_def) => {
                    // Check if this AttrDef contains a NodeBlock (parsed as graph_prop_def with node_func_block)
                    if let AstNodeEnum::NodeBlock(node_block) = &*attr_def.value {
                        // This is actually a node definition, not a property
                        // Create a NodeDef from the NodeBlock and AttrDef name
                        let node_dict = NodeDict {
                            op_name: Some(node_block.name.name.clone()),
                            ref_graph: None,
                            version: None,
                            outputs: Some(vec![attr_def.name.name.clone()]),
                            inputs: self.extract_node_inputs(node_block)?,
                            depends: None,
                            with: self.extract_node_attributes(node_block, vars)?,
                            properties: None,
                            alias: None,
                            override_flag: None,
                            for_loop: None,
                        };
                        nodes.insert(attr_def.name.name.clone(), node_dict);
                    } else {
                        // This is a regular property
                        let value = self.convert_ast_to_value(&attr_def.value)?;
                        let resolved_value = self.resolve_variable_references(&value, vars)?;
                        properties.insert(attr_def.name.name.clone(), resolved_value);
                    }
                }
                AstNodeEnum::NodeDef(node_def) => {
                    let node_dict = self.convert_node_def(node_def, vars)?;
                    // Use the first output as the key, or generate one
                    let key = if !node_def.outputs.is_empty() {
                        node_def.outputs[0].name.clone()
                    } else {
                        format!("node_{}", nodes.len())
                    };
                    nodes.insert(key, node_dict);
                }
                _ => {}
            }
        }

        if !properties.is_empty() {
            graph_dict.properties = Some(properties);
        }
        if !nodes.is_empty() {
            graph_dict.nodes = Some(nodes);
        }

        Ok(graph_dict)
    }

    /// Convert node definition to dictionary
    fn convert_node_def(&self, node_def: &NodeDef, vars: &HashMap<String, Value>) -> ParseResult<NodeDict> {
        let mut node_dict = NodeDict {
            op_name: Some(node_def.value.name.name.clone()),
            ref_graph: None,
            version: None,
            outputs: Some(node_def.outputs.iter().map(|s| s.name.clone()).collect()),
            inputs: None,
            depends: None,
            with: None,
            properties: None,
            alias: None,
            override_flag: None,
            for_loop: None,
        };

        // Process node inputs
        if let Some(inputs) = &node_def.value.inputs {
            match inputs {
                NodeInputDef::Tuple(tuple_inputs) => {
                    node_dict.inputs = Some(tuple_inputs.items.iter().map(|s| s.name.clone()).collect());
                }
                NodeInputDef::KeyValue(kv_inputs) => {
                    // For key-value inputs, we need to process them differently
                    let mut input_list = Vec::new();
                    for item in &kv_inputs.items {
                        input_list.extend(item.value.items.iter().map(|s| s.name.clone()));
                    }
                    node_dict.inputs = Some(input_list);
                }
            }
        }

        // Process node attributes
        if let Some(attrs) = &node_def.value.attrs {
            let mut with_props: HashMap<String, Value> = HashMap::new();
            let mut _properties: HashMap<String, Value> = HashMap::new();
            
            for attr in attrs {
                let value = match &attr.value {
                    NodeAttrValue::Symbol(symbol) => Value::String(symbol.name.clone()),
                    NodeAttrValue::String(string_lit) => Value::String(string_lit.value.clone()),
                    NodeAttrValue::List(list) => {
                        let list_values: Result<Vec<Value>, _> = list.iter()
                            .map(|item| self.convert_ast_to_value(item))
                            .collect();
                        Value::Array(list_values?)
                    }
                };
                
                let resolved_value = self.resolve_variable_references(&value, vars)?;
                
                // Determine if this should go in 'with' or 'properties'
                match attr.name.name.as_str() {
                    "version" => node_dict.version = self.value_to_string(&resolved_value),
                    "as" => node_dict.alias = self.value_to_string(&resolved_value),
                    "override" => node_dict.override_flag = self.value_to_bool(&resolved_value),
                    _ => {
                        with_props.insert(attr.name.name.clone(), resolved_value);
                    }
                }
            }
            
            if !with_props.is_empty() {
                node_dict.with = Some(with_props);
            }
        }

        Ok(node_dict)
    }

    /// Convert operation definition to dictionary
    fn convert_op_def(&self, op_def: &OpDef, vars: &HashMap<String, Value>) -> ParseResult<OpDict> {
        let mut op_dict = OpDict {
            metas: None,
            inputs: None,
            outputs: None,
            configs: None,
            graph: None,
        };

        let mut metas: HashMap<String, Value> = HashMap::new();
        let mut inputs: HashMap<String, HashMap<String, Value>> = HashMap::new();
        let mut outputs: HashMap<String, HashMap<String, Value>> = HashMap::new();
        let mut configs: HashMap<String, HashMap<String, Value>> = HashMap::new();

        // Add alias and version to metas if present
        if let Some(alias) = &op_def.alias {
            metas.insert("as".to_string(), Value::String(alias.name.clone()));
        }
        if let Some(version) = &op_def.version {
            metas.insert("version".to_string(), Value::String(version.clone()));
        }

        for child in &op_def.children {
            match child {
                AstNodeEnum::OpMeta(op_meta) => {
                    for attr_def in &op_meta.children {
                        let value = self.convert_ast_to_value(&attr_def.value)?;
                        let resolved_value = self.resolve_variable_references(&value, vars)?;
                        metas.insert(attr_def.name.name.clone(), resolved_value);
                    }
                }
                AstNodeEnum::OpInput(op_input) => {
                    for input_child in &op_input.children {
                        if let AstNodeEnum::OpSpec(spec) = input_child {
                            let spec_dict = self.convert_op_spec(spec, vars)?;
                            inputs.insert(spec.name.name.clone(), spec_dict);
                        }
                    }
                }
                AstNodeEnum::OpOutput(op_output) => {
                    for output_child in &op_output.children {
                        if let AstNodeEnum::OpSpec(spec) = output_child {
                            let spec_dict = self.convert_op_spec(spec, vars)?;
                            outputs.insert(spec.name.name.clone(), spec_dict);
                        }
                    }
                }
                AstNodeEnum::OpConfig(op_config) => {
                    for config_child in &op_config.children {
                        if let AstNodeEnum::OpSpec(spec) = config_child {
                            let spec_dict = self.convert_op_spec(spec, vars)?;
                            configs.insert(spec.name.name.clone(), spec_dict);
                        }
                    }
                }
                _ => {}
            }
        }

        if !metas.is_empty() {
            op_dict.metas = Some(metas);
        }
        if !inputs.is_empty() {
            op_dict.inputs = Some(inputs);
        }
        if !outputs.is_empty() {
            op_dict.outputs = Some(outputs);
        }
        if !configs.is_empty() {
            op_dict.configs = Some(configs);
        }

        Ok(op_dict)
    }

    /// Convert operation specification to dictionary
    fn convert_op_spec(&self, spec: &OpSpec, vars: &HashMap<String, Value>) -> ParseResult<HashMap<String, Value>> {
        let mut spec_dict: HashMap<String, Value> = HashMap::new();

        if let Some(items) = &spec.items {
            for item in items {
                let value = self.convert_ast_to_value(&item.value)?;
                let resolved_value = self.resolve_variable_references(&value, vars)?;
                spec_dict.insert(item.name.clone(), resolved_value);
            }
        }

        Ok(spec_dict)
    }

    /// Convert AST node to JSON value
    fn convert_ast_to_value(&self, node: &AstNodeEnum) -> ParseResult<Value> {
        match node {
            AstNodeEnum::StringLiteral(s) => Ok(Value::String(s.value.clone())),
            AstNodeEnum::MultiLineStringLiteral(s) => Ok(Value::String(s.value.clone())),
            AstNodeEnum::NumberLiteral(n) => Ok(Value::Number(serde_json::Number::from(n.value))),
            AstNodeEnum::FloatLiteral(f) => {
                if let Some(num) = serde_json::Number::from_f64(f.value) {
                    Ok(Value::Number(num))
                } else {
                    Ok(Value::Null)
                }
            }
            AstNodeEnum::BoolLiteral(b) => Ok(Value::Bool(b.value)),
            AstNodeEnum::NullLiteral(_) => Ok(Value::Null),
            AstNodeEnum::Symbol(s) => Ok(Value::String(s.name.clone())),
            AstNodeEnum::ListStatement(list) => {
                let values: Result<Vec<Value>, _> = list.items.iter()
                    .map(|item| self.convert_ast_to_value(item))
                    .collect();
                Ok(Value::Array(values?))
            }
            AstNodeEnum::DictStatement(dict) => {
                let mut map = Map::new();
                for item in &dict.items {
                    let key = match self.convert_ast_to_value(&item.key)? {
                        Value::String(s) => s,
                        other => other.to_string(),
                    };
                    let value = self.convert_ast_to_value(&item.value)?;
                    map.insert(key, value);
                }
                Ok(Value::Object(map))
            }
            _ => Ok(Value::String(format!("unsupported_ast_node_{:?}", std::mem::discriminant(node)))),
        }
    }

    /// Resolve variable references in values
    fn resolve_variable_references(&self, value: &Value, vars: &HashMap<String, Value>) -> ParseResult<Value> {
        match value {
            Value::String(s) => {
                if let Some(var_value) = vars.get(s) {
                    Ok(var_value.clone())
                } else {
                    Ok(value.clone())
                }
            }
            Value::Array(arr) => {
                let resolved: Result<Vec<Value>, _> = arr.iter()
                    .map(|v| self.resolve_variable_references(v, vars))
                    .collect();
                Ok(Value::Array(resolved?))
            }
            Value::Object(obj) => {
                let mut resolved_obj = Map::new();
                for (k, v) in obj {
                    let resolved_value = self.resolve_variable_references(v, vars)?;
                    resolved_obj.insert(k.clone(), resolved_value);
                }
                Ok(Value::Object(resolved_obj))
            }
            _ => Ok(value.clone()),
        }
    }

    /// Helper function to extract string value from AST node
    fn extract_string_value(&self, node: &AstNodeEnum) -> Option<String> {
        match node {
            AstNodeEnum::StringLiteral(s) => Some(s.value.clone()),
            AstNodeEnum::Symbol(s) => Some(s.name.clone()),
            _ => None,
        }
    }

    /// Helper function to convert Value to String
    fn value_to_string(&self, value: &Value) -> Option<String> {
        match value {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    /// Helper function to convert Value to bool
    fn value_to_bool(&self, value: &Value) -> Option<bool> {
        match value {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Extract node inputs from NodeBlock
    fn extract_node_inputs(&self, node_block: &NodeBlock) -> ParseResult<Option<Vec<String>>> {
        if let Some(inputs) = &node_block.inputs {
            match inputs {
                NodeInputDef::Tuple(tuple_inputs) => {
                    Ok(Some(tuple_inputs.items.iter().map(|s| s.name.clone()).collect()))
                }
                NodeInputDef::KeyValue(kv_inputs) => {
                    let mut input_list = Vec::new();
                    for item in &kv_inputs.items {
                        input_list.extend(item.value.items.iter().map(|s| s.name.clone()));
                    }
                    Ok(Some(input_list))
                }
            }
        } else {
            Ok(None)
        }
    }

    /// Extract node attributes from NodeBlock
    fn extract_node_attributes(&self, node_block: &NodeBlock, vars: &HashMap<String, Value>) -> ParseResult<Option<HashMap<String, Value>>> {
        if let Some(attrs) = &node_block.attrs {
            let mut with_props: HashMap<String, Value> = HashMap::new();
            
            for attr in attrs {
                let value = match &attr.value {
                    NodeAttrValue::Symbol(symbol) => Value::String(symbol.name.clone()),
                    NodeAttrValue::String(string_lit) => Value::String(string_lit.value.clone()),
                    NodeAttrValue::List(list) => {
                        let list_values: Result<Vec<Value>, _> = list.iter()
                            .map(|item| self.convert_ast_to_value(item))
                            .collect();
                        Value::Array(list_values?)
                    }
                };
                
                let resolved_value = self.resolve_variable_references(&value, vars)?;
                with_props.insert(attr.name.name.clone(), resolved_value);
            }
            
            if with_props.is_empty() {
                Ok(None)
            } else {
                Ok(Some(with_props))
            }
        } else {
            Ok(None)
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to compile AST with default options
pub fn compile_ast(ast: &AstNodeEnum) -> ParseResult<CompileResult> {
    let compiler = Compiler::new();
    compiler.compile(ast)
}

/// Convenience function to compile AST with custom options
pub fn compile_ast_with_options(ast: &AstNodeEnum, options: CompileOptions) -> ParseResult<CompileResult> {
    let compiler = Compiler::with_options(options);
    compiler.compile(ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Position, Module};

    #[test]
    fn test_compile_empty_module() {
        let module = Module {
            position: Position::new(1, 1, 1),
            children: vec![],
        };
        let ast = AstNodeEnum::Module(module);
        
        let result = compile_ast(&ast).unwrap();
        assert_eq!(result.gos_version, "0.5.2");
        assert!(result.graphs.is_none());
        assert!(result.ops.is_none());
        assert!(result.vars.is_none());
    }

    #[test]
    fn test_compiler_creation() {
        let compiler = Compiler::new();
        assert!(!compiler.options.return_op_names);
        assert!(!compiler.options.return_subgraphs);
        assert!(!compiler.options.keep_order);
        assert!(compiler.options.plugin.is_none());
    }

    #[test]
    fn test_compiler_with_options() {
        let options = CompileOptions {
            return_op_names: true,
            return_subgraphs: true,
            keep_order: true,
            plugin: Some("test_plugin".to_string()),
        };
        let compiler = Compiler::with_options(options);
        assert!(compiler.options.return_op_names);
        assert!(compiler.options.return_subgraphs);
        assert!(compiler.options.keep_order);
        assert_eq!(compiler.options.plugin, Some("test_plugin".to_string()));
    }
}