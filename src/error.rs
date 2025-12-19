//! Error handling for GOS
//! 
//! This module defines error types and handling mechanisms for the GOS,
//! providing detailed error information including position and context.

use std::fmt;
use thiserror::Error;

/// Parse error types
#[derive(Error, Debug, Clone)]
pub enum ParseError {
    #[error("Syntax error at line {line}, column {column}: {message}")]
    SyntaxError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Lexical error at line {line}, column {column}: illegal character '{character}'")]
    LexicalError {
        line: usize,
        column: usize,
        character: char,
    },

    #[error("Semantic error at line {line}, column {column}: {message}")]
    SemanticError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Duplicate definition: {name} at line {line}, column {column}")]
    DuplicateDefinition {
        name: String,
        line: usize,
        column: usize,
    },

    #[error("Deprecated feature: {feature} at line {line}, column {column}. {suggestion}")]
    DeprecatedFeature {
        feature: String,
        line: usize,
        column: usize,
        suggestion: String,
    },

    #[error("Unsupported feature: {feature} at line {line}, column {column}")]
    UnsupportedFeature {
        feature: String,
        line: usize,
        column: usize,
    },

    #[error("Invalid value: {message} at line {line}, column {column}")]
    InvalidValue {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Parse error: {message}")]
    General { message: String },

    #[error("IO error: {0}")]
    Io(String),

    #[error("Pest parsing error: {0}")]
    Pest(String),
}

impl ParseError {
    pub fn syntax_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::SyntaxError {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn lexical_error(line: usize, column: usize, character: char) -> Self {
        Self::LexicalError {
            line,
            column,
            character,
        }
    }

    pub fn semantic_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::SemanticError {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn duplicate_definition(name: impl Into<String>, line: usize, column: usize) -> Self {
        Self::DuplicateDefinition {
            name: name.into(),
            line,
            column,
        }
    }

    pub fn deprecated_feature(
        feature: impl Into<String>,
        line: usize,
        column: usize,
        suggestion: impl Into<String>,
    ) -> Self {
        Self::DeprecatedFeature {
            feature: feature.into(),
            line,
            column,
            suggestion: suggestion.into(),
        }
    }

    pub fn unsupported_feature(
        feature: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self::UnsupportedFeature {
            feature: feature.into(),
            line,
            column,
        }
    }

    pub fn invalid_value(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self::InvalidValue {
            message: message.into(),
            line,
            column,
        }
    }

    pub fn general(message: impl Into<String>) -> Self {
        Self::General {
            message: message.into(),
        }
    }

    /// Get the line number if available
    pub fn line(&self) -> Option<usize> {
        match self {
            ParseError::SyntaxError { line, .. }
            | ParseError::LexicalError { line, .. }
            | ParseError::SemanticError { line, .. }
            | ParseError::DuplicateDefinition { line, .. }
            | ParseError::DeprecatedFeature { line, .. }
            | ParseError::UnsupportedFeature { line, .. }
            | ParseError::InvalidValue { line, .. } => Some(*line),
            _ => None,
        }
    }

    /// Get the column number if available
    pub fn column(&self) -> Option<usize> {
        match self {
            ParseError::SyntaxError { column, .. }
            | ParseError::LexicalError { column, .. }
            | ParseError::SemanticError { column, .. }
            | ParseError::DuplicateDefinition { column, .. }
            | ParseError::DeprecatedFeature { column, .. }
            | ParseError::UnsupportedFeature { column, .. }
            | ParseError::InvalidValue { column, .. } => Some(*column),
            _ => None,
        }
    }
}

// Note: This implementation will be added when the parser module is complete
// impl From<pest::error::Error<Rule>> for ParseError {
//     fn from(error: pest::error::Error<Rule>) -> Self {
//         let (line, column) = match error.line_col {
//             pest::error::LineColLocation::Pos((line, col)) => (line, col),
//             pest::error::LineColLocation::Span((line, col), _) => (line, col),
//         };
//
//         ParseError::SyntaxError {
//             line,
//             column,
//             message: format!("Parsing failed: {}", error.variant),
//         }
//     }
// }

/// Result type for parsing operations
pub type ParseResult<T> = Result<T, ParseError>;

/// Error collection for batch error reporting
#[derive(Debug, Clone, Default)]
pub struct ErrorCollection {
    pub errors: Vec<ParseError>,
    pub warnings: Vec<ParseError>,
}

impl ErrorCollection {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: ParseError) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: ParseError) {
        self.warnings.push(warning);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }

