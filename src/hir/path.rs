use std::collections::BTreeMap;

use super::expr::HirExpression;

// fix
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HirPathIndex(usize);

impl From<usize> for HirPathIndex {
    fn from(value: usize) -> Self {
        HirPathIndex(value)
    }
}

impl From<HirPathIndex> for usize {
    fn from(value: HirPathIndex) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirPathIndexGenerator(usize);

impl HirPathIndexGenerator {
    pub fn new() -> HirPathIndexGenerator {
        HirPathIndexGenerator(0)
    }

    pub fn generate(&mut self) -> HirPathIndex {
        let index = self.0;
        self.0 += 1;
        HirPathIndex::from(index)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirPathTree {
    pub(crate) hako_indexes: Vec<HirPathIndex>,
    pub(crate) nodes: BTreeMap<HirPathIndex, HirPathNode>,
}

impl HirPathTree {
    pub fn new() -> HirPathTree {
        HirPathTree {
            hako_indexes: Vec::new(),
            nodes: BTreeMap::new(),
        }
    }

    // Path index will be auto-generated when None specified.
    pub fn add_node(&mut self, index_generator: &mut HirPathIndexGenerator, index: Option<HirPathIndex>, node: HirPathNode, is_hako: bool) -> HirPathIndex {
        let node_index = if let Some(v) = index {
            v
        } else {
            index_generator.generate()
        };

        if is_hako {
            self.hako_indexes.push(node_index);
        }

        self.nodes.insert(node_index, node);
        node_index
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirPathNode {
    pub id: String,
    pub kind: HirPathKind,
    pub parent: Option<HirPathIndex>,
    pub children: Vec<HirPathIndex>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirPathKind {
    Hako,
    Module,
    Function,
    Struct,
    Enum,
    Trait,
}

// todo: 活用する?
#[derive(Clone, Debug, PartialEq)]
pub enum HirPath {
    Resolved(HirPathIndex),
    Unresolved(Vec<HirExpression>),
}
