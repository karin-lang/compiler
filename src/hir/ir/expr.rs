use crate::hir::*;

#[derive(Clone, Debug, PartialEq)]
pub enum HirExpression {
    Operation(Box<HirOperation>),
    Literal(HirLiteral),
    Identifier(HirIdentifier),
    DataType(HirDataType),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirLiteral {
    Boolean(bool),
    Integer(HirIntegerLiteral),
    Float(HirFloatLiteral),
    String(String),
    SelfValue,
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirIntegerBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirIntegerLiteral {
    pub data_type: Option<HirPrimitiveDataType>,
    pub base: HirIntegerBase,
    pub value: String,
    pub exponent: Option<HirIntegerExponent>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirIntegerExponent {
    pub positive: bool,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirFloatLiteral {
    pub data_type: Option<HirPrimitiveDataType>,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirOperation {
    Substitute(HirExpression, HirExpression),
    Add(HirExpression, HirExpression),
    Subtract(HirExpression, HirExpression),
    Multiply(HirExpression, HirExpression),
    Not(HirExpression),
    BitNot(HirExpression),
    Negative(HirExpression),
    Nonnize(HirExpression),
    Propagate(HirExpression),
    FunctionCall(HirExpression, Vec<HirExpression>),
    MemberAccess(HirExpression, HirExpression),
    Path(HirPath),
    Group(HirExpression),
}

pub type HirOperationSequence = Vec<HirOperationToken>;

#[derive(Clone, Debug, PartialEq)]
pub enum HirOperationToken {
    Operator(HirOperator),
    Term(HirExpression),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirOperator {
    Substitute,
    Add,
    Subtract,
    Multiply,
    Not,
    BitNot,
    Negative,
    Nonnize,
    Propagate,
    FunctionCall(Vec<HirExpression>),
    MemberAccess,
    Path,
    GroupBegin,
    GroupEnd,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirDataType {
    Primitive(HirPrimitiveDataType),
    Generic(HirIdentifierBinding<HirGenericDataType>),
    Identifier(HirIdentifier),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirPrimitiveDataType {
    Boolean,
    S8,
    S16,
    S32,
    S64,
    Ssize,
    U8,
    U16,
    U32,
    U64,
    Usize,
    F32,
    F64,
    Character,
    String,
    SelfType,
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirGenericDataType {
    pub arguments: Vec<HirDataType>,
}
