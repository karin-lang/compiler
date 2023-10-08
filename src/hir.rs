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
