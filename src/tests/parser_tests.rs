//! Basic parser functionality tests
//!
//! Tests for core parsing functionality including variables, imports, graphs,
//! operations, nodes, comments, and various value types.
#![allow(unused_imports)]

use crate::ast::*;
use crate::tests::*;

pub mod assert_ast {
    use crate::ast::*;
    pub fn assert_symbol_option(
        symbol: &Option<Symbol>,
        expected_pos: &Position,
        expected_name: &str,
        expected_kind: SymbolKind,
    ) {
        assert!(symbol.is_some());
        let symbol = symbol.as_ref().unwrap();
        assert_symbol(symbol, expected_pos, expected_name, expected_kind);
    }

    pub fn assert_symbol(
        symbol: &Symbol,
        expected_pos: &Position,
        expected_name: &str,
        expected_kind: SymbolKind,
    ) {
        assert_eq!(symbol.position, *expected_pos);
        assert_eq!(symbol.name, expected_name);
        assert_eq!(symbol.kind, expected_kind);
    }

    /// 辅助函数：断言字符串字面量属性值
    pub fn assert_string_value(value: Box<AstNodeEnum>, expected_pos: &Position, expected_value: &str) {
        match &*value {
            AstNodeEnum::StringLiteral(string_lit) => {
                assert_eq!(string_lit.position, *expected_pos);
                assert_eq!(string_lit.value, expected_value);
            }
            AstNodeEnum::MultiLineStringLiteral(string_lit) => {
                assert_eq!(string_lit.position, *expected_pos);
                assert_eq!(string_lit.value, expected_value);
            }
            _ => panic!("Expected StringLiteral for attribute value"),
        }
    }

    /// 辅助函数：断言数字字面量属性值
    pub fn assert_number_value(
        value: Box<AstNodeEnum>,
        expected_pos: &Position,
        expected_raw: &str,
        expected_value: i64,
    ) {
        match &*value {
            AstNodeEnum::NumberLiteral(num_lit) => {
                assert_eq!(num_lit.position, *expected_pos);
                assert_eq!(num_lit.value, expected_value);
                assert_eq!(num_lit.raw, expected_raw);
            }
            _ => panic!("Expected NumberLiteral for attribute value"),
        }
    }

    /// 辅助函数：断言浮点数字面量属性值
    pub fn assert_float_value(
        value: Box<AstNodeEnum>,
        expected_pos: &Position,
        expected_raw: &str,
        expected_value: f64,
    ) {
        match &*value {
            AstNodeEnum::FloatLiteral(float_lit) => {
                assert_eq!(float_lit.position, *expected_pos);
                assert_eq!(float_lit.value, expected_value);
                assert_eq!(float_lit.raw, expected_raw);
            }
            _ => panic!("Expected FloatLiteral for attribute value"),
        }
    }

    /// 辅助函数：断言布尔字面量属性值
    pub fn assert_bool_value(
        value: Box<AstNodeEnum>,
        expected_pos: &Position,
        expected_raw: &str,
        expected_value: bool,
    ) {
        match &*value {
            AstNodeEnum::BoolLiteral(bool_lit) => {
                assert_eq!(bool_lit.position, *expected_pos);
                assert_eq!(bool_lit.value, expected_value);
                assert_eq!(bool_lit.raw, expected_raw);
            }
            _ => panic!("Expected BoolLiteral for attribute value"),
        }
    }

    /// 辅助函数：断言日期字面量属性值
    pub fn assert_date_value(
        value: Box<AstNodeEnum>,
        expected_pos: &Position,
        expected_value: &str,
    ) {
        match &*value {
            AstNodeEnum::DateLiteral(date_lit) => {
                assert_eq!(date_lit.position, *expected_pos);
                assert_eq!(date_lit.value, expected_value);
            }
            _ => panic!("Expected DateLiteral for attribute value"),
        }
    }

    /// 辅助函数：断言null字面量属性值
    pub fn assert_null_value(value: Box<AstNodeEnum>, expected_pos: &Position) {
        match &*value {
            AstNodeEnum::NullLiteral(null_lit) => {
                assert_eq!(null_lit.position, *expected_pos);
            }
            _ => panic!("Expected NullLiteral for attribute value"),
        }
    }
}


#[cfg(test)]
mod value_tests {
    use crate::ast::*;
    use super::assert_ast::*;
    use crate::tests::*;

    #[test]
    fn test_parse_string_literals() {
        let content = r#"
var {
    single_quote = 'single quoted string';
    double_quote = "double quoted string";
    multiline = """
    This is a multiline
    string with newlines
    """;
};
"#;
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => {
                let mut pos = Position::new_all(2, 9, 1, 2);
                assert_eq!(module.position, pos);
                assert_eq!(module.children.len(), 1);
                match &module.children[0] {
                    AstNodeEnum::VarDef(var_def) => {
                        assert_eq!(var_def.children.len(), 3);
                        match &var_def.children[0] {
                            AstNodeEnum::AttrDef(attr_def) => {
                                pos.set(3, 3, 20, 42);
                                assert_string_value(attr_def.value.clone(), &pos, "single quoted string");
                            },
                            _ => panic!("Expected AttrDef"),
                        }
                        match &var_def.children[1] {
                            AstNodeEnum::AttrDef(attr_def) => {
                                pos.set(4, 4, 20, 42);
                                assert_string_value(attr_def.value.clone(), &pos, "double quoted string");
                            },
                            _ => panic!("Expected AttrDef"),
                        }
                        match &var_def.children[2] {
                            AstNodeEnum::AttrDef(attr_def) => {
                                pos.set(5, 8, 17, 8);
                                assert_string_value(attr_def.value.clone(), &pos, "\n    This is a multiline\n    string with newlines\n    ");
                            },
                            _ => panic!("Expected AttrDef"),
                        }
                    }
                    _ => panic!("Expected VarDef node"),
                }
            }
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_parse_complex_values() {
        let content = r#"
var {
    list_val = [89, -123, 3.14, -2.71, 1.23e-4, 0.0];
    dict_val = {
        "tuple_val": (1, "two", date("2025-01-01")),
        "set_val": {1,2,3},
        "string_val": "test string",
        "nest": [{"true_val": true},
            {"false_val": false},
            {"null_val": null}]
    };
};"#;
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => match &module.children[0] {
                AstNodeEnum::VarDef(var_def) => {
                    let mut pos = Position::new_all(2, 12, 1, 2);
                    assert_eq!(var_def.position, pos);
                    assert_eq!(var_def.children.len(), 2);
                    
                    // 测试第一个属性：list_val = [89, -123, 3.14, -2.71, 1.23e-4, 0.0]
                    match &var_def.children[0] {
                        AstNodeEnum::AttrDef(attr_def) => {
                            pos.set(3, 3, 5, 53);
                            assert_eq!(attr_def.position, pos);
                            pos.set(3, 3, 5, 13);
                            assert_symbol(&attr_def.name, &pos, "list_val", SymbolKind::VarAttr);
                            match &*attr_def.value {
                                AstNodeEnum::ListStatement(list_stmt) => {
                                    pos.set(3, 3, 16, 53);
                                    assert_eq!(list_stmt.position, pos);
                                    assert_eq!(list_stmt.items.len(), 6);
                                    // 验证列表中的各个元素
                                    pos.set(3, 3, 17, 19);
                                    assert_number_value(Box::new(list_stmt.items[0].clone()), &pos, "89", 89);
                                    pos.set(3, 3, 21, 25);
                                    assert_number_value(Box::new(list_stmt.items[1].clone()), &pos, "-123", -123);
                                    pos.set(3, 3, 27, 31);
                                    assert_float_value(Box::new(list_stmt.items[2].clone()), &pos, "3.14", 3.14);
                                    pos.set(3, 3, 33, 38);
                                    assert_float_value(Box::new(list_stmt.items[3].clone()), &pos, "-2.71", -2.71);
                                    pos.set(3, 3, 40, 47);
                                    assert_float_value(Box::new(list_stmt.items[4].clone()), &pos, "1.23e-4", 0.000123);
                                    pos.set(3, 3, 49, 52);
                                    assert_float_value(Box::new(list_stmt.items[5].clone()), &pos, "0.0", 0.0);
                                },
                                _ => panic!("Expected ListStatement"),
                            }
                        },
                        _ => panic!("Expected AttrDef"),
                    }
                    
                    // 测试第二个属性：dict_val = {...}
                    match &var_def.children[1] {
                        AstNodeEnum::AttrDef(attr_def) => {
                            pos.set(4, 11, 5, 6);
                            assert_eq!(attr_def.position, pos);
                            pos.set(4, 4, 5, 13);
                            assert_symbol(&attr_def.name, &pos, "dict_val", SymbolKind::VarAttr);
                            
                            // 验证字典内容
                            match &*attr_def.value {
                                AstNodeEnum::DictStatement(dict_stmt) => {
                                    pos.set(4, 11, 16, 6);
                                    assert_eq!(dict_stmt.position, pos);
                                    assert_eq!(dict_stmt.items.len(), 4);
                                    
                                    // 测试第一个字典项："tuple_val": (1, "two", date("2025-01-01"))
                                    let dict_item = &dict_stmt.items[0];
                                    pos.set(5, 5, 9, 52);
                                    assert_eq!(dict_item.position, pos);
                                    
                                    // 验证键
                                    pos.set(5, 5, 9, 20);
                                    assert_string_value(dict_item.key.clone(), &pos, "tuple_val");
                                    
                                    // 验证值（元组）
                                    match &*dict_item.value {
                                        AstNodeEnum::TupleStatement(tuple_stmt) => {
                                            pos.set(5, 5, 22, 52);
                                            assert_eq!(tuple_stmt.position, pos);
                                            assert_eq!(tuple_stmt.items.len(), 3);
                                            
                                            pos.set(5, 5, 23, 24);
                                            assert_number_value(Box::new(tuple_stmt.items[0].clone()), &pos, "1", 1);
                                            pos.set(5, 5, 26, 31);
                                            assert_string_value(Box::new(tuple_stmt.items[1].clone()), &pos, "two");
                                            pos.set(5, 5, 33, 51);
                                            assert_date_value(Box::new(tuple_stmt.items[2].clone()), &pos, "2025-01-01");
                                        },
                                        _ => panic!("Expected TupleStatement"),
                                    }
                                    
                                    // 测试第二个字典项："set_val": {1,2,3}
                                    let dict_item = &dict_stmt.items[1];
                                    pos.set(6, 6, 9, 27);
                                    assert_eq!(dict_item.position, pos);
                                    pos.set(6, 6, 9, 18);
                                    assert_string_value(dict_item.key.clone(), &pos, "set_val");
                                    
                                    match &*dict_item.value {
                                        AstNodeEnum::SetStatement(set_stmt) => {
                                            pos.set(6, 6, 20, 27);
                                            assert_eq!(set_stmt.position, pos);
                                            assert_eq!(set_stmt.items.len(), 3);
                                            
                                            pos.set(6, 6, 21, 22);
                                            assert_number_value(Box::new(set_stmt.items[0].clone()), &pos, "1", 1);
                                            pos.set(6, 6, 23, 24);
                                            assert_number_value(Box::new(set_stmt.items[1].clone()), &pos, "2", 2);
                                            pos.set(6, 6, 25, 26);
                                            assert_number_value(Box::new(set_stmt.items[2].clone()), &pos, "3", 3);
                                        },
                                        _ => panic!("Expected SetStatement"),
                                    }
                                    
                                    // 测试第三个字典项："string_val": "test string"
                                    let dict_item = &dict_stmt.items[2];
                                    pos.set(7, 7, 9, 36);
                                    assert_eq!(dict_item.position, pos);
                                    pos.set(7, 7, 9, 21);
                                    assert_string_value(dict_item.key.clone(), &pos, "string_val");
                                    pos.set(7, 7, 23, 36);
                                    assert_string_value(dict_item.value.clone(), &pos, "test string");
                                    
                                    // 测试第四个字典项："nest": [{"true_val": true}, {"false_val": false}, {"null_val": null}]
                                    let dict_item = &dict_stmt.items[3];
                                    pos.set(8, 10, 9, 32);
                                    assert_eq!(dict_item.position, pos);
                                    pos.set(8, 8, 9, 15);
                                    assert_string_value(dict_item.key.clone(), &pos, "nest");
                                    
                                    match &*dict_item.value {
                                        AstNodeEnum::ListStatement(nest_list) => {
                                            pos.set(8, 10, 17, 32);
                                            assert_eq!(nest_list.position, pos);
                                            assert_eq!(nest_list.items.len(), 3);
                                            
                                            // 测试嵌套列表中的第一个字典
                                            match &nest_list.items[0] {
                                                AstNodeEnum::DictStatement(nest_dict1) => {
                                                    pos.set(8, 8, 18, 36);
                                                    assert_eq!(nest_dict1.position, pos);
                                                    assert_eq!(nest_dict1.items.len(), 1);
                                                    
                                                    let nest_item1 = &nest_dict1.items[0];
                                                    pos.set(8, 8, 19, 35);
                                                    assert_eq!(nest_item1.position, pos);
                                                    pos.set(8, 8, 19, 29);
                                                    assert_string_value(nest_item1.key.clone(), &pos, "true_val");
                                                    pos.set(8, 8, 31, 35);
                                                    assert_bool_value(nest_item1.value.clone(), &pos, "true", true);
                                                },
                                                _ => panic!("Expected DictStatement"),
                                            }
                                            
                                            // 测试嵌套列表中的第二个字典
                                            match &nest_list.items[1] {
                                                AstNodeEnum::DictStatement(nest_dict2) => {
                                                    pos.set(9, 9, 13, 33);
                                                    assert_eq!(nest_dict2.position, pos);
                                                    assert_eq!(nest_dict2.items.len(), 1);
                                                    
                                                    let nest_item2 = &nest_dict2.items[0];
                                                    pos.set(9, 9, 14, 32);
                                                    assert_eq!(nest_item2.position, pos);
                                                    pos.set(9, 9, 14, 25);
                                                    assert_string_value(nest_item2.key.clone(), &pos, "false_val");
                                                    pos.set(9, 9, 27, 32);
                                                    assert_bool_value(nest_item2.value.clone(), &pos, "false", false);
                                                },
                                                _ => panic!("Expected DictStatement"),
                                            }
                                            
                                            // 测试嵌套列表中的第三个字典
                                            match &nest_list.items[2] {
                                                AstNodeEnum::DictStatement(nest_dict3) => {
                                                    pos.set(10, 10, 13, 31);
                                                    assert_eq!(nest_dict3.position, pos);
                                                    assert_eq!(nest_dict3.items.len(), 1);
                                                    
                                                    let nest_item3 = &nest_dict3.items[0];
                                                    pos.set(10, 10, 14, 30);
                                                    assert_eq!(nest_item3.position, pos);
                                                    pos.set(10, 10, 14, 24);
                                                    assert_string_value(nest_item3.key.clone(), &pos, "null_val");
                                                    pos.set(10, 10, 26, 30);
                                                    assert_null_value(nest_item3.value.clone(), &pos);
                                                },
                                                _ => panic!("Expected DictStatement"),
                                            }
                                        },
                                        _ => panic!("Expected ListStatement"),
                                    }
                                },
                                _ => panic!("Expected DictStatement"),
                            }
                        },
                        _ => panic!("Expected AttrDef"),
                    }
                }
                _ => panic!("Expected VarDef node"),
            },
            _ => panic!("Expected Module node"),
        }
    }
}

