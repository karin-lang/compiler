use std::collections::HashMap;
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
        let mut items = HashMap::new();

        for each_item in &hir.items {
            items.insert(format!("{}", each_item.index()), generator.item(each_item));
        }

        Js { items }
    }

    pub fn item(&mut self, item: &HirPathIndexBinding<HirItem>) -> JsItem {
        let path_index = item.index();
        let path_node = self.path_tree.get(path_index).expect(&format!("failed to resolve identifier (path index {})", path_index));

        match item.value() {
            HirItem::Function(function) => {
                JsItem::Function(
                    JsFunction {
                        id: path_node.id.clone().into(),
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
