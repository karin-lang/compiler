use crate::hir::ir::{path::*, item::*, expr::*};

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
            // todo: 文字列中の埋め込みリテラルが追加された際に型検査を通す
            HirExpression::Literal(_) => (),
            HirExpression::Operation(operation) => match &mut **operation {
                HirOperation::Substitute(left, right) => {
                    self.expression(left);
                    self.expression(right);
                },
                HirOperation::Add(left, right) => {
                    self.expression(left);
                    self.expression(right);
                },
                HirOperation::Subtract(left, right) => {
                    self.expression(left);
                    self.expression(right);
                },
                HirOperation::Multiply(left, right) => {
                    self.expression(left);
                    self.expression(right);
                },
                HirOperation::Not(term) => self.expression(term),
                HirOperation::BitNot(term) => self.expression(term),
                HirOperation::Negative(term) => self.expression(term),
                HirOperation::Nonnize(term) => self.expression(term),
                HirOperation::Propagate(term) => self.expression(term),
                HirOperation::FunctionCall(term, arguments) => {
                    self.expression(term);

                    for each_argument in arguments {
                        self.expression(each_argument);
                    }
                },
                HirOperation::MemberAccess(left, right) => {
                    self.expression(left);
                    self.expression(right);
                },
                HirOperation::Path(path) => self.path(path),
                HirOperation::Group(term) => self.expression(term),
            },
            HirExpression::DataType(_) => unimplemented!(),
            HirExpression::Identifier(_) => unimplemented!(),
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
