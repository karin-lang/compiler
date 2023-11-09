use super::expr::*;
use crate::hir::*;

#[derive(Clone, Debug, PartialEq)]
pub enum HirItem {
    Function(HirFunction),
    Struct,
    Enum,
    Trait,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirFunction {
    pub accessibility: HirAccessibility,
    pub return_type: HirDataType,
    pub arguments: Vec<HirIdentifierBinding<HirFormalArgument>>,
    pub expressions: Vec<HirExpression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirFormalArgument {
    pub mutability: HirMutability,
    pub data_type: HirDataType,
}
