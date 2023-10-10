use crate::_hir::*;
use super::ir::*;

pub struct JsCodeGenerator;

impl JsCodeGenerator {
    pub fn generate(hir: Hir) {
        let mut generator = JsCodeGenerator;
        hir.hakos.iter().map(|v| generator.hako(v));
    }

    pub fn hako(&mut self, hako: HirHako) -> 

    pub fn item(&mut self, path: &HirPath, item: &HirPathItem) -> JsItem {
        match item {
            HirPathItem::Function(function) => {
                JsItem::Function(
                    JsFunction {
                        id: path.last().clone(),
                        arguments: function.arguments.iter().map(|v| v.id().clone()).collect(),
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
            _ => unimplemented!(),
        }
    }

    pub fn literal(&mut self, literal: &HirLiteral) -> JsLiteral {
        match literal {
            HirLiteral::Boolean(boolean) => JsLiteral::Boolean(*boolean),
            _ => unimplemented!(),
        }
    }
}
