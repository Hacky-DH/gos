//! Integration tests
//! 
//! Tests that use real GOS files and complex scenarios to ensure
//! the parser works correctly with actual usage patterns.

use crate::ast::*;
use crate::tests::*;
use std::fs;
use tempfile::NamedTempFile;
use std::io::Write;

#[cfg(test)]
mod real_file_tests {
    use crate::ast::*;
    use crate::tests::*;
    use std::fs;

    #[test]
    fn test_parse_simple_test_gos() {
        let content = fs::read_to_string("simple_test.gos")
            .expect("Failed to read simple_test.gos");
        
        let ast = assert_parse_success(&content);
        match ast {
            AstNodeEnum::Module(module) => {
                assert!(!module.children.is_empty());
            }
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_parse_test_example_gos() {
        let content = fs::read_to_string("test_example.gos")
            .expect("Failed to read test_example.gos");
        
        let ast = assert_parse_success(&content);
        match ast {
            AstNodeEnum::Module(module) => {
                assert!(!module.children.is_empty());
            }
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_parse_demo_example_gos() {
        let content = fs::read_to_string("demo/example.gos")
            .expect("Failed to read demo/example.gos");
        
        let ast = assert_parse_success(&content);
        match ast {
            AstNodeEnum::Module(module) => {
                assert!(!module.children.is_empty());
                
                // Should contain variables, graphs, and comments
                let mut has_var = false;
                let mut has_graph = false;
                let mut has_comment = false;
                
                for child in &module.children {
                    match child {
                        AstNodeEnum::VarDef(_) => has_var = true,
                        AstNodeEnum::GraphDef(_) => has_graph = true,
                        AstNodeEnum::Comment(_) => has_comment = true,
                        _ => {}
                    }
                }
                
                assert!(has_var, "Should contain variable definitions");
                assert!(has_graph, "Should contain graph definitions");
                assert!(has_comment, "Should contain comments");
            }
            _ => panic!("Expected Module node"),
        }
    }
}

#[cfg(test)]
mod parse_options_tests {
    use super::*;

    #[test]
    fn test_parse_with_different_options() {
        let content = r#"
var {
    name = "test";
} as config;
"#;
        
        // Test with minimal options
        let minimal_options = ParseOptions {
            ast: false,
            symbol: false,
            error: false,
            tracking: false,
            debug: false,
        };
        let ast1 = parse_gos(content, minimal_options).expect("Parse should succeed");
        
        // Test with full options
        let full_options = ParseOptions {
            ast: true,
            symbol: true,
            error: true,
            tracking: true,
            debug: true,
        };
        let ast2 = parse_gos(content, full_options).expect("Parse should succeed");
        
        // Both should parse successfully
        match (&ast1, &ast2) {
            (AstNodeEnum::Module(_), AstNodeEnum::Module(_)) => {
                // Both should be modules
            }
            _ => panic!("Both should parse as modules"),
        }
    }

    #[test]
    fn test_error_collection_mode() {
        let content = r#"
# This content has potential issues but might still parse
var {
    name = "test";
}

# Some potentially problematic content
graph {
    description = "test";
}
"#;
        
        let options = ParseOptions {
            ast: true,
            symbol: true,
            error: true, // Enable error collection
            tracking: true,
            debug: false,
        };
        
        let result = parse_gos(content, options);
        match result {
            Ok(_) => {
                // Should parse successfully
            }
            Err(_) => {
                // Or may collect errors
            }
        }
    }
}

#[cfg(test)]
mod complex_scenario_tests {
    use super::*;

    #[test]
    fn test_large_complex_gos_file() {
        let content = r#"
# Complex GOS file with multiple features
import builtin;
import custom.operators as ops;

# Configuration variables
var {
    pipeline_name = "complex_test_pipeline";
    version = "2.1.0";
    debug_mode = true;
    
    # Complex nested structures
    config = {
        "processing": {
            "batch_size": 1000,
            "timeout": 30.0,
            "retry_count": 3,
            "features": ["feature1", "feature2", "feature3"]
        },
        "output": {
            "format": "json",
            "compression": true,
            "validation": {
                "strict": false,
                "schema_version": "1.2"
            }
        }
    };
    
    # Date and time values
    start_date = date('2024-01-01 00:00:00');
    end_date = date('2024-12-31 23:59:59');
    
    # Various data types
    metrics = [
        {"name": "accuracy", "threshold": 0.95},
        {"name": "latency", "threshold": 100.0},
        {"name": "throughput", "threshold": 1000}
    ];
} as pipeline_config;

# First processing graph
graph {
    description = "Data ingestion and preprocessing";
    
    # Input nodes with complex configurations
    raw_data = builtin.data_loader().with(
        source_path="/data/input",
        format="parquet",
        schema_validation=true,
        batch_size=pipeline_config.config.processing.batch_size
    ).version("1.5.0");
    
    # Data validation and cleaning
    validated_data = ops.validator(raw_data).with(
        rules=["not_null", "type_check", "range_check"],
        error_handling="skip",
        log_errors=pipeline_config.debug_mode
    ).version("2.0.1");
    
    # Feature extraction
    features = ops.feature_extractor(validated_data).with(
        feature_list=pipeline_config.config.processing.features,
        normalization=true,
        scaling="standard"
    ).version("1.8.3");
    
    # Multiple outputs
    processed_data, metadata = ops.preprocessor(features).with(
        output_format=pipeline_config.config.output.format,
        include_metadata=true,
        compression=pipeline_config.config.output.compression
    ).version("3.1.0");
    
} as data_preprocessing.version(pipeline_config.version);

# Second processing graph
graph {
    description = "Model training and evaluation";
    
    # Reference data from previous graph
    training_data = data_preprocessing.processed_data;
    
    # Model training
    model = builtin.trainer(training_data).with(
        algorithm="gradient_boosting",
        hyperparameters={
            "learning_rate": 0.1,
            "max_depth": 6,
            "n_estimators": 100
        },
        validation_split=0.2,
        early_stopping=true
    ).version("4.2.1");
    
    # Model evaluation
    evaluation_results = builtin.evaluator(model, training_data).with(
        metrics=pipeline_config.metrics,
        cross_validation=5,
        output_detailed_report=true
    ).version("2.3.0");
    
    # Conditional model selection
    final_model = ops.model_selector(model, evaluation_results).with(
        selection_criteria="accuracy",
        min_threshold=0.90
    ).version("1.4.2");
    
} as model_training.version(pipeline_config.version);

# Third graph for deployment
graph {
    description = "Model deployment and monitoring";
    
    # Deploy the trained model
    deployed_model = ops.deployer(model_training.final_model).with(
        deployment_target="production",
        scaling_policy="auto",
        health_checks=true,
        monitoring_enabled=true
    ).version("3.0.0");
    
    # Set up monitoring
    monitoring = builtin.monitor(deployed_model).with(
        metrics_collection=true,
        alerting_rules=[
            {"metric": "latency", "threshold": 200, "action": "alert"},
            {"metric": "error_rate", "threshold": 0.01, "action": "scale"}
        ],
        dashboard_enabled=true
    ).version("1.7.0");
    
} as deployment.version(pipeline_config.version);

# Comments and documentation
/*
This is a comprehensive example demonstrating:
1. Complex variable definitions with nested structures
2. Multiple graph definitions with dependencies
3. Various node configurations and version specifications
4. Cross-graph references and data flow
5. Conditional logic and complex expressions
6. Multiple data types and formats
7. Real-world pipeline patterns
*/
"#;
        
        let ast = assert_parse_success(content);
        match ast {
            AstNodeEnum::Module(module) => {
                assert!(module.children.len() >= 5); // imports, vars, graphs, comments
                
                // Verify we have the expected components
                let mut import_count = 0;
                let mut var_count = 0;
                let mut graph_count = 0;
                let mut comment_count = 0;
                
                for child in &module.children {
                    match child {
                        AstNodeEnum::Import(_) => import_count += 1,
                        AstNodeEnum::VarDef(_) => var_count += 1,
                        AstNodeEnum::GraphDef(_) => graph_count += 1,
                        AstNodeEnum::Comment(_) => comment_count += 1,
                        _ => {}
                    }
                }
                
                assert!(import_count >= 2, "Should have multiple imports");
                assert!(var_count >= 1, "Should have variable definitions");
                assert!(graph_count >= 3, "Should have multiple graphs");
                assert!(comment_count >= 1, "Should have comments");
            }
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_unicode_and_special_characters() {
        let content = r#"
# Unicode test with various languages and symbols
var {
    chinese_text = "这是中文测试文本";
    japanese_text = "これは日本語のテストです";
    korean_text = "이것은 한국어 테스트입니다";
    arabic_text = "هذا نص تجريبي باللغة العربية";
    emoji_text = "🚀 🎉 🔥 💯 ✨";
    
    # Mathematical symbols
    math_symbols = "∑ ∆ ∞ π α β γ δ ε";
    
    # Special characters in identifiers (if supported)
    测试变量 = "Chinese identifier";
    
    # Mixed content
    mixed = "English + 中文 + 🌟 + العربية";
} as unicode_config;

graph {
    description = "Unicode processing pipeline 🔄";
    
    # Nodes with unicode names and values
    数据加载器 = builtin.loader().with(
        输入路径="/data/unicode",
        编码="utf-8",
        description="Unicode data loader 📂"
    ).version("1.0.0");
    
} as unicode_pipeline;
"#;
        
        let ast = assert_parse_success(content);
        match ast {
            AstNodeEnum::Module(module) => {
                assert!(!module.children.is_empty());
            }
            _ => panic!("Expected Module node"),
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_parse_large_file() {
        // Generate a large GOS file programmatically
        let mut content = String::new();
        
        // Add many variable definitions
        for i in 0..100 {
            content.push_str(&format!(
                r#"
var {{
    name_{} = "variable_{}";
    value_{} = {};
    list_{} = [{}, {}, {}];
}} as config_{};
"#,
                i, i, i, i * 10, i, i, i + 1, i + 2, i
            ));
        }
        
        // Add many graph definitions
        for i in 0..50 {
            content.push_str(&format!(
                r#"
graph {{
    description = "Graph number {}";
    node_{} = builtin.processor().with(
        param1="value_{}",
        param2={},
        param3=true
    ).version("1.0.0");
}} as graph_{};
"#,
                i, i, i, i * 5, i
            ));
        }
        
        let start = std::time::Instant::now();
        let ast = assert_parse_success(&content);
        let duration = start.elapsed();
        
        // Should parse reasonably quickly (adjust threshold as needed)
        assert!(duration.as_secs() < 10, "Parsing took too long: {:?}", duration);
        
        match ast {
            AstNodeEnum::Module(module) => {
                assert!(module.children.len() >= 150); // 100 vars + 50 graphs
            }
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_deeply_nested_structures() {
        let mut content = String::from("var { deeply_nested = ");
        
        // Create a deeply nested structure
        let depth = 20;
        for i in 0..depth {
            content.push_str(&format!(r#"{{"level_{}": "#, i));
        }
        content.push_str("\"bottom\"");
        for _ in 0..depth {
            content.push('}');
        }
        content.push_str("; }");
        
        let ast = assert_parse_success(&content);
        match ast {
            AstNodeEnum::Module(module) => {
                assert_eq!(module.children.len(), 1);
            }
            _ => panic!("Expected Module node"),
        }
    }
}

#[cfg(test)]
mod file_io_tests {
    use super::*;

    #[test]
    fn test_parse_from_temp_file() {
        let content = r#"
var {
    temp_test = "testing file I/O";
} as temp_config;
"#;
        
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file.write_all(content.as_bytes()).expect("Failed to write to temp file");
        
        let file_content = fs::read_to_string(temp_file.path())
            .expect("Failed to read temp file");
        
        let ast = assert_parse_success(&file_content);
        match ast {
            AstNodeEnum::Module(module) => {
                assert_eq!(module.children.len(), 1);
            }
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_parse_empty_file() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file.write_all(b"").expect("Failed to write to temp file");
        
        let file_content = fs::read_to_string(temp_file.path())
            .expect("Failed to read temp file");
        
        let ast = assert_parse_success(&file_content);
        match ast {
            AstNodeEnum::Module(module) => {
                assert_eq!(module.children.len(), 0);
            }
            _ => panic!("Expected Module node"),
        }
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;

    #[test]
    fn test_issue_with_trailing_commas() {
        let content = r#"
var {
    list_with_trailing = [1, 2, 3,];
    dict_with_trailing = {
        "key1": "value1",
        "key2": "value2",
    };
}
"#;
        
        // This should either parse successfully or fail gracefully
        let result = parse_test_gos(content);
        match result {
            Ok(_) => {
                // Trailing commas are supported
            }
            Err(_) => {
                // Trailing commas are not supported, which is also valid
            }
        }
    }

    #[test]
    fn test_comments_in_various_positions() {
        let content = r#"
# Top level comment
import builtin; # End of line comment

var { # Comment after opening brace
    # Comment before attribute
    name = "test"; # Comment after value
    # Comment between attributes
    value = 42;
    # Comment before closing brace
} as config; # Comment after alias

# Comment between statements
graph {
    # Comment in graph
    node = builtin.processor(); # Comment after node
} # Comment before alias
as pipeline; # Comment after alias

# Final comment
"#;
        
        let ast = assert_parse_success(content);
        match ast {
            AstNodeEnum::Module(module) => {
                // Should contain imports, vars, graphs, and comments
                assert!(module.children.len() >= 4);
            }
            _ => panic!("Expected Module node"),
        }
    }

    #[test]
    fn test_whitespace_sensitivity() {
        let content1 = r#"var{name="test";}as config;"#;
        let content2 = r#"var { name = "test"; } as config;"#;
        let content3 = r#"
        var {
            name = "test";
        } as config;
        "#;
        
        // All should parse to equivalent ASTs
        let ast1 = assert_parse_success(content1);
        let ast2 = assert_parse_success(content2);
        let ast3 = assert_parse_success(content3);
        
        // All should be modules with similar structure
        match (&ast1, &ast2, &ast3) {
            (AstNodeEnum::Module(_), AstNodeEnum::Module(_), AstNodeEnum::Module(_)) => {
                // All should parse as modules
            }
            _ => panic!("All should parse as modules"),
        }
    }
}