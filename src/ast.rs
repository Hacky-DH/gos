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

    pub fn new_all(line: usize, end_line:usize, start: usize, end: usize) -> Self {
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

    pub fn set(&mut self, line: usize, end_line:usize, start: usize, end: usize) {
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

impl AstNode for Module {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Comment node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
    pub position: Position,
    pub value: String,
}

impl AstNode for Comment {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Symbol - represents identifiers with kind information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol {
    pub position: Position,
    pub name: String,
    pub kind: SymbolKind,
}

impl AstNode for Symbol {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
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

/// Literal value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralValue {
    String(StringLiteral),
    MultiLineString(MultiLineStringLiteral),
    Number(NumberLiteral),
    Float(FloatLiteral),
    Bool(BoolLiteral),
    DateTime(DateTimeLiteral),
    Date(DateLiteral),
    Null(NullLiteral),
}

/// String literal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StringLiteral {
    pub position: Position,
    pub value: String,
}

impl AstNode for StringLiteral {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Multi-line string literal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiLineStringLiteral {
    pub position: Position,
    pub value: String,
}

impl AstNode for MultiLineStringLiteral {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Number literal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NumberLiteral {
    pub position: Position,
    pub raw: String,
    pub value: i64,
}

impl AstNode for NumberLiteral {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Float literal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatLiteral {
    pub position: Position,
    pub raw: String,
    pub value: f64,
}

impl AstNode for FloatLiteral {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Boolean literal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoolLiteral {
    pub position: Position,
    pub raw: String,
    pub value: bool,
}

impl AstNode for BoolLiteral {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// DateTime literal (deprecated)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateTimeLiteral {
    pub position: Position,
    pub raw: String,
    pub value: DateTime<Utc>,
}

impl AstNode for DateTimeLiteral {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Date literal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateLiteral {
    pub position: Position,
    pub value: String,
}

impl AstNode for DateLiteral {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Null literal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NullLiteral {
    pub position: Position,
}

impl AstNode for NullLiteral {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Collection types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollectionValue {
    Dict(DictStatement),
    List(ListStatement),
    Tuple(TupleStatement),
    Set(SetStatement),
}

/// Dictionary statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictStatement {
    pub position: Position,
    pub items: Vec<DictItem>,
}

impl AstNode for DictStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Dictionary item (key-value pair)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictItem {
    pub position: Position,
    pub key: Box<AstNodeEnum>,
    pub value: Box<AstNodeEnum>,
}

impl AstNode for DictItem {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// List statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListStatement {
    pub position: Position,
    pub items: Vec<AstNodeEnum>,
}

impl AstNode for ListStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Tuple statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupleStatement {
    pub position: Position,
    pub items: Vec<AstNodeEnum>,
}

impl AstNode for TupleStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Set statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStatement {
    pub position: Position,
    pub items: Vec<AstNodeEnum>,
}

impl AstNode for SetStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Import statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Import {
    pub position: Position,
    pub items: Vec<ImportItem>,
}

impl AstNode for Import {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Import item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportItem {
    pub position: Position,
    pub path: Symbol,
    pub alias: Option<Symbol>,
}

impl AstNode for ImportItem {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
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

impl AstNode for AttrDef {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
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

impl AstNode for RefDef {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Variable definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VarDef {
    pub position: Position,
    pub children: Vec<AstNodeEnum>,
    pub alias: Option<Symbol>,
    pub offset: Option<HashMap<String, usize>>,
}

impl AstNode for VarDef {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
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

impl AstNode for GraphDef {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Node definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeDef {
    pub position: Position,
    pub outputs: Vec<Symbol>,
    pub value: NodeBlock,
}

impl AstNode for NodeDef {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Node block (function call with attributes)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeBlock {
    pub position: Position,
    pub name_or_ref: Symbol,
    pub inputs: Option<NodeInputDef>,
    pub attrs: Option<Vec<NodeAttr>>,
}

impl AstNode for NodeBlock {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
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
    pub items: Vec<Symbol>,
}

impl AstNode for NodeInputTuple {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Node input key-value definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInputKeyDef {
    pub position: Position,
    pub items: Vec<NodeInputKeyItem>,
}

impl AstNode for NodeInputKeyDef {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Node input key-value item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInputKeyItem {
    pub position: Position,
    pub key: Symbol,
    pub value: NodeInputValues,
}

impl AstNode for NodeInputKeyItem {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Node input values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInputValues {
    pub position: Position,
    pub items: Vec<Symbol>,
}

impl AstNode for NodeInputValues {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Node attribute
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeAttr {
    pub position: Position,
    pub name: Symbol,
    pub value: NodeAttrValue,
    pub offset: Option<HashMap<String, usize>>,
}

impl AstNode for NodeAttr {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
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

impl AstNode for ConditionDef {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Condition block
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionBlock {
    pub position: Position,
    pub condition: Box<ConditionExpr>,
    pub true_branch: Box<AstNodeEnum>,
    pub false_branch: Box<AstNodeEnum>,
}

impl AstNode for ConditionBlock {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
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

impl AstNode for ConditionStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
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

impl AstNode for ForLoopBlock {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
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

impl AstNode for OpDef {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Op meta section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpMeta {
    pub position: Position,
    pub children: Vec<AttrDef>,
    pub offset: Option<HashMap<String, usize>>,
}

impl AstNode for OpMeta {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Op input section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpInput {
    pub position: Position,
    pub children: Vec<AstNodeEnum>,
    pub offset: Option<HashMap<String, usize>>,
}

impl AstNode for OpInput {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Op output section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpOutput {
    pub position: Position,
    pub children: Vec<AstNodeEnum>,
    pub offset: Option<HashMap<String, usize>>,
}

impl AstNode for OpOutput {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Op config section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpConfig {
    pub position: Position,
    pub children: Vec<AstNodeEnum>,
    pub offset: Option<HashMap<String, usize>>,
}

impl AstNode for OpConfig {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Op spec definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpSpec {
    pub position: Position,
    pub name: Symbol,
    pub items: Option<Vec<OpSpecItem>>,
}

impl AstNode for OpSpec {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Op spec item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpSpecItem {
    pub position: Position,
    pub name: String,
    pub value: Box<AstNodeEnum>,
}

impl AstNode for OpSpecItem {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

/// Interval types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosedInterval {
    pub position: Position,
    pub ge: Option<NumberLiteral>,
    pub le: Option<NumberLiteral>,
}

impl AstNode for ClosedInterval {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MixInterval {
    pub position: Position,
    pub ge: Option<NumberLiteral>,
    pub gt: Option<NumberLiteral>,
    pub le: Option<NumberLiteral>,
    pub lt: Option<NumberLiteral>,
}

impl AstNode for MixInterval {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

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

impl AstNode for AstNodeEnum {
    fn position(&self) -> &Position {
        match self {
            AstNodeEnum::Module(n) => n.position(),
            AstNodeEnum::Comment(n) => n.position(),
            AstNodeEnum::Symbol(n) => n.position(),
            AstNodeEnum::StringLiteral(n) => n.position(),
            AstNodeEnum::MultiLineStringLiteral(n) => n.position(),
            AstNodeEnum::NumberLiteral(n) => n.position(),
            AstNodeEnum::FloatLiteral(n) => n.position(),
            AstNodeEnum::BoolLiteral(n) => n.position(),
            AstNodeEnum::DateTimeLiteral(n) => n.position(),
            AstNodeEnum::DateLiteral(n) => n.position(),
            AstNodeEnum::NullLiteral(n) => n.position(),
            AstNodeEnum::DictStatement(n) => n.position(),
            AstNodeEnum::DictItem(n) => n.position(),
            AstNodeEnum::ListStatement(n) => n.position(),
            AstNodeEnum::TupleStatement(n) => n.position(),
            AstNodeEnum::SetStatement(n) => n.position(),
            AstNodeEnum::Import(n) => n.position(),
            AstNodeEnum::ImportItem(n) => n.position(),
            AstNodeEnum::AttrDef(n) => n.position(),
            AstNodeEnum::RefDef(n) => n.position(),
            AstNodeEnum::VarDef(n) => n.position(),
            AstNodeEnum::GraphDef(n) => n.position(),
            AstNodeEnum::NodeDef(n) => n.position(),
            AstNodeEnum::NodeBlock(n) => n.position(),
            AstNodeEnum::NodeInputTuple(n) => n.position(),
            AstNodeEnum::NodeInputKeyDef(n) => n.position(),
            AstNodeEnum::NodeInputKeyItem(n) => n.position(),
            AstNodeEnum::NodeInputValues(n) => n.position(),
            AstNodeEnum::NodeAttr(n) => n.position(),
            AstNodeEnum::ConditionDef(n) => n.position(),
            AstNodeEnum::ConditionBlock(n) => n.position(),
            AstNodeEnum::ConditionStatement(n) => n.position(),
            AstNodeEnum::ForLoopBlock(n) => n.position(),
            AstNodeEnum::OpDef(n) => n.position(),
            AstNodeEnum::OpMeta(n) => n.position(),
            AstNodeEnum::OpInput(n) => n.position(),
            AstNodeEnum::OpOutput(n) => n.position(),
            AstNodeEnum::OpConfig(n) => n.position(),
            AstNodeEnum::OpSpec(n) => n.position(),
            AstNodeEnum::OpSpecItem(n) => n.position(),
            AstNodeEnum::ClosedInterval(n) => n.position(),
            AstNodeEnum::MixInterval(n) => n.position(),
        }
    }

    fn position_mut(&mut self) -> &mut Position {
        match self {
            AstNodeEnum::Module(n) => n.position_mut(),
            AstNodeEnum::Comment(n) => n.position_mut(),
            AstNodeEnum::Symbol(n) => n.position_mut(),
            AstNodeEnum::StringLiteral(n) => n.position_mut(),
            AstNodeEnum::MultiLineStringLiteral(n) => n.position_mut(),
            AstNodeEnum::NumberLiteral(n) => n.position_mut(),
            AstNodeEnum::FloatLiteral(n) => n.position_mut(),
            AstNodeEnum::BoolLiteral(n) => n.position_mut(),
            AstNodeEnum::DateTimeLiteral(n) => n.position_mut(),
            AstNodeEnum::DateLiteral(n) => n.position_mut(),
            AstNodeEnum::NullLiteral(n) => n.position_mut(),
            AstNodeEnum::DictStatement(n) => n.position_mut(),
            AstNodeEnum::DictItem(n) => n.position_mut(),
            AstNodeEnum::ListStatement(n) => n.position_mut(),
            AstNodeEnum::TupleStatement(n) => n.position_mut(),
            AstNodeEnum::SetStatement(n) => n.position_mut(),
            AstNodeEnum::Import(n) => n.position_mut(),
            AstNodeEnum::ImportItem(n) => n.position_mut(),
            AstNodeEnum::AttrDef(n) => n.position_mut(),
            AstNodeEnum::RefDef(n) => n.position_mut(),
            AstNodeEnum::VarDef(n) => n.position_mut(),
            AstNodeEnum::GraphDef(n) => n.position_mut(),
            AstNodeEnum::NodeDef(n) => n.position_mut(),
            AstNodeEnum::NodeBlock(n) => n.position_mut(),
            AstNodeEnum::NodeInputTuple(n) => n.position_mut(),
            AstNodeEnum::NodeInputKeyDef(n) => n.position_mut(),
            AstNodeEnum::NodeInputKeyItem(n) => n.position_mut(),
            AstNodeEnum::NodeInputValues(n) => n.position_mut(),
            AstNodeEnum::NodeAttr(n) => n.position_mut(),
            AstNodeEnum::ConditionDef(n) => n.position_mut(),
            AstNodeEnum::ConditionBlock(n) => n.position_mut(),
            AstNodeEnum::ConditionStatement(n) => n.position_mut(),
            AstNodeEnum::ForLoopBlock(n) => n.position_mut(),
            AstNodeEnum::OpDef(n) => n.position_mut(),
            AstNodeEnum::OpMeta(n) => n.position_mut(),
            AstNodeEnum::OpInput(n) => n.position_mut(),
            AstNodeEnum::OpOutput(n) => n.position_mut(),
            AstNodeEnum::OpConfig(n) => n.position_mut(),
            AstNodeEnum::OpSpec(n) => n.position_mut(),
            AstNodeEnum::OpSpecItem(n) => n.position_mut(),
            AstNodeEnum::ClosedInterval(n) => n.position_mut(),
            AstNodeEnum::MixInterval(n) => n.position_mut(),
        }
    }
}