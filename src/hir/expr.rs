use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum HirExpression {
    Literal(HirLiteral),
    Operation(Box<HirOperation>),
    DataType(HirDataType),
    Identifier(HirIdentifier),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirLiteral {
    Boolean(bool),
    Integer(HirIntegerLiteral),
    Float(HirFloatLiteral),
    String(String),
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
    Multiply(HirExpression, HirExpression),
    Path(HirPath),
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
    Negative,
    Not,
    BitNot,
    Nonnize,
    Propagate,
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
    Usize,
    F32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirGenericDataType {
    pub arguments: Vec<HirDataType>,
}
