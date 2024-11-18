use crate::syntax::operator::*;
use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Span {
    /// Start offset in source
    pub start: usize,

    /// End offset in source
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Program<'a> {
    pub span: Span,
    pub source_text: &'a str,
    pub body: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Statement<'a> {
    BlockStatement(Box<BlockStatement<'a>>),
    IfStatement(Box<IfStatement<'a>>),
    ExpressionStatement(Box<ExpressionStatement<'a>>),
    EmptyStatement(Box<EmptyStatement>),
    ReturnStatement(Box<ReturnStatement<'a>>),
    ForStatement(Box<ForStatement<'a>>),
    WhileStatement(Box<WhileStatement<'a>>),
    BreakStatement(Box<BreakStatement>),
    ContinueStatement(Box<ContinueStatement>),
    DoWhileStatement(Box<DoWhileStatement<'a>>),
    SwitchStatement(Box<SwitchStatement<'a>>),
    WithStatement(Box<WithStatement<'a>>),
    VariableDeclarationStatement(Box<VariableDeclaration<'a>>),
    FunctionDeclaration(Box<Function<'a>>),
}

#[derive(Debug, Clone, Serialize)]
pub struct ReturnStatement<'a> {
    pub span: Span,
    pub argument: Option<Expression<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DoWhileStatement<'a> {
    pub span: Span,
    pub body: Statement<'a>,
    pub test: Expression<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WhileStatement<'a> {
    pub span: Span,
    pub test: Expression<'a>,
    pub body: Statement<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ForStatement<'a> {
    pub span: Span,
    pub init: Option<ForStatementInit<'a>>,
    pub test: Option<Expression<'a>>,
    pub update: Option<Expression<'a>>,
    pub body: Statement<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub enum ForStatementInit<'a> {
    VariableDeclaration(Box<VariableDeclaration<'a>>),
    Expression(Expression<'a>),
}

#[derive(Debug, Clone, Serialize)]
pub enum ForStatementLeft<'a> {
    VariableDeclaration(Box<VariableDeclaration<'a>>),
    Expression(Expression<'a>),
}

#[derive(Debug, Clone, Serialize)]
pub struct ContinueStatement {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct BreakStatement {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct WithStatement<'a> {
    pub span: Span,
    pub object: Expression<'a>,
    pub body: Statement<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SwitchStatement<'a> {
    pub span: Span,
    pub discriminant: Expression<'a>,
    pub cases: Vec<SwitchCase<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SwitchCase<'a> {
    pub span: Span,
    pub test: Option<Expression<'a>>,
    pub consequent: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmptyStatement {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExpressionStatement<'a> {
    pub span: Span,
    pub expression: Expression<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BlockStatement<'a> {
    pub span: Span,
    pub body: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IfStatement<'a> {
    pub span: Span,
    pub test: Expression<'a>,
    pub consequent: Statement<'a>,
    pub alternate: Option<Statement<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VariableDeclaration<'a> {
    pub span: Span,
    pub kind: VariableDeclarationKind,
    pub id: Identifier<'a>,
    pub init: Option<Expression<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

#[derive(Debug, Clone, Serialize)]
pub struct Function<'a> {
    pub span: Span,
    pub id: Option<Identifier<'a>>,
    pub params: Box<FormalParameters<'a>>,
    pub body: Option<Box<FunctionBody<'a>>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FormalParameters<'a> {
    pub span: Span,
    pub params: Vec<Identifier<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionBody<'a> {
    pub span: Span,
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Expression<'a> {
    BooleanLiteral(Box<BooleanLiteral>),
    NullLiteral(Box<NullLiteral>),
    NumericLiteral(Box<NumericLiteral<'a>>),
    StringLiteral(Box<StringLiteral<'a>>),
    Identifier(Box<Identifier<'a>>),
    SequenceExpression(Box<SequenceExpression<'a>>),
    BinaryExpression(Box<BinaryExpression<'a>>),
    UnaryExpression(Box<UnaryExpression<'a>>),
    LogicalExpression(Box<LogicalExpression<'a>>),
    AssignmentExpression(Box<AssignmentExpression<'a>>),
    ArrayExpression(Box<ArrayExpression<'a>>),
    ObjectExpression(Box<ObjectExpression<'a>>),
    StaticMemberExpression(Box<StaticMemberExpression<'a>>),
    ComputedMemberExpression(Box<ComputedMemberExpression<'a>>),
    CallExpression(Box<CallExpression<'a>>),
    ParenthesizedExpression(Box<ParenthesizedExpression<'a>>),
}

#[derive(Debug, Clone, Serialize)]
pub struct ArrayExpression<'a> {
    pub span: Span,
    pub elements: Vec<ArrayExpressionElement<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub enum ArrayExpressionElement<'a> {
    Elision(Elision),
    Expression(Expression<'a>),
}

#[derive(Debug, Clone, Serialize)]
pub struct Elision {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct ObjectExpression<'a> {
    pub span: Span,
    /// Properties declared in the object
    pub properties: Vec<ObjectProperty<'a>>,
    pub trailing_comma: Option<Span>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ObjectProperty<'a> {
    pub span: Span,
    pub key: PropertyKey<'a>,
    pub value: Expression<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub enum PropertyKey<'a> {
    IdentifierName(IdentifierName<'a>),
    StringLiteral(StringLiteral<'a>),
    NumericLiteral(NumericLiteral<'a>),
}

#[derive(Debug, Clone, Serialize)]
pub struct IdentifierName<'a> {
    pub span: Span,
    pub name: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct BinaryExpression<'a> {
    pub span: Span,
    pub left: Expression<'a>,
    pub operator: BinaryOperator,
    pub right: Expression<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnaryExpression<'a> {
    pub span: Span,
    pub operator: UnaryOperator,
    pub argument: Expression<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StaticMemberExpression<'a> {
    pub span: Span,
    pub object: Expression<'a>,
    pub property: IdentifierName<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ComputedMemberExpression<'a> {
    pub span: Span,
    pub object: Expression<'a>,
    pub expression: Expression<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CallExpression<'a> {
    pub span: Span,
    pub callee: Expression<'a>,
    pub arguments: Vec<Expression<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssignmentExpression<'a> {
    pub span: Span,
    pub operator: AssignmentOperator,
    pub left: AssignmentTarget<'a>,
    pub right: Expression<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub enum AssignmentTarget<'a> {
    Identifier(Box<Identifier<'a>>),
    StaticMemberExpression(Box<StaticMemberExpression<'a>>),
    ComputedMemberExpression(Box<ComputedMemberExpression<'a>>),
}

#[derive(Debug, Clone, Serialize)]
pub struct LogicalExpression<'a> {
    pub span: Span,
    pub left: Expression<'a>,
    pub operator: LogicalOperator,
    pub right: Expression<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ParenthesizedExpression<'a> {
    pub span: Span,
    pub expression: Expression<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BooleanLiteral {
    pub span: Span,
    pub value: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct NullLiteral {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct NumericLiteral<'a> {
    pub span: Span,
    pub value: f64,
    pub raw: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct StringLiteral<'a> {
    pub span: Span,
    pub value: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct Identifier<'a> {
    pub span: Span,
    pub name: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct SequenceExpression<'a> {
    pub span: Span,
    pub expressions: Vec<Expression<'a>>,
}
