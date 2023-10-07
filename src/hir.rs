#[derive(Clone, Debug, PartialEq)]
pub struct Hir {
    pub items: Vec<HirItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirItem {
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
    pub name: String,
    pub accessibility: HirAccessibility,
}
