# GOS

一个完整的Rust实现的图编排规范GOS (Graph Orchestration Specification) ，包括解析器，编译器，反编译器，AST，格式化等。

图编排规范，语法类似python和toml格式，比较易于上手，用来描述图编排以及元数据。可用于各类图编排场景，比如RAG中图编排，推荐，搜索等等场景算子编排，任务编排等等。


## 功能特性

- ✅ **完整的GOS语言支持**
  - 图定义 (`graph`)
  - 节点定义
  - 变量定义 (`var`)
  - 导入语句 (`import`)
  - 操作定义 (`op`)
  - 注释和多行字符串

- ✅ **AST生成**
  - 完整的位置信息跟踪
  - 支持所有数据类型（字符串、数字、布尔值、集合等等）

- ✅ **错误处理**
  - 详细的错误位置信息
  - 语法错误、语义错误、重复定义检测
  - 弃用功能警告
  - 批量错误收集

- ✅ **Unicode支持**
  - 字符串转义处理
  - 多行字符串支持

## 项目结构

```
src/
├── lib.rs          # 库入口，公共API
├── main.rs         # 命令行工具
├── gos.pest        # Pest语法文件
├── ast.rs          # AST节点类型定义
├── parser.rs       # 解析器实现
├── compiler.rs     # 编译器
├── format.rs       # 格式化
└── error.rs        # 错误处理
```

## 核心组件

### 1. AST节点类型 (`src/ast.rs`)

定义了与Python实现相对应的所有AST节点类型：

- `Module` - 顶层模块
- `Symbol` - 标识符和符号
- `Comment` - 注释节点
- 字面量类型：`StringLiteral`, `NumberLiteral`, `FloatLiteral`, `BoolLiteral`, `DateLiteral`
- 集合类型：`DictStatement`, `ListStatement`, `TupleStatement`, `SetStatement`
- 语言构造：`VarDef`, `GraphDef`, `NodeDef`, `OpDef`, `Import`

### 2. 词法和语法分析 (`src/gos.pest`)

使用Pest解析器生成器定义的完整GOS语法：

- 关键字和保留字
- 字面量（字符串、数字、布尔值、日期时间）
- 操作符和分隔符
- 语法规则（变量、导入、图、操作等）

### 3. 解析器实现 (`src/parser.rs`)

核心解析逻辑：

- `GosParser` - Pest生成的解析器
- `ParseOptions` - 解析选项配置
- `parse_gos()` - 主解析函数
- Unicode转义处理
- 位置信息跟踪

### 4. 错误处理 (`src/error.rs`)

全面的错误处理系统：

- `ParseError` - 各种错误类型
- `ErrorCollection` - 批量错误收集
- 详细的位置信息（行号、列号）
- 弃用功能警告

## 使用方法

### 作为库使用

```rust
use gos_parser::{parse, ParseOptions};

let content = r#"
var {
    name = "example";
    value = 42;
} as config;
"#;

match parse(content) {
    Ok(ast) => println!("解析成功: {:#?}", ast),
    Err(error) => eprintln!("解析错误: {}", error),
}
```

### 命令行工具

```bash
# 解析文件并输出JSON
cargo run -- example.gos

# 输出美化的AST
cargo run -- -f pretty example.gos

# 保存到文件
cargo run -- -o output.json example.gos

# 从标准输入读取
cat example.gos | cargo run -- -

# 启用调试和错误模式
cargo run -- --debug --error example.gos
```


## 技术选择说明

### 为什么选择Pest而不是ANTLR4Rust？

1. **更好的Rust生态集成** - Pest是专为Rust设计的解析器生成器
2. **更高的性能** - 编译时生成，运行时开销更小
3. **更简洁的语法** - PEG语法比ANTLR的EBNF更直观
4. **更好的错误消息** - 提供更详细的解析错误信息
5. **更小的依赖** - 相比ANTLR4Rust有更少的外部依赖

### 架构设计

- **模块化设计** - 清晰分离AST、解析器、错误处理
- **类型安全** - 充分利用Rust的类型系统确保内存安全
- **零拷贝优化** - 尽可能避免不必要的字符串复制
- **错误恢复** - 支持错误收集模式，一次性报告多个错误


## 开发状态

- ✅ 核心解析功能完成
- ✅ AST节点类型定义完成
- ✅ 错误处理机制完成
- ✅ 命令行工具完成
- 🔄 测试用例开发中
- ⏳ 性能优化待完成

## 性能特性

- **编译时优化** - Pest在编译时生成解析器代码
- **零分配解析** - 在可能的情况下避免内存分配
- **增量解析** - 支持流式解析大文件
- **内存效率** - 使用引用而非拷贝减少内存使用

## 未来改进

1. **性能优化**
   - 并行解析支持
   - 内存池分配器
   - SIMD优化的字符串处理

2. **功能扩展**
   - LSP (Language Server Protocol) 支持
   - 语法高亮
   - 自动补全

3. **工具链集成**
   - Cargo插件
   - IDE扩展
   - CI/CD集成

## 贡献指南

1. Fork项目
2. 创建功能分支
3. 提交更改
4. 创建Pull Request

## 许可证

MIT License - 详见LICENSE文件
