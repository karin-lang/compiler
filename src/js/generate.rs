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
                        id: format!("i_{}", path_index),
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
            HirExpression::Operation(operation) => JsStatement::Expression(self.operation(operation)),
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

    pub fn operation(&mut self, operation: &HirOperation) -> JsExpression {
        // todo: convert statement to expression
        let js_operation = match operation {
            HirOperation::Substitute(_, _) => unimplemented!(),
            HirOperation::Add(left, right) => JsOperation::Add(self.statement(left).into(), self.statement(right).into()),
            HirOperation::Subtract(left, right) => JsOperation::Subtract(self.statement(left).into(), self.statement(right).into()),
            HirOperation::Multiply(left, right) => JsOperation::Multiply(self.statement(left).into(), self.statement(right).into()),
            HirOperation::Not(term) => JsOperation::Not(self.statement(term).into()),
            HirOperation::BitNot(term) => JsOperation::BitNot(self.statement(term).into()),
            HirOperation::Negative(term) => JsOperation::Negative(self.statement(term).into()),
            HirOperation::Nonnize(_) => unimplemented!(),
            HirOperation::Propagate(_) => unimplemented!(),
            HirOperation::FunctionCall(term, arguments) => JsOperation::FunctionCall(
                self.statement(term).into(),
                arguments.iter().map(|v| self.statement(v).into()).collect(),
            ),
            HirOperation::MemberAccess(left, right) => JsOperation::MemberAccess(self.statement(left).into(), self.statement(right).into()),
            HirOperation::Path(path) => return JsExpression::Identifier(self.path(path)),
            HirOperation::Group(term) => JsOperation::Group(self.statement(term).into()),
        };

        JsExpression::Operation(Box::new(js_operation))
    }

    pub fn path(&mut self, path: &HirPath) -> String {
        match path {
            HirPath::Resolved(index) => format!("i_{index}"),
            HirPath::Unresolved(_) => unreachable!("path not resolved"),
        }
    }
}