    /// Convert to a single error if there are any errors
    pub fn into_result<T>(self, value: T) -> ParseResult<T> {
        if self.has_errors() {
            // Return the first error, or combine multiple errors
            if self.errors.len() == 1 {
                Err(self.errors.into_iter().next().unwrap())
            } else {
                let message = self
                    .errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("; ");
                Err(ParseError::general(format!("Multiple errors: {}", message)))
            }
        } else {
            Ok(value)
        }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::Io(err.to_string())
    }
}

impl<R> From<pest::error::Error<R>> for ParseError
where
    R: std::fmt::Debug + std::hash::Hash + std::marker::Copy + Ord,
{
    fn from(err: pest::error::Error<R>) -> Self {
        let (line, column) = match err.line_col {
            pest::error::LineColLocation::Pos((line, col)) => (line, col),
            pest::error::LineColLocation::Span((line, col), _) => (line, col),
        };
        
        ParseError::SyntaxError {
            line,
            column,
            message: format!("Parsing failed: {}", err.variant),
        }
    }
}

impl fmt::Display for ErrorCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.errors.is_empty() {
            writeln!(f, "Errors:")?;
            for error in &self.errors {
                writeln!(f, "  {}", error)?;
            }
        }

        if !self.warnings.is_empty() {
            writeln!(f, "Warnings:")?;
            for warning in &self.warnings {
                writeln!(f, "  {}", warning)?;
            }
        }

        Ok(())
    }
}

/// Helper functions for common error scenarios
pub mod helpers {
    use super::*;

    pub fn duplicate_var_as(name: &str, line: usize, column: usize) -> ParseError {
        ParseError::duplicate_definition(
            format!("var as '{}'", name),
            line,
            column,
        )
    }

    pub fn duplicate_import_as(name: &str, line: usize, column: usize) -> ParseError {
        ParseError::duplicate_definition(
            format!("import as '{}'", name),
            line,
            column,
        )
    }

    pub fn duplicate_graph_as(name: &str, line: usize, column: usize) -> ParseError {
        ParseError::duplicate_definition(
            format!("graph as '{}'", name),
            line,
            column,
        )
    }

    pub fn duplicate_op_as(name: &str, line: usize, column: usize) -> ParseError {
        ParseError::duplicate_definition(
            format!("op as '{}'", name),
            line,
            column,
        )
    }

    pub fn duplicate_attribute(name: &str, line: usize, column: usize) -> ParseError {
        ParseError::duplicate_definition(
            format!("attribute '{}'", name),
            line,
            column,
        )
    }

    pub fn duplicate_node_output(name: &str, line: usize, column: usize) -> ParseError {
        ParseError::duplicate_definition(
            format!("node output '{}'", name),
            line,
            column,
        )
    }

    pub fn deprecated_node_syntax(line: usize, column: usize) -> ParseError {
        ParseError::deprecated_feature(
            "node definition syntax",
            line,
            column,
            "Please use function-style node definition instead",
        )
    }

    pub fn deprecated_meta_syntax(line: usize, column: usize) -> ParseError {
        ParseError::deprecated_feature(
            "meta definition syntax",
            line,
            column,
            "Please use op definition instead",
        )
    }

    pub fn deprecated_datetime_literal(line: usize, column: usize) -> ParseError {
        ParseError::deprecated_feature(
            "datetime literal",
            line,
            column,
            "Please use date(\"2025-01-01 00:00:00\") to specify dates",
        )
    }

    pub fn unsupported_edge_syntax(line: usize, column: usize) -> ParseError {
        ParseError::unsupported_feature(
            "edge syntax",
            line,
            column,
        )
    }

    pub fn unsupported_from_import(line: usize, column: usize) -> ParseError {
        ParseError::unsupported_feature(
            "from import syntax",
            line,
            column,
        )
    }

    pub fn multiple_if_conditions(name: &str, line: usize, column: usize) -> ParseError {
        ParseError::invalid_value(
            format!("attribute '{}' cannot have multiple if conditions", name),
            line,
            column,
        )
    }

    pub fn multiple_else_values(name: &str, line: usize, column: usize) -> ParseError {
        ParseError::invalid_value(
            format!("attribute '{}' cannot have multiple else values", name),
            line,
            column,
        )
    }

    pub fn multiple_or_defaults(name: &str, line: usize, column: usize) -> ParseError {
        ParseError::invalid_value(
            format!("attribute '{}' cannot have multiple or defaults", name),
            line,
            column,
        )
    }
}