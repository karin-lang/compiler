use crate::hir::{path::*, item::*, expr::*};

#[derive(Clone, Debug, PartialEq)]
pub enum DataTypeError {
    UnknownIdentifier,
}

#[derive(Debug)]
pub struct DataTypeChecker<'a> {
    pub(crate) path_tree: &'a HirPathTree,
    pub(crate) errors: Vec<DataTypeError>,
}

impl<'a> DataTypeChecker<'a> {
    pub(crate) fn new(path_tree: &'a HirPathTree) -> DataTypeChecker<'a> {
        DataTypeChecker::<'a> {
            path_tree,
            errors: Vec::new(),
        }
    }

    pub fn check(path_tree: &'a HirPathTree, items: &mut Vec<HirPathIndexBinding<HirItem>>) -> Vec<DataTypeError> {
        let mut checker = DataTypeChecker::new(path_tree);

        for each_item in items {
            checker.item(each_item.value_mut());
        }

        checker.errors
    }

    pub(crate) fn item(&mut self, item: &mut HirItem) {
        // todo: add PathBinding<HirItem>
        match item {
            HirItem::Function(function) => {
                for each_expr in &mut function.expressions {
                    self.expression(each_expr);
                }
            },
            _ => unimplemented!(),
        }
    }

    pub(crate) fn expression(&mut self, expr: &mut HirExpression) {
        match expr {
            HirExpression::Operation(operation) => match &mut **operation {
                HirOperation::Path(path) => self.path(path),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }

    pub(crate) fn path(&mut self, path: &mut HirPath) {
        let path_index = match path {
            HirPath::Resolved(_) => unreachable!("path is already resolved"),
            HirPath::Unresolved(segments) => {
                match self.path_tree.find(segments) {
                    Some((path_index, _)) => path_index,
                    None => {
                        self.errors.push(DataTypeError::UnknownIdentifier);
                        return;
                    },
                }
            },
        };

        let new_path = HirPath::Resolved(path_index.clone());
        let _ = std::mem::replace(path, new_path);
    }
}
