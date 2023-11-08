pub mod hirify;
pub mod ir;
pub mod type_check;

use self::ir::{item::*, path::*};

#[derive(Clone, Debug, PartialEq)]
pub struct Hir {
    pub path_tree: HirPathTree,
    pub items: Vec<HirPathIndexBinding<HirItem>>,
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

    pub fn identifier(&self) -> &HirIdentifier {
        &self.0
    }

    pub fn value(&self) -> &T {
        &self.1
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
