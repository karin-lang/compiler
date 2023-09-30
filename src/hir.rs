#[derive(Clone, Debug, PartialEq)]
pub struct Hir {
    pub items: Vec<HirItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirItem {
    Function,
}
