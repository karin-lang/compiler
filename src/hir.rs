use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HirPath(Vec<String>);

impl Default for HirPath {
    fn default() -> Self {
        HirPath(Vec::new())
    }
}

impl HirPath {
    pub fn new(value: Vec<String>) -> HirPath {
        HirPath(value)
    }

    pub fn append_clone(&self, path: String) -> HirPath {
        let mut cloned = self.0.clone();
        cloned.push(path);
        HirPath(cloned)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HirIdentifierBinding<T>(String, T);

impl<T> HirIdentifierBinding<T> {
    pub fn new(id: String, data: T) -> HirIdentifierBinding<T> {
        HirIdentifierBinding(id, data)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hir {
    pub hakos: HashMap<String, HirHako>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirHako {
    pub items: HashMap<HirPath, HirPathItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirPathItem {
    Module,
    Function(HirFunction),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirAccessibility {
    Private,
    Public,
    PublicInHako,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirFunction {
    pub accessibility: HirAccessibility,
    pub arguments: Vec<HirIdentifierBinding<HirFormalArgument>>,
    pub expressions: Vec<HirExpression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirFormalArgument {
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
    Generic(HirIdentifierBinding<HirGenericDataType>),
    Identifier(HirPath),
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
