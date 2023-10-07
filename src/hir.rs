#[derive(Clone, Debug, PartialEq)]
pub struct Hir {
    pub items: Vec<HirItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirItem {
    Function(HirFunction),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Accessibility {
    Private,
    Public,
    InHako,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirFunction {
    pub name: String,
    // pub accessibility: Accessibility,
}
