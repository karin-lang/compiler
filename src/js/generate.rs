use crate::hir::{*, expr::*, item::*, path::*};
use crate::js::ir::*;

pub struct JsGenerator<'a> {
    path_tree: &'a HirPathTree,
}

impl<'a> JsGenerator<'a> {
    pub(crate) fn new(path_tree: &'a HirPathTree) -> JsGenerator<'a> {
        JsGenerator { path_tree }
    }

    pub fn generate(hir: &'a Hir) -> Js {
        let mut generator = JsGenerator::new(&hir.path_tree);
        let items = hir.items.iter().map(|v| generator.item(v)).collect();
        Js { items }
    }

    pub fn item(&mut self, item: &HirPathIndexBinding<HirItem>) -> JsItem {
        let path_index = item.index();

        match item.value() {
            HirItem::Function(function) => {
                JsItem::Function(
                    JsFunction {
                        id: format!("f_{}", path_index),
                        arguments: function.arguments.iter().map(|v| v.identifier().clone().into()).collect(),
                        statements: function.expressions.iter().map(|v| self.statement(v)).collect(),
                    },
                )
            },
            _ => unimplemented!(),
        }
    }

    pub fn statement(&mut self, expr: &HirExpression) -> JsStatement {
        match expr {
            HirExpression::Literal(literal) => JsStatement::Expression(JsExpression::Literal(self.literal(literal))),
            HirExpression::Operation(operation) => JsStatement::Expression(JsExpression::Operation(Box::new(self.operation(operation)))),
            _ => unimplemented!(),
        }
    }

    pub fn literal(&mut self, literal: &HirLiteral) -> JsLiteral {
        match literal {
            HirLiteral::Boolean(boolean) => JsLiteral::Boolean(*boolean),
            HirLiteral::Integer(integer) => {
                // todo: support exponent and add test case
                JsLiteral::Integer(integer.value.clone())
            },
            _ => unimplemented!(),
        }
    }

    pub fn operation(&mut self, operation: &HirOperation) -> JsOperation {
        match operation {
            HirOperation::Add(left, right) => JsOperation::Add(self.statement(left).into(), self.statement(right).into()),
            HirOperation::Subtract(left, right) => JsOperation::Subtract(self.statement(left).into(), self.statement(right).into()),
            HirOperation::Multiply(left, right) => JsOperation::Multiply(self.statement(left).into(), self.statement(right).into()),
            HirOperation::Not(term) => JsOperation::Not(self.statement(term).into()),
            HirOperation::BitNot(term) => JsOperation::BitNot(self.statement(term).into()),
            HirOperation::Negative(term) => JsOperation::Negative(self.statement(term).into()),
            HirOperation::MemberAccess(left, right) => JsOperation::MemberAccess(self.statement(left).into(), self.statement(right).into()),
            HirOperation::Group(term) => JsOperation::Group(self.statement(term).into()),
            _ => unreachable!("cannot convert to js operation"),
        }
    }
}
