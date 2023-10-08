#[derive(Clone, Debug, PartialEq)]
pub struct Hir {
    pub items: Vec<HirItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirItem {
    Function(HirFunction),
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirUnresolvedIdentifier(pub String);

#[derive(Clone, Debug, PartialEq)]
pub enum HirAccessibility {
    Private,
    Public,
    PublicInHako,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirFunction {
    pub name: String,
    pub accessibility: HirAccessibility,
    pub arguments: Vec<HirFormalArgument>,
    pub expressions: Vec<HirExpression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirFormalArgument {
    pub name: String,
    pub data_type: HirDataType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirExpression {
    Literal(HirLiteral),
    DataType(HirDataType),
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
pub enum HirDataType {
    Primitive(HirPrimitiveDataType),
    Generic(HirGenericDataType),
    Identifier(HirUnresolvedIdentifier),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirPrimitiveDataType {
    Usize,
    F32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirGenericDataType {
    pub name: String,
    pub arguments: Vec<HirDataType>,
}
