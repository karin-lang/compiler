use volt::tree::*;
use crate::hir::*;

pub struct TreeAnalysis;

impl TreeAnalysis {
    pub fn analyze(&mut self, node: &SyntaxNode) -> Hir {
        let mut items = Vec::new();

        for each_node in node.children.filter_nodes() {
            items.push(self.item(each_node));
        }

        Hir {
            items,
        }
    }

    pub fn accessibility(&mut self, node: &SyntaxNode) -> HirAccessibility {
        if let Some(content) = node.children.get_leaf_or_none(0) {
            match content.value.as_str() {
                "pub" => HirAccessibility::Public,
                "pub@hako" => HirAccessibility::PublicInHako,
                _ => unreachable!("unknown accessibility"),
            }
        } else {
            HirAccessibility::Private
        }
    }

    pub fn item(&mut self, node: &SyntaxNode) -> HirItem {
        let content = node.children.get_node(0);

        match content.name.as_str() {
            "Function::function" => HirItem::Function(self.function(content)),
            _ => unreachable!("unknown item content name"),
        }
    }

    pub fn function(&mut self, node: &SyntaxNode) -> HirFunction {
        let name = self.identifier(&node.children.find_node("name"));
        let accessibility = self.accessibility(&node.children.find_node("Main::accessibility"));

        HirFunction {
            name,
            accessibility,
        }
    }

    pub fn identifier(&mut self, node: &SyntaxNode) -> String {
        node.children.get_leaf(0).value.clone()
    }
}
