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
        let name = self.identifier(&node.children.find_node("Identifier::identifier"));
        let accessibility = self.accessibility(node.children.find_node("Main::accessibility"));
        let arguments = node.children.find_node("args").children.filter_nodes().iter().map(|v| self.formal_argument(v)).collect();
        HirFunction { name, accessibility, arguments }
    }

    pub fn formal_argument(&mut self, node: &SyntaxNode) -> HirFormalArgument {
        let name = self.identifier(node.children.find_node("Identifier::identifier"));
        let data_type = self.data_type(node.children.find_node("DataType::data_type"));
        HirFormalArgument { name, data_type }
    }

    pub fn identifier(&mut self, node: &SyntaxNode) -> String {
        node.children.get_leaf(0).value.clone()
    }

    pub fn data_type(&mut self, node: &SyntaxNode) -> HirDataType {
        let content = node.children.get_node(0);

        match content.name.as_str() {
            "DataType::primitive" => {
                let primitive = match content.children.get_leaf(0).value.as_str() {
                    "usize" => HirPrimitiveDataType::Usize,
                    "f32" => HirPrimitiveDataType::F32,
                    _ => unreachable!("unknown primitive data type"),
                };

                HirDataType::Primitive(primitive)
            },
            "DataType::generic" => {
                let name = content.children.find_node("Identifier::identifier").children.get_leaf(0).value.clone();
                let argument_nodes = &content.children.find_node("args").children.filter_nodes();
                let arguments = argument_nodes.iter().map(|data_type_node| {
                    match data_type_node.name.as_str() {
                        "Identifier::identifier" => HirDataType::Identifier(HirUnresolvedIdentifier(data_type_node.children.get_leaf(0).value.clone())),
                        "DataType::data_type" => self.data_type(data_type_node),
                        _ => unreachable!("unknown argument format in generic data type"),
                    }
                }).collect();
                HirDataType::Generic(HirGenericDataType { name, arguments })
            },
            _ => unreachable!("unknown data type"),
        }
    }
}
