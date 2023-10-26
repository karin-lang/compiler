pub mod expr;
pub mod item;
pub mod path;
pub mod type_check;

use self::{item::*, path::*};

#[derive(Clone, Debug, PartialEq)]
pub struct Hir {
    pub path_tree: HirPathTree,
    pub items: Vec<HirItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirIdentifier(String);

impl From<&str> for HirIdentifier {
    fn from(value: &str) -> Self {
        HirIdentifier(value.to_string())
    }
}

impl From<String> for HirIdentifier {
    fn from(value: String) -> Self {
        HirIdentifier(value)
    }
}

impl From<HirIdentifier> for String {
    fn from(value: HirIdentifier) -> Self {
        value.0.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirIdentifierBinding<T>(HirIdentifier, T);

impl<T> HirIdentifierBinding<T> {
    pub fn new(id: HirIdentifier, value: T) -> HirIdentifierBinding<T> {
        HirIdentifierBinding(id, value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirAccessibility {
    Private,
    Public,
    PublicInHako,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirMutability {
    Immutable,
    Mutable,
}