#[cfg(test)]
mod variable_tests {
    use crate::ast::*;
    use crate::tests::*;
    use super::assert_ast::*;
    // TODO 增加多个var和注释的测试， 空var  加if else

    #[test]
    fn test_parse_simple_var_definition() {
        let content = r#"
var {
    name = "test";
    value = 42;
} as config;"#;
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => {
                let mut pos = Position::new_all(2, 5, 1, 12);
                assert_eq!(module.position, pos);
                assert_eq!(module.children.len(), 1);
                match &module.children[0] {
                    AstNodeEnum::VarDef(var_def) => {
                        assert_eq!(var_def.position, pos);
                        assert!(var_def.offset.is_none());

                        pos.set(5, 5, 6, 12);
                        assert_symbol_option(&var_def.alias, &pos, "config", SymbolKind::VarAsName);

                        assert_eq!(var_def.children.len(), 2);
                        // 检查第一个属性定义：name = "test"
                        match &var_def.children[0] {
                            AstNodeEnum::AttrDef(attr_def) => {
                                pos.set(3, 3, 5, 18);
                                assert_eq!(attr_def.position, pos);
                                pos.set(3, 3, 5, 9);
                                assert_symbol(&attr_def.name, &pos, "name", SymbolKind::VarAttr);
                                assert!(attr_def.condition.is_none());
                                assert!(attr_def.else_value.is_none());
                                pos.set(3, 3, 12, 18);
                                assert_string_value(attr_def.value.clone(), &pos, "test");
                            }
                            _ => panic!("Expected AttrDef for first child"),
                        }

                        // 检查第二个属性定义：value = 42
                        match &var_def.children[1] {
                            AstNodeEnum::AttrDef(attr_def) => {
                                pos.set(4, 4, 5, 15);
                                assert_eq!(attr_def.position, pos);
                                pos.set(4, 4, 5, 10);
                                assert_symbol(&attr_def.name, &pos, "value", SymbolKind::VarAttr);
                                assert!(attr_def.condition.is_none());
                                assert!(attr_def.else_value.is_none());
                                pos.set(4, 4, 13, 15);
                                assert_number_value(attr_def.value.clone(), &pos, "42", 42);
                            }
                            _ => panic!("Expected AttrDef for second child"),
                        }
                    }
                    _ => panic!("Expected VarDef node"),
                }
            }
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_parse_var_without_alias() {
        let content = r#"
var {
    name = "test";
    value = 42;
};
"#;
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => match &module.children[0] {
                AstNodeEnum::VarDef(var_def) => {
                    assert!(var_def.alias.is_none());
                    assert_eq!(var_def.children.len(), 2);
                }
                _ => panic!("Expected VarDef node"),
            },
            _ => panic!("Expected Module node"),
        }
    }
}

#[cfg(test)]
mod import_tests {
    use crate::ast::*;
    use crate::tests::*;

    #[test]
    fn test_parse_simple_import() {
        let content = r#"import foo;"#;
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => {
                assert_eq!(module.children.len(), 1);
                match &module.children[0] {
                    AstNodeEnum::Import(import) => {
                        assert_eq!(import.items.len(), 1);
                        assert_eq!(import.items[0].path.name, "foo");
                        assert!(import.items[0].alias.is_none());
                    }
                    _ => panic!("Expected Import node"),
                }
            }
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_parse_import_with_alias() {
        let content = r#"import foo as bar;"#;
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => match &module.children[0] {
                AstNodeEnum::Import(import) => {
                    assert_eq!(import.items[0].path.name, "foo");
                    assert!(import.items[0].alias.is_some());
                    assert_eq!(import.items[0].alias.as_ref().unwrap().name, "bar");
                }
                _ => panic!("Expected Import node"),
            },
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_parse_multiple_imports() {
        let content = r#"import foo, bar, baz as qux;"#;
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => match &module.children[0] {
                AstNodeEnum::Import(import) => {
                    assert_eq!(import.items.len(), 3);
                    assert_eq!(import.items[0].path.name, "foo");
                    assert_eq!(import.items[1].path.name, "bar");
                    assert_eq!(import.items[2].path.name, "baz");
                    assert!(import.items[2].alias.is_some());
                }
                _ => panic!("Expected Import node"),
            },
            _ => panic!("Expected Module node"),
        }
    }
}

#[cfg(test)]
mod graph_tests {
    use crate::ast::*;
    use crate::tests::*;

    #[test]
    fn test_parse_simple_graph() {
        let content = r#"
graph {
    description = "test graph";
    input_node = data_loader();
}
"#;
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => {
                assert_eq!(module.children.len(), 1);
                match &module.children[0] {
                    AstNodeEnum::GraphDef(graph_def) => {
                        assert!(graph_def.alias.is_none());
                        assert!(!graph_def.children.is_empty());
                    }
                    _ => panic!("Expected GraphDef node"),
                }
            }
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_parse_graph_with_alias() {
        let content = r#"
graph {
    description = "test graph";
    input_node = data_loader();
} as test_pipeline;
"#;
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => match &module.children[0] {
                AstNodeEnum::GraphDef(graph_def) => {
                    assert!(graph_def.alias.is_some());
                }
                _ => panic!("Expected GraphDef node"),
            },
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_parse_complex_graph() {
        let content = r#"
graph {
    description = "Complex test pipeline";
    
    a, b = builtin.node1().with(
        description="node1 description",
        attr1="demo attr",
        attr2=23.8,
        attr3=true
    ).version('1.1.0');
    
    c = builtin.node2(a, b).with(
        attr1=42,
        attr2="test"
    ).version("1.2.0");
} as complex_pipeline.version("1.0.0");
"#;
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => match &module.children[0] {
                AstNodeEnum::GraphDef(graph_def) => {
                    assert!(graph_def.alias.is_some());
                    assert!(!graph_def.children.is_empty());
                }
                _ => panic!("Expected GraphDef node"),
            },
            _ => panic!("Expected Module node"),
        }
    }
}

#[cfg(test)]
mod comment_tests {
    use crate::ast::*;
    use crate::tests::*;

    #[test]
    fn test_parse_single_line_comment() {
        let content = r#"# This is a comment"#;
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => {
                assert_eq!(module.children.len(), 1);
                match &module.children[0] {
                    AstNodeEnum::Comment(comment) => {
                        assert!(comment.value.contains("This is a comment"));
                    }
                    _ => panic!("Expected Comment node"),
                }
            }
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_parse_multiline_comment() {
        let content = r#"
/*
This is a
multiline comment
*/
"#;
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => {
                assert_eq!(module.children.len(), 1);
                match &module.children[0] {
                    AstNodeEnum::Comment(comment) => {
                        assert!(comment.value.contains("multiline comment"));
                    }
                    _ => panic!("Expected Comment node"),
                }
            }
            _ => panic!("Expected Module node"),
        }
    }
}

#[cfg(test)]
mod mixed_content_tests {
    use crate::ast::*;
    use crate::tests::*;

    #[test]
    fn test_parse_mixed_statements() {
        let content = r#"
# Import statement
import builtin;

# Variable definition
var {
    name = "test pipeline";
    version = "1.0.0";
} as config;

# Graph definition
graph {
    description = config.name;
    node = builtin.processor().version(config.version);
} as pipeline;
"#;
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => {
                assert_eq!(module.children.len(), 4); // comment, import, var, graph
            }
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_parse_empty_content() {
        let content = "";
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => {
                assert_eq!(module.children.len(), 0);
            }
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_parse_whitespace_only() {
        let content = "   \n\n  \t  \n  ";
        let ast = assert_parse_success(content);

        match ast {
            AstNodeEnum::Module(module) => {
                assert_eq!(module.children.len(), 0);
            }
            _ => panic!("Expected Module node"),
        }
    }
}

mod comments_tests {
    use crate::ast::*;
    use crate::tests::*;

    #[test]
    fn test_comments() {
        // let content = r#"
        // # Single line comment
        // var {
        //     name = "test"; # Inline comment
        // };
        
        // """
        // Multi-line comment
        // spanning multiple lines
        // """
        // "#;

        // let result = parse(content);
        // assert!(result.is_ok(), "Failed to parse comments: {:?}", result);
    }
}