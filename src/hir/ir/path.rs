use std::fmt;
use std::collections::BTreeMap;
use crate::hir::HirIdentifier;

// fix
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HirPathIndex(usize);

impl fmt::Display for HirPathIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
pub struct HirPathIndexBinding<T>(HirPathIndex, T);

impl<T> HirPathIndexBinding<T> {
    pub fn new(index: HirPathIndex, value: T) -> HirPathIndexBinding<T> {
        HirPathIndexBinding(index, value)
    }

    pub fn index(&self) -> &HirPathIndex {
        &self.0
    }

    pub fn value(&self) -> &T {
        &self.1
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.1
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirPathIndexGenerator(usize);

impl HirPathIndexGenerator {
    pub fn new() -> HirPathIndexGenerator {
        HirPathIndexGenerator(0)
    }

    pub fn index(&self) -> Option<HirPathIndex> {
        if self.0 >= 1 {
            Some(HirPathIndex(self.0 - 1))
        } else {
            None
        }
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

    pub fn get(&self, index: &HirPathIndex) -> Option<&HirPathNode> {
        self.nodes.get(index)
    }

    // Path index will be auto-generated when None specified.
    pub fn add_node(&mut self, index_generator: &mut HirPathIndexGenerator, index: Option<HirPathIndex>, node: HirPathNode) -> HirPathIndex {
        let node_index = if let Some(v) = index {
            v
        } else {
            index_generator.generate()
        };

        if node.kind == HirPathKind::Hako {
            if node.parent.is_some() {
                panic!("hako path node can't have a parent");
            }

            self.hako_indexes.push(node_index);
        }

        self.nodes.insert(node_index, node);
        node_index
    }

    pub fn find<'a>(&'a self, path_segments: &'a Vec<HirPathSegment>) -> Option<(&'a HirPathIndex, &'a HirPathNode)> {
        let mut path_segment_iter = path_segments.iter();

        let hako_node_pair = if let Some(hako_segment) = path_segment_iter.next() {
            match self.find_hako(hako_segment) {
                Some(v) => v,
                None => return None,
            }
        } else {
            return None;
        };

        let mut current_path_node = hako_node_pair;

        while let Some(path_segment) = path_segment_iter.next() {
            match self.find_child(&current_path_node.1.children, path_segment) {
                Some(path_node_pair) => current_path_node = path_node_pair,
                None => return None,
            }
        }

        Some(current_path_node)
    }

    pub fn find_hako<'a>(&'a self, segment: &'a HirPathSegment) -> Option<(&'a HirPathIndex, &'a HirPathNode)> {
        self.find_child(&self.hako_indexes, segment)
    }

    pub(crate) fn find_child<'a>(&'a self, indexes: &'a Vec<HirPathIndex>, segment: &'a HirPathSegment) -> Option<(&'a HirPathIndex, &'a HirPathNode)> {
        for each_index in indexes {
            if let Some(path_node) = self.get(each_index) {
                if path_node.id == *segment {
                    return Some((each_index, path_node));
                }
            }
        }

        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirPathNode {
    pub id: HirIdentifier,
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

#[derive(Clone, Debug, PartialEq)]
pub enum HirPath {
    Resolved(HirPathIndex),
    Unresolved(Vec<HirPathSegment>),
}

pub type HirPathSegment = HirIdentifier;
