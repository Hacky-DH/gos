//! Test module for GOS
//! 
//! This module contains comprehensive tests for the GOS implementation,
//! covering all major language constructs and error conditions.

pub mod parser_tests;
pub mod error_tests;
pub mod integration_tests;
pub mod decompiler_tests;

// Test utilities and common fixtures
use crate::{parse_gos, ParseOptions, AstNodeEnum};
use crate::error::ParseError;

/// Helper function to create default parse options for testing
pub fn default_test_options() -> ParseOptions {
    ParseOptions {
        ast: true,
        symbol: true,
        error: true,
        tracking: true,
        debug: false,
    }
}

/// Helper function to parse GOS content with default test options
pub fn parse_test_gos(content: &str) -> Result<AstNodeEnum, ParseError> {
    parse_gos(content, default_test_options())
}

/// Helper function to assert successful parsing
pub fn assert_parse_success(content: &str) -> AstNodeEnum {
    parse_test_gos(content).expect("Expected successful parsing")
}

/// Helper function to assert parsing failure
pub fn assert_parse_error(content: &str) -> ParseError {
    parse_test_gos(content).expect_err("Expected parsing to fail")
}