use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum HirExpression {
    Literal(HirLiteral),
    Operation(Box<HirOperation>),
    DataType(HirDataType),
    Identifier(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirLiteral {
    Boolean(bool),
    Integer(HirIntegerLiteral),
    Float(HirFloatLiteral),
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
    Addition(HirExpression, HirExpression),
    Multiplication(HirExpression, HirExpression),
    Negative(HirExpression),
    Not(HirExpression),
    BitNot(HirExpression),
    MemberAccess(HirExpression, HirExpression),
    Path(HirPath),
    Group(HirExpression),
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
