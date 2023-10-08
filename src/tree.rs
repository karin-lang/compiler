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
        let arguments = node.children.find_node("args").children.filter_nodes().iter()
            .map(|v| self.formal_argument(v)).collect();
        let expressions = node.children.find_node("exprs").children.filter_nodes().iter()
            .map(|v| self.expression(v)).collect();
        HirFunction { name, accessibility, arguments, expressions }
    }

    pub fn formal_argument(&mut self, node: &SyntaxNode) -> HirFormalArgument {
        let name = self.identifier(node.children.find_node("Identifier::identifier"));
        let data_type = self.data_type(node.children.find_node("DataType::data_type"));
        HirFormalArgument { name, data_type }
    }

    pub fn expression(&mut self, node: &SyntaxNode) -> HirExpression {
        let content_node = node.children.get_node(0);

        match content_node.name.as_str() {
            "Literal::literal" => HirExpression::Literal(self.literal(content_node)),
            "DataType::data_type" => HirExpression::DataType(self.data_type(content_node)),
            _ => unreachable!("unknown expression"),
        }
    }

    pub fn literal(&mut self, node: &SyntaxNode) -> HirLiteral {
        let content = node.children.get_node(0);

        match content.name.as_str() {
            "Literal::boolean" => match content.children.get_leaf(0).value.as_str() {
                "true" => HirLiteral::Boolean(true),
                "false" => HirLiteral::Boolean(false),
                _ => unreachable!("unknown boolean value"),
            },
            "Literal::number" => {
                let float_number_node = content.children.get_node(0);

                if float_number_node.name == "Literal::float_number" {
                    let data_type = match float_number_node.children.find_node_or_none("DataType::primitive_number") {
                        Some(v) => Some(self.primitive_data_type(v)),
                        None => None,
                    };

                    let integer = float_number_node.children.find_node("integer").children.get_leaf(0).value.clone();
                    let float = float_number_node.children.find_node("float").children.get_leaf(0).value.clone();
                    let value = format!("{}.{}", integer, float);

                    HirLiteral::Float(HirFloatLiteral { data_type, value })
                } else {
                    let data_type = match content.children.find_node_or_none("DataType::primitive_number") {
                        Some(v) => Some(self.primitive_data_type(v)),
                        None => None,
                    };

                    let value_content = content.children.find_node("value").children.get_node(0);

                    let base = match value_content.name.as_str() {
                        "Literal::binary_number" => HirIntegerBase::Binary,
                        "Literal::octal_number" => HirIntegerBase::Octal,
                        "Literal::decimal_number" => HirIntegerBase::Decimal,
                        "Literal::hexadecimal_number" => HirIntegerBase::Hexadecimal,
                        _ => unreachable!("unknown integer base"),
                    };

                    let value = value_content.children.get_leaf(0).value.clone();

                    let exponent = match content.children.find_node_or_none("Literal::number_exponent") {
                        Some(exponent_node) => {
                            let positive = match exponent_node.children.get_leaf(0).value.as_str() {
                                "+" => true,
                                "-" => false,
                                _ => unreachable!("unknown positivity"),
                            };
                            let value = exponent_node.children.find_node("value").children.get_leaf(0).value.clone();
                            Some(HirIntegerExponent { positive, value })
                        },
                        None => None,
                    };

                    HirLiteral::Integer(HirIntegerLiteral { data_type, base, value, exponent })
                }
            },
            _ => unreachable!("unknown literal"),
        }
    }

    pub fn identifier(&mut self, node: &SyntaxNode) -> String {
        node.children.get_leaf(0).value.clone()
    }

    pub fn data_type(&mut self, node: &SyntaxNode) -> HirDataType {
        let content = node.children.get_node(0);

        match content.name.as_str() {
            "DataType::primitive" => HirDataType::Primitive(self.primitive_data_type(content)),
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

    // Also available to DataType::primitive_number rule.
    pub fn primitive_data_type(&mut self, node: &SyntaxNode) -> HirPrimitiveDataType {
        match node.children.get_leaf(0).value.as_str() {
            "usize" => HirPrimitiveDataType::Usize,
            "f32" => HirPrimitiveDataType::F32,
            _ => unreachable!("unknown primitive data type"),
        }
    }
}
