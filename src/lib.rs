//! GOS (Graph Representation Language) for Rust
//! 
//! This library provides a complete Rust implementation of the GOS.
//! It supports parsing GOS source code into Abstract Syntax Trees (AST) 
//! with full position tracking and comprehensive error reporting.
//! Additionally, it offers a compiler that can compile GOS ASTs into json format.
//! And, it offers a decompiler that can decompile GOS json format into GOS code.
//! Additionally, it provides a formatter that can format GOS code.
//! 
//! # Features
//! 
//! - Complete GOS language support (variables, imports, graphs, operations, nodes)
//! - AST generation with position information
//! - Compiler, decompiler and formatter support
//! - Comprehensive error handling with detailed location information
//! - Unicode string escape handling
//! 
//! # Usage
//! 
//! ```rust
//! use gos::parse_gos;
//! 
//! let content = r#"
//! var {
//!     name = "example";
//!     value = 42;
//! } as config;
//! "#;
//! 
//! match parse_gos(content, options) {
//!     Ok(ast) => println!("Parsed successfully: {:#?}", ast),
//!     Err(error) => eprintln!("Parse error: {}", error),
//! }
//! ```

pub mod ast;
pub mod compiler;
pub mod decompiler;
pub mod error;
pub mod format;
pub mod parser;

#[cfg(test)]
pub mod tests;

// Re-export main types for convenience
pub use ast::*;
pub use compiler::{compile_ast, compile_ast_with_options, Compiler, CompileOptions, CompileResult};
pub use decompiler::{decompile, decompile_from_data, DecompileOptions, DecompileResult};
pub use error::{ParseError, ParseResult, ErrorCollection};
pub use format::{format_from_data, format, Formatter, IndentBuffer};
pub use parser::{parse_gos, ParseOptions};

/// Parse GOS content with default options (AST mode enabled)
pub fn parse(content: &str) -> ParseResult<AstNodeEnum> {
    parse_gos(content, ParseOptions {
        ast: true,
        tracking: true,
        ..Default::default()
    })
}

/// Parse GOS content with error collection enabled
pub fn parse_with_errors(content: &str) -> (Option<AstNodeEnum>, ErrorCollection) {
    let options = ParseOptions {
        ast: true,
        error: true,
        tracking: true,
        ..Default::default()
    };
    
    match parse_gos(content, options) {
        Ok(ast) => (Some(ast), ErrorCollection::new()),
        Err(ParseError::General { message }) => {
            // Try to extract error collection from general error
            let mut errors = ErrorCollection::new();
            errors.add_error(ParseError::General { message });
            (None, errors)
        }
        Err(error) => {
            let mut errors = ErrorCollection::new();
            errors.add_error(error);
            (None, errors)
        }
    }
}

/// Validate GOS syntax without building AST
pub fn validate(content: &str) -> ParseResult<()> {
    parse_gos(content, ParseOptions {
        ast: false,
        error: true,
        ..Default::default()
    })?;
    Ok(())
}

/// Get version information
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod mytests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let content = r#"
        # Simple variable definition
        var {
            name = "test";
            value = 42;
        } as config;
        "#;

        let result = parse(content);
        assert!(result.is_ok(), "Failed to parse basic GOS: {:?}", result);
        
        if let Ok(AstNodeEnum::Module(module)) = result {
            assert_eq!(module.children.len(), 1);
            match &module.children[0] {
                AstNodeEnum::VarDef(var_def) => {
                    assert_eq!(var_def.children.len(), 2);
                }
                _ => panic!("Expected VarDef"),
            }
        }
    }

    #[test]
    fn test_error_handling() {
        let content = r#"
        var {
            name = "test
        }
        "#;

        let result = parse(content);
        // This should either succeed (if our grammar is lenient) or fail with a clear error
        match result {
            Ok(_) => {panic!("Not expected to succeed");}
            Err(error) => {
                assert_eq!(error.line().unwrap(), 3);
                assert_eq!(error.column().unwrap(), 20);
                assert!(error.to_string().contains("parsing error"),
                    "Error should contain 'error'");
            }
        }
    }


    #[test]
    fn test_validation() {
        let valid_content = r#"
        var { name = "test"; };
        "#;
        
        let _invalid_content = r#"
        var { name = ; # Invalid syntax
        "#;

        assert!(validate(valid_content).is_ok());
        assert!(validate(_invalid_content).is_err());
    }

    #[test]
    fn test_version() {
        let ver = version();
        assert!(!ver.is_empty(), "Version should not be empty");
        assert!(ver.contains('.'), "Version should contain dots");
    }

    #[test]
    fn test_parse_with_errors_succ() {
        let content = r#"
        var {
            name = "test";
        };
        "#;

        let (ast, errors) = parse_with_errors(content);
        assert!(ast.is_some(), "Should have AST");
        assert!(errors.is_empty() || !errors.has_errors(), "Should not have errors for valid content");
    }
    #[test]
    fn test_parse_with_errors() {
        let content = r#"
        var {
            name = ;
        };
        "#;

        let (ast, errors) = parse_with_errors(content);
        assert!(ast.is_none(), "Should not have AST");
        assert!(errors.has_errors(), "Should have errors");
    }
}