//! AST (Abstract Syntax Tree) node definitions for GOS parser
//!
//! This module defines all the AST node types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Position information for AST nodes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    /// Starting line number (1-based)
    pub line: usize,
    /// Ending line number (1-based)  
    pub end_line: usize,
    /// Starting column (1-based)
    pub start: usize,
    /// Ending column (1-based)
    pub end: usize,
}

impl Position {
    pub fn new(line: usize, start: usize, end: usize) -> Self {
        Self {
            line,
            end_line: line,
            start,
            end,
        }
    }

    pub fn new_all(line: usize, end_line: usize, start: usize, end: usize) -> Self {
        Self {
            line,
            end_line: end_line,
            start,
            end,
        }
    }

    pub fn with_end_line(mut self, end_line: usize) -> Self {
        self.end_line = end_line;
        self
    }

    pub fn set(&mut self, line: usize, end_line: usize, start: usize, end: usize) {
        self.line = line;
        self.end_line = end_line;
        self.start = start;
        self.end = end;
    }
}

/// Base trait for all AST nodes
pub trait AstNode {
    fn position(&self) -> &Position;
    fn position_mut(&mut self) -> &mut Position;
}

/// Symbol kinds corresponding to SymbolKind enum in Python
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    Unknown,
    ImportName,
    ImportAsName,
    VarAttr,
    VarAsName,
    VarRef,
    GraphProperty,
    GraphAsName,
    RefGraphName,
    GraphTemplate,
    NodeName,
    NodeOutput,
    NodeInput,
    NodeDepend,
    NodeProperty,
    NodeAttr,
    NodeAsName,
    OpAsName,
    OpMetaAttr,
    OpInputAttr,
    OpOutputAttr,
    OpConfigAttr,
    NodeAttrName,
    NodeInputKey,
    OpSpecDtype,
    ForLoopInputs,
    ForLoopOutputs,
}

/// Module - top-level AST node representing a GOS file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub position: Position,
    pub children: Vec<AstNodeEnum>,
}

/// Comment node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
    pub position: Position,
    pub value: String,
}

/// Symbol - represents identifiers with kind information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol {
    pub position: Position,
    pub name: String,
    pub kind: SymbolKind,
}

impl Symbol {
    pub fn new(position: Position, name: String) -> Self {
        Self {
            position,
            name,
            kind: SymbolKind::Unknown,
        }
    }

    pub fn with_kind(mut self, kind: SymbolKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn set_kind(&mut self, kind: SymbolKind) {
        self.kind = kind;
    }
}

/// Reference to another symbol
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ref {
    pub name: Symbol,
}

/// String literal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StringLiteral {
    pub position: Position,
    pub value: String,
}

/// Multi-line string literal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiLineStringLiteral {
    pub position: Position,
    pub value: String,
}

/// Number literal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NumberLiteral {
    pub position: Position,
    pub raw: String,
    pub value: i64,
}

/// Float literal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatLiteral {
    pub position: Position,
    pub raw: String,
    pub value: f64,
}

/// Boolean literal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoolLiteral {
    pub position: Position,
    pub raw: String,
    pub value: bool,
}

/// DateTime literal (deprecated)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateTimeLiteral {
    pub position: Position,
    pub raw: String,
    pub value: DateTime<Utc>,
}

/// Date literal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateLiteral {
    pub position: Position,
    pub value: String,
}

/// Null literal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NullLiteral {
    pub position: Position,
}

/// Dictionary statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictStatement {
    pub position: Position,
    pub items: Vec<DictItem>,
}

/// Dictionary item (key-value pair)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictItem {
    pub position: Position,
    pub key: Box<AstNodeEnum>,
    pub value: Box<AstNodeEnum>,
}

/// List statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListStatement {
    pub position: Position,
    pub items: Vec<AstNodeEnum>,
}

/// Tuple statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupleStatement {
    pub position: Position,
    pub items: Vec<AstNodeEnum>,
}

/// Set statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStatement {
    pub position: Position,
    pub items: Vec<AstNodeEnum>,
}

/// Import statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Import {
    pub position: Position,
    pub items: Vec<ImportItem>,
}

/// Import item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportItem {
    pub position: Position,
    pub path: Symbol,
    pub alias: Option<Symbol>,
}

/// Attribute definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttrDef {
    pub position: Position,
    pub name: Symbol,
    pub value: Box<AstNodeEnum>,
    pub condition: Option<Box<AstNodeEnum>>,
    pub else_value: Option<Box<AstNodeEnum>>,
}

/// Reference definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefDef {
    pub position: Position,
    pub name: Symbol,
    pub value: Symbol,
    pub condition: Option<Box<AstNodeEnum>>,
    pub default: Option<Box<AstNodeEnum>>,
}

/// Variable definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VarDef {
    pub position: Position,
    pub children: Vec<AstNodeEnum>,
    pub alias: Option<Symbol>,
    pub offset: Option<HashMap<String, usize>>,
}

/// Graph definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphDef {
    pub position: Position,
    pub children: Vec<AstNodeEnum>,
    pub alias: Option<Symbol>,
    pub version: Option<Box<AstNodeEnum>>,
    pub template_graph: Option<Symbol>,
    pub template_version: Option<Box<AstNodeEnum>>,
    pub offset: Option<HashMap<String, usize>>,
}

/// Node definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeDef {
    pub position: Position,
    pub outputs: Vec<Symbol>,
    pub value: NodeBlock,
}

/// Node block definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeBlock {
    pub position: Position,
    pub name: Symbol,
    pub inputs: Option<NodeInputDef>,
    pub attrs: Option<Vec<NodeAttr>>,
}

/// ref Graph block definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefGraphBlock {
    pub position: Position,
    pub ref_name: Symbol,
    pub inputs: Option<NodeInputDef>,
    pub attrs: Option<Vec<NodeAttr>>,
}

/// Node input definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeInputDef {
    Tuple(NodeInputTuple),
    KeyValue(NodeInputKeyDef),
}

/// Node input tuple (positional arguments)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInputTuple {
    pub position: Position,
    pub items: Vec<Box<AstNodeEnum>>,
}

/// Node input key-value definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInputKeyDef {
    pub position: Position,
    pub items: Vec<NodeInputKeyItem>,
}

/// Node input key-value item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInputKeyItem {
    pub position: Position,
    pub key: Symbol,
    pub value: Box<AstNodeEnum>,
}

/// Node input values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInputValues {
    pub position: Position,
    pub items: Vec<Symbol>,
}

/// Node attribute
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeAttr {
    pub position: Position,
    pub name: Symbol,
    pub value: NodeAttrValue,
    pub offset: Option<HashMap<String, usize>>,
}

/// Node attribute value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeAttrValue {
    Symbol(Symbol),
    String(StringLiteral),
    List(Vec<AstNodeEnum>),
}

/// Condition definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionDef {
    pub position: Position,
    pub outputs: Vec<Symbol>,
    pub value: Box<ConditionBlock>,
}

/// Condition block
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionBlock {
    pub position: Position,
    pub condition: Box<ConditionExpr>,
    pub true_branch: Box<AstNodeEnum>,
    pub false_branch: Box<AstNodeEnum>,
}

/// Condition expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionExpr {
    Statement(Box<ConditionStatement>),
    Block(NodeBlock),
}

/// Condition statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionStatement {
    pub position: Position,
    pub left_operand: Box<AstNodeEnum>,
    pub right_operand: Box<AstNodeEnum>,
    pub operator: String,
}

/// For loop block
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForLoopBlock {
    pub position: Position,
    pub inputs: Symbol,
    pub outputs: Vec<Symbol>,
    pub node: NodeBlock,
    pub condition: Option<Box<AstNodeEnum>>,
    pub offset: Option<HashMap<String, usize>>,
}

/// Op definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpDef {
    pub position: Position,
    pub children: Vec<AstNodeEnum>,
    pub alias: Option<Symbol>,
    pub version: Option<String>,
    pub offset: Option<HashMap<String, usize>>,
}

/// Op meta section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpMeta {
    pub position: Position,
    pub children: Vec<AttrDef>,
    pub offset: Option<HashMap<String, usize>>,
}

/// Op input section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpInput {
    pub position: Position,
    pub children: Vec<AstNodeEnum>,
    pub offset: Option<HashMap<String, usize>>,
}

/// Op output section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpOutput {
    pub position: Position,
    pub children: Vec<AstNodeEnum>,
    pub offset: Option<HashMap<String, usize>>,
}

/// Op config section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpConfig {
    pub position: Position,
    pub children: Vec<AstNodeEnum>,
    pub offset: Option<HashMap<String, usize>>,
}

/// Op spec definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpSpec {
    pub position: Position,
    pub name: Symbol,
    pub items: Option<Vec<OpSpecItem>>,
}

/// Op spec item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpSpecItem {
    pub position: Position,
    pub name: String,
    pub value: Box<AstNodeEnum>,
}

/// Interval types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosedInterval {
    pub position: Position,
    pub ge: Option<NumberLiteral>,
    pub le: Option<NumberLiteral>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MixInterval {
    pub position: Position,
    pub ge: Option<NumberLiteral>,
    pub gt: Option<NumberLiteral>,
    pub le: Option<NumberLiteral>,
    pub lt: Option<NumberLiteral>,
}

macro_rules! define_ast_enum {
    (
        $(#[$enum_meta:meta])*
        $enum_vis:vis enum $enum_name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident($type:ident)
            ),*
            $(,)?
        }
    ) => {
        // 1. 生成 enum 定义
        $(#[$enum_meta])*
        $enum_vis enum $enum_name {
            $(
                $(#[$variant_meta])*
                $variant($type),
            )*
        }

        // 2. 为每个具体类型实现 AstNode
        $(
            impl AstNode for $type {
                fn position(&self) -> &Position {
                    &self.position
                }

                fn position_mut(&mut self) -> &mut Position {
                    &mut self.position
                }
            }
        )*

        // 3. 为 enum 实现 AstNode
        impl AstNode for $enum_name {
            fn position(&self) -> &Position {
                match self {
                    $(
                        $enum_name::$variant(node) => node.position(),
                    )*
                }
            }

            fn position_mut(&mut self) -> &mut Position {
                match self {
                    $(
                        $enum_name::$variant(node) => node.position_mut(),
                    )*
                }
            }
        }

        // 4. 可选：生成 From 实现，方便转换
        $(
            impl From<$type> for $enum_name {
                fn from(value: $type) -> Self {
                    $enum_name::$variant(value)
                }
            }
        )*
    };
}

define_ast_enum! {
/// Enum containing all possible AST node types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNodeEnum {
    Module(Module),
    Comment(Comment),
    Symbol(Symbol),
    StringLiteral(StringLiteral),
    MultiLineStringLiteral(MultiLineStringLiteral),
    NumberLiteral(NumberLiteral),
    FloatLiteral(FloatLiteral),
    BoolLiteral(BoolLiteral),
    DateTimeLiteral(DateTimeLiteral),
    DateLiteral(DateLiteral),
    NullLiteral(NullLiteral),
    DictStatement(DictStatement),
    DictItem(DictItem),
    ListStatement(ListStatement),
    TupleStatement(TupleStatement),
    SetStatement(SetStatement),
    Import(Import),
    ImportItem(ImportItem),
    AttrDef(AttrDef),
    RefDef(RefDef),
    VarDef(VarDef),
    GraphDef(GraphDef),
    NodeDef(NodeDef),
    NodeBlock(NodeBlock),
    NodeInputTuple(NodeInputTuple),
    NodeInputKeyDef(NodeInputKeyDef),
    NodeInputKeyItem(NodeInputKeyItem),
    NodeInputValues(NodeInputValues),
    NodeAttr(NodeAttr),
    ConditionDef(ConditionDef),
    ConditionBlock(ConditionBlock),
    ConditionStatement(ConditionStatement),
    ForLoopBlock(ForLoopBlock),
    OpDef(OpDef),
    OpMeta(OpMeta),
    OpInput(OpInput),
    OpOutput(OpOutput),
    OpConfig(OpConfig),
    OpSpec(OpSpec),
    OpSpecItem(OpSpecItem),
    ClosedInterval(ClosedInterval),
    MixInterval(MixInterval),
}
}
