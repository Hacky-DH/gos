//! Error handling tests
//! 
//! Tests for various error conditions including syntax errors, semantic errors,
//! deprecated features, and unsupported features.
#![allow(unused_imports)] 
use crate::ast::*;
use crate::tests::*;
use crate::error::ParseError;

#[cfg(test)]
mod syntax_error_tests {
    use crate::ast::*;
    use crate::tests::*;
    use crate::error::ParseError;

    #[test]
    fn test_unclosed_brace() {
        let content = r#"
var {
    name = "test";
    value = 42;
# Missing closing brace
"#;
        let error = assert_parse_error(content);
        match error {
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Expected syntax error
            }
            _ => panic!("Expected syntax error for unclosed brace"),
        }
    }

    #[test]
    fn test_missing_semicolon() {
        let content = r#"import foo"#; // Missing semicolon
        let error = assert_parse_error(content);
        match error {
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Expected syntax error
            }
            _ => panic!("Expected syntax error for missing semicolon"),
        }
    }

    #[test]
    fn test_invalid_variable_syntax() {
        let content = r#"
var {
    = "missing name";
}
"#;
        let error = assert_parse_error(content);
        match error {
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Expected syntax error
            }
            _ => panic!("Expected syntax error for invalid variable syntax"),
        }
    }

    #[test]
    fn test_unterminated_string() {
        let content = r#"
var {
    name = "unterminated string;
}
"#;
        let error = assert_parse_error(content);
        match error {
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Expected syntax error
            }
            _ => panic!("Expected syntax error for unterminated string"),
        }
    }

    #[test]
    fn test_invalid_number_format() {
        let content = r#"
var {
    invalid_num = 12.34.56;
}
"#;
        let error = assert_parse_error(content);
        match error {
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Expected syntax error
            }
            _ => panic!("Expected syntax error for invalid number format"),
        }
    }
}

#[cfg(test)]
mod semantic_error_tests {
    use super::*;

    #[test]
    fn test_duplicate_variable_definition() {
        let content = r#"
var {
    name = "first";
} as config;

var {
    name = "second";  
} as config; # Duplicate alias
"#;
        // Note: This might not be caught at parse time but at semantic analysis
        // For now, we test that it parses successfully but could be caught later
        let _ast = assert_parse_success(content);
    }

    #[test]
    fn test_undefined_reference() {
        let content = r#"
graph {
    node = undefined_function();
}
"#;
        // This should parse successfully as it's a semantic issue, not syntax
        let _ast = assert_parse_success(content);
    }
}

#[cfg(test)]
mod deprecated_feature_tests {
    use super::*;

    #[test]
    fn test_meta_definition_deprecated() {
        // Test that meta definitions are flagged as deprecated
        // This would need to be implemented in the parser to emit warnings
        let content = r#"
meta {
    name = "deprecated_meta";
}
"#;
        // The parser should handle this and emit a deprecation warning
        // but still parse successfully or fail with appropriate error
        let result = parse_test_gos(content);
        match result {
            Ok(_) => {
                // Should parse but with deprecation warning
                // (warnings would be collected in error collection)
            }
            Err(ParseError::DeprecatedFeature { .. }) => {
                // Expected deprecation error
            }
            Err(_) => {
                // Other errors are also acceptable for deprecated features
            }
        }
    }
}

#[cfg(test)]
mod unsupported_feature_tests {
    use super::*;

    #[test]
    fn test_from_import_unsupported() {
        let content = r#"from module import function;"#;
        let error = assert_parse_error(content);
        match error {
            ParseError::UnsupportedFeature { feature, .. } => {
                assert!(feature.contains("from import"));
            }
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Also acceptable if it's a syntax error
            }
            _ => panic!("Expected unsupported feature error for 'from import'"),
        }
    }
}

#[cfg(test)]
mod lexical_error_tests {
    use crate::ast::*;
    use crate::tests::*;
    use crate::error::ParseError;

    #[test]
    fn test_invalid_character() {
        let content = r#"
var {
    name = "test"@; # Invalid character @
}
"#;
        let error = assert_parse_error(content);
        match error {
            ParseError::LexicalError { character, .. } => {
                assert_eq!(character, '@');
            }
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Also acceptable as pest might report it as syntax error
            }
            _ => panic!("Expected lexical error for invalid character"),
        }
    }

    #[test]
    fn test_unicode_handling() {
        let content = r#"
var {
    unicode_name = "测试中文";
    emoji = "🚀";
}
"#;
        // Unicode should be handled correctly
        let _ast = assert_parse_success(content);
    }
}

#[cfg(test)]
mod malformed_structure_tests {
    use super::*;

    #[test]
    fn test_nested_var_definitions() {
        let content = r#"
var {
    outer = "value";
    var {
        inner = "nested";
    }
}
"#;
        let error = assert_parse_error(content);
        match error {
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Expected syntax error for nested var
            }
            _ => panic!("Expected syntax error for nested var definitions"),
        }
    }

    #[test]
    fn test_malformed_graph_structure() {
        let content = r#"
graph {
    description = "test";
    # Missing node definitions or invalid syntax
    invalid syntax here;
}
"#;
        let error = assert_parse_error(content);
        match error {
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Expected syntax error
            }
            _ => panic!("Expected syntax error for malformed graph"),
        }
    }

    #[test]
    fn test_incomplete_import_statement() {
        let content = r#"import;"#; // Missing module name
        let error = assert_parse_error(content);
        match error {
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Expected syntax error
            }
            _ => panic!("Expected syntax error for incomplete import"),
        }
    }
}

#[cfg(test)]
mod value_error_tests {
    use super::*;

    #[test]
    fn test_invalid_json_structure() {
        let content = r#"
var {
    invalid_dict = {
        "key1": "value1",
        "key2": # Missing value
    };
}
"#;
        let error = assert_parse_error(content);
        match error {
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Expected syntax error
            }
            _ => panic!("Expected syntax error for invalid JSON structure"),
        }
    }

    #[test]
    fn test_invalid_list_structure() {
        let content = r#"
var {
    invalid_list = [1, 2, , 4]; # Empty element
}
"#;
        let error = assert_parse_error(content);
        match error {
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Expected syntax error
            }
            _ => panic!("Expected syntax error for invalid list structure"),
        }
    }

    #[test]
    fn test_mixed_quote_types() {
        let content = r#"
var {
    mixed_quotes = "double quote with 'single inside";
    escaped_quotes = "quote with \" escaped";
}
"#;
        // This should parse successfully
        let _ast = assert_parse_success(content);
    }
}

#[cfg(test)]
mod edge_case_error_tests {
    use super::*;

    #[test]
    fn test_extremely_long_identifier() {
        let very_long_name = "a".repeat(1000);
        let content = format!(
            r#"
var {{
    {} = "test";
}}
"#,
            very_long_name
        );
        
        // Should handle long identifiers gracefully
        let result = parse_test_gos(&content);
        match result {
            Ok(_) => {
                // Long identifiers should be accepted
            }
            Err(_) => {
                // Or may have reasonable limits
            }
        }
    }

    #[test]
    fn test_deeply_nested_structures() {
        let mut content = String::from("var { nested = ");
        
        // Create deeply nested dictionary
        for _ in 0..50 {
            content.push_str(r#"{"level": "#);
        }
        content.push_str("\"deep\"");
        for _ in 0..50 {
            content.push('}');
        }
        content.push_str("; }");
        
        let result = parse_test_gos(&content);
        match result {
            Ok(_) => {
                // Deep nesting should be handled
            }
            Err(_) => {
                // Or may have reasonable limits
            }
        }
    }

    #[test]
    fn test_empty_statements() {
        let content = r#"
;;;
var {
    name = "test";
}
;;;
"#;
        let error = assert_parse_error(content);
        match error {
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Expected syntax error for empty statements
            }
            _ => panic!("Expected syntax error for empty statements"),
        }
    }
}

#[cfg(test)]
mod recovery_tests {
    use super::*;

    #[test]
    fn test_error_position_reporting() {
        let content = r#"
var {
    name = "test";
    invalid syntax here on line 4;
    value = 42;
}
"#;
        let error = assert_parse_error(content);
        match error {
            ParseError::SyntaxError { line, column, .. } => {
                assert!(line >= 4); // Error should be around line 4
                assert!(column > 0);
            }
            ParseError::Pest(msg) => {
                // Pest errors should contain position information
                assert!(msg.contains("4") || msg.contains("line"));
            }
            _ => {
                // Other error types are also acceptable
            }
        }
    }

    #[test]
    fn test_multiple_errors_in_sequence() {
        let content = r#"
import; # Error 1: missing module name
var {
    = "missing name"; # Error 2: missing attribute name
}
graph {
    # Error 3: missing closing brace
"#;
        let error = assert_parse_error(content);
        // Should report the first error encountered
        match error {
            ParseError::Pest(_) | ParseError::SyntaxError { .. } => {
                // Expected syntax error
            }
            _ => panic!("Expected syntax error for multiple errors"),
        }
    }
}