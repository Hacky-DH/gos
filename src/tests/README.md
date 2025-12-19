# GOS 测试套件

本目录包含了 GOS 解析器的全面测试套件，涵盖了解析器的各个方面和功能。

## 测试结构

### 测试模块

- **`mod.rs`** - 测试模块入口和通用工具函数
- **`parser_tests.rs`** - 基础解析功能测试
- **`error_tests.rs`** - 错误处理和边界条件测试
- **`integration_tests.rs`** - 集成测试和复杂场景测试

### 测试覆盖范围

#### 1. 基础解析功能测试 (`parser_tests.rs`)

**变量定义测试**
- 简单变量定义
- 带别名的变量定义
- 复杂值类型的变量定义

**导入语句测试**
- 简单导入
- 带别名的导入
- 多个导入语句

**图定义测试**
- 简单图定义
- 带别名的图定义
- 复杂图定义with节点配置

**注释测试**
- 单行注释
- 多行注释

**值类型测试**
- 字符串字面量（单引号、双引号、多行）
- 数值字面量（整数、浮点数、科学计数法）
- 布尔值和null
- 集合类型（列表、字典、元组、集合）

**混合内容测试**
- 多种语句类型组合
- 空内容处理
- 仅空白字符内容

#### 2. 错误处理测试 (`error_tests.rs`)

**语法错误测试**
- 未闭合的大括号
- 缺少分号
- 无效的变量语法
- 未终止的字符串
- 无效的数字格式

**语义错误测试**
- 重复变量定义
- 未定义引用

**已弃用功能测试**
- meta 定义语法弃用警告

**不支持功能测试**
- from import 语法

**词法错误测试**
- 无效字符
- Unicode 处理

**结构错误测试**
- 嵌套变量定义
- 格式错误的图结构
- 不完整的导入语句

**值错误测试**
- 无效的JSON结构
- 无效的列表结构
- 混合引号类型

**边界条件测试**
- 极长标识符
- 深度嵌套结构
- 空语句

**错误恢复测试**
- 错误位置报告
- 连续多个错误

#### 3. 集成测试 (`integration_tests.rs`)

**真实文件测试**
- 解析 `simple_test.gos`
- 解析 `test_example.gos`
- 解析 `demo/example.gos`

**解析选项测试**
- 不同解析选项组合
- 错误收集模式

**复杂场景测试**
- 大型复杂GOS文件
- Unicode和特殊字符处理

**性能测试**
- 大文件解析性能
- 深度嵌套结构处理

**文件I/O测试**
- 临时文件解析
- 空文件处理

**回归测试**
- 尾随逗号处理
- 各种位置的注释
- 空白字符敏感性

## 运行测试

### 运行所有测试
```bash
cargo test
```

### 运行特定测试模块
```bash
# 运行基础解析测试
cargo test parser_tests

# 运行错误处理测试
cargo test error_tests

# 运行集成测试
cargo test integration_tests
```

### 运行特定测试
```bash
# 运行变量定义测试
cargo test variable_tests

# 运行语法错误测试
cargo test syntax_error_tests

# 运行真实文件测试
cargo test real_file_tests
```

### 详细输出
```bash
# 显示详细测试输出
cargo test -- --nocapture

# 显示所有测试（包括成功的）
cargo test -- --show-output
```

## 测试工具函数

### `default_test_options()`
创建默认的测试解析选项，启用所有功能。

### `parse_test_gos(content: &str)`
使用默认测试选项解析GOS内容。

### `assert_parse_success(content: &str)`
断言解析成功并返回AST。

### `assert_parse_error(content: &str)`
断言解析失败并返回错误。

## 测试数据

测试使用以下数据源：
- 内联GOS代码片段
- 项目根目录的示例文件
- 动态生成的测试内容
- 临时文件

## 添加新测试

### 添加基础功能测试
在 `parser_tests.rs` 中添加新的测试模块或测试函数：

```rust
#[cfg(test)]
mod new_feature_tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        let content = r#"
        # 你的GOS代码
        "#;
        let ast = assert_parse_success(content);
        // 添加断言
    }
}
```

### 添加错误测试
在 `error_tests.rs` 中添加新的错误条件测试：

```rust
#[test]
fn test_new_error_condition() {
    let content = r#"
    # 会导致错误的GOS代码
    "#;
    let error = assert_parse_error(content);
    // 验证错误类型和消息
}
```

### 添加集成测试
在 `integration_tests.rs` 中添加复杂场景测试：

```rust
#[test]
fn test_complex_scenario() {
    let content = r#"
    # 复杂的真实世界GOS代码
    "#;
    let ast = assert_parse_success(content);
    // 验证复杂的AST结构
}
```

## 测试最佳实践

1. **明确的测试名称** - 使用描述性的测试函数名
2. **独立的测试** - 每个测试应该独立运行
3. **有意义的断言** - 验证具体的预期行为
4. **错误测试** - 测试错误条件和边界情况
5. **文档化** - 为复杂测试添加注释说明
6. **性能考虑** - 避免过于耗时的测试

## 持续集成

这些测试设计为在CI/CD管道中运行：
- 所有测试应该快速执行
- 测试不依赖外部资源
- 测试结果是确定性的
- 包含充分的错误信息

## 故障排除

### 常见问题

1. **解析失败** - 检查GOS语法是否正确
2. **AST结构不匹配** - 验证预期的AST节点类型
3. **错误类型不匹配** - 确认预期的错误类型
4. **文件不存在** - 确保测试文件路径正确

### 调试技巧

1. 使用 `--nocapture` 查看调试输出
2. 添加 `eprintln!` 语句进行调试
3. 使用 `pretty_assertions` 获得更好的断言输出
4. 检查解析器的调试模式输出
