//! Tests for the GOS decompiler module

use crate::decompiler::{decompile_from_data, decompile, DecompileOptions, DecompileResult};
use serde_json::json;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_basic_graph_decompile() {
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
fn test_graph_with_template() {
    let data = json!({
        "graphs": [{
            "template_graph": "base_graph",
            "template_version": "1.0.0",
            "as": "main",
            "nodes": {
                "node1": {
                    "output": ["node1"],
                    "op_name": "test.op"
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("graph : base_graph.version('1.0.0') {"));
            assert!(text.contains("} as main;"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_node_with_version_and_alias() {
    let data = json!({
        "graphs": [{
            "nodes": {
                "my_node": {
                    "output": ["my_node"],
                    "op_name": "test.op",
                    "version": "2.0.0",
                    "input": ["data"]
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("my_node = test.op(data).version('2.0.0');"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_node_with_dependencies() {
    let data = json!({
        "graphs": [{
            "nodes": {
                "node1": {
                    "output": ["node1"],
                    "op_name": "test.op"
                },
                "node2": {
                    "output": ["node2"],
                    "op_name": "test.op2",
                    "depend": ["node1"]
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("node2 = test.op2().depend(node1);"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_condition_node() {
    let data = json!({
        "graphs": [{
            "nodes": {
                "result": {
                    "output": ["result"],
                    "op_name": "builtin.conditions.str",
                    "condition": "x > 0",
                    "true_branch": {
                        "op_name": "math.add",
                        "input": ["x", "1"]
                    },
                    "false_branch": {
                        "op_name": "math.sub",
                        "input": ["x", "1"]
                    }
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("result = x > 0 ? math.add(x,1) : math.sub(x,1);"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_for_loop_node() {
    let data = json!({
        "graphs": [{
            "nodes": {
                "result": {
                    "output": ["result"],
                    "op_name": "test.op",
                    "for_loop": {
                        "inputs": "items",
                        "outputs": ["item"],
                        "condition": "item.valid"
                    }
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("result = [test.op()"));
            assert!(text.contains("for item in items"));
            assert!(text.contains("if item.valid"));
            assert!(text.contains("];"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_operation_decompile() {
    let data = json!({
        "ops": [{
            "as": "my_op",
            "version": "1.0.0",
            "metas": {
                "description": "Test operation",
                "author": "test"
            },
            "inputs": {
                "input1": {
                    "dtype": "string",
                    "length": {"ge": 1, "le": 100}
                }
            },
            "outputs": {
                "output1": {
                    "dtype": "int"
                }
            },
            "configs": {
                "param1": {
                    "dtype": "bool",
                    "default": true
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("op {"));
            assert!(text.contains("meta {"));
            assert!(text.contains("description='Test operation'"));
            assert!(text.contains("input {"));
            assert!(text.contains("input1:(dtype=string,length=[1,100]);"));
            assert!(text.contains("output {"));
            assert!(text.contains("output1:(dtype=int);"));
            assert!(text.contains("config {"));
            assert!(text.contains("param1:(dtype=bool,default='true');"));
            assert!(text.contains("} as my_op.version('1.0.0');"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_string_escaping() {
    let data = json!({
        "graphs": [{
            "nodes": {
                "node1": {
                    "output": ["node1"],
                    "op_name": "test.op",
                    "input": ["input with spaces", "input'with'quotes"]
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("node1 = test.op('input with spaces','input\\'with\\'quotes');"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_unescape_option() {
    let data = json!({
        "graphs": [{
            "nodes": {
                "node1": {
                    "output": ["node1"],
                    "op_name": "test.op",
                    "input": ["line1\\nline2\\ttab"]
                }
            }
        }]
    });
    
    let options = DecompileOptions {
        unescape: true,
        ..Default::default()
    };
    
    let result = decompile_from_data(data, Some(options)).unwrap();
    match result {
        DecompileResult::Text(text) => {
            // After unescaping, the \n and \t should be actual newlines and tabs
            assert!(text.contains("line1"));
            assert!(text.contains("line2"));
            assert!(text.contains("\t")); // Actual tab character
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_invalid_identifier() {
    let data = json!({
        "graphs": [{
            "as": "123invalid",
            "nodes": {
                "valid_node": {
                    "output": ["valid_node"],
                    "op_name": "test.op"
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid identifier"));
}


#[test]
fn test_decompile_from_file() {
    // Create a temporary JSON file
    let temp_file = NamedTempFile::new().unwrap();
    let data = json!({
        "graphs": [{
            "as": "file_test",
            "nodes": {
                "node1": {
                    "output": ["node1"],
                    "op_name": "file.op"
                }
            }
        }]
    });
    
    fs::write(temp_file.path(), serde_json::to_string_pretty(&data).unwrap()).unwrap();
    
    let result = decompile(temp_file.path().to_str().unwrap(), None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("} as file_test;"));
            assert!(text.contains("node1 = file.op();"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_decompile_nonexistent_file() {
    let result = decompile("nonexistent_file.json", None);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File nonexistent_file.json not found"));
}

#[test]
fn test_decompile_invalid_json_file() {
    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), "invalid json content").unwrap();
    
    let result = decompile(temp_file.path().to_str().unwrap(), None);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("is not valid JSON"));
}

#[test]
fn test_multiple_graphs() {
    let data = json!({
        "graphs": [
            {
                "as": "graph1",
                "nodes": {
                    "node1": {
                        "output": ["node1"],
                        "op_name": "op1"
                    }
                }
            },
            {
                "as": "graph2",
                "nodes": {
                    "node2": {
                        "output": ["node2"],
                        "op_name": "op2"
                    }
                }
            }
        ]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("} as graph1;"));
            assert!(text.contains("} as graph2;"));
            // Should have double newline between graphs
            assert!(text.contains("};\n\ngraph"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_graph_with_properties() {
    let data = json!({
        "graphs": [{
            "property": {
                "description": "Test graph",
                "version": "1.0"
            },
            "nodes": {
                "node1": {
                    "output": ["node1"],
                    "op_name": "test.op"
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("graph {"));
            assert!(text.contains("description='Test graph',version='1.0';"));
            assert!(text.contains("node1 = test.op();"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_node_with_ref_graph() {
    let data = json!({
        "graphs": [{
            "nodes": {
                "node1": {
                    "output": ["node1"],
                    "ref_graph": "subgraph",
                    "input": ["data"]
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("node1 = ref(subgraph)(data);"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_node_with_start_end_markers() {
    let data = json!({
        "graphs": [{
            "nodes": {
                "start_node": {
                    "output": ["start_node"],
                    "op_name": "test.op",
                    "start": true
                },
                "end_node": {
                    "output": ["end_node"],
                    "op_name": "test.op",
                    "end": true
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("start_node = test.op().as(start);"));
            assert!(text.contains("end_node = test.op().as(end);"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_node_with_override() {
    let data = json!({
        "graphs": [{
            "nodes": {
                "node1": {
                    "output": ["node1"],
                    "op_name": "test.op",
                    "override": true
                },
                "node2": {
                    "output": ["node2"],
                    "op_name": "test.op",
                    "override": 42
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("node1 = test.op().override(true);"));
            assert!(text.contains("node2 = test.op().override(42);"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_operation_with_choice_spec() {
    let data = json!({
        "ops": [{
            "inputs": {
                "param": {
                    "dtype": "string",
                    "choice": ["option1", "option2", "option3"]
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("param:(dtype=string,choice=('option1','option2','option3'));"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_operation_with_range_spec() {
    let data = json!({
        "ops": [{
            "inputs": {
                "param": {
                    "dtype": "int",
                    "range": {"ge": 0, "le": 100}
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("param:(dtype=int,range=[0,100]);"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_operation_with_exact_length() {
    let data = json!({
        "ops": [{
            "inputs": {
                "param": {
                    "dtype": "string",
                    "length": {"eq": 10}
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("param:(dtype=string,length=10);"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_custom_indentation() {
    let data = json!({
        "graphs": [{
            "nodes": {
                "node1": {
                    "output": ["node1"],
                    "op_name": "test.op"
                }
            }
        }]
    });
    
    let options = DecompileOptions {
        indent: 2,
        ..Default::default()
    };
    
    let result = decompile_from_data(data, Some(options)).unwrap();
    match result {
        DecompileResult::Text(text) => {
            // Should have 2-space indentation instead of 4
            assert!(text.contains("\n  node1 = test.op();"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_decompile_options() {
    let data = json!({
        "graphs": [{
            "nodes": {
                "node1": {
                    "output": ["node1"],
                    "op_name": "test.op"
                }
            }
        }]
    });
    
    let options = DecompileOptions {
        indent: 8,
        max_col: 50,
        unescape: true,
        keep_order: true,
    };
    
    let result = decompile_from_data(data, Some(options)).unwrap();
    match result {
        DecompileResult::Text(text) => {
            // Should work with custom options
            assert!(text.contains("node1 = test.op();"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_complex_nested_structure() {
    let data = json!({
        "graphs": [{
            "property": {
                "config": {
                    "nested": {
                        "value": 42,
                        "list": [1, 2, 3]
                    }
                }
            },
            "nodes": {
                "complex_node": {
                    "output": ["complex_node"],
                    "op_name": "complex.op",
                    "input": [{
                        "key1": "value1",
                        "key2": 123
                    }]
                }
            }
        }]
    });
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            assert!(text.contains("graph {"));
            assert!(text.contains("complex_node = complex.op({key1:'value1',key2:123});"));
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_empty_data() {
    let data = json!({});
    
    let result = decompile_from_data(data, None).unwrap();
    match result {
        DecompileResult::Text(text) => {
            // Should produce empty string for empty input
            assert_eq!(text.trim(), "");
        },
        _ => panic!("Expected text result"),
    }
}

#[test]
fn test_invalid_input_not_object() {
    let data = json!("invalid string");
    
    let result = decompile_from_data(data, None);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Decompile input must be a JSON object"));
}