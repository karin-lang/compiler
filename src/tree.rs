use volt::tree::*;
use crate::hir::{*, expr::*, item::*, path::*};

#[derive(Clone, Debug, PartialEq)]
pub struct AstHako<'a> {
    pub id: String,
    pub modules: Vec<AstModule<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstModule<'a> {
    pub id: String,
    pub node: &'a SyntaxNode,
    pub submodules: Vec<AstModule<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TreeAnalysis {
    path_index_generator: HirPathIndexGenerator,
    pub(crate) path_tree: HirPathTree,
    pub(crate) items: Vec<HirItem>,
}

impl TreeAnalysis {
    pub fn new() -> TreeAnalysis {
        TreeAnalysis {
            path_index_generator: HirPathIndexGenerator::new(),
            path_tree: HirPathTree::new(),
            items: Vec::new(),
        }
    }

    pub fn analyze(hakos: Vec<&AstHako>) -> Hir {
        let mut analyzer = TreeAnalysis::new();

        for each_hako in &hakos {
            analyzer.hako(each_hako);
        }

        Hir {
            path_tree: analyzer.path_tree,
            items: analyzer.items,
        }
    }

    pub fn hako(&mut self, hako: &AstHako) {
        let path_index = self.path_index_generator.generate();
        let mut child_items = Vec::new();
        let mut child_path_indexes = Vec::new();

        for each_module in &hako.modules {
            for each_item_node in &each_module.node.children.get_node(0).children.filter_nodes() {
                child_items.push(self.item(each_item_node));
            }

            child_path_indexes.push(self.module(each_module, path_index));
        }

        let path_node = HirPathNode {
            id: hako.id.clone(),
            kind: HirPathKind::Hako,
            parent: None,
            children: child_path_indexes,
        };

        self.path_tree.add_node(&mut self.path_index_generator, Some(path_index), path_node, true);
    }

    pub fn module(&mut self, module: &AstModule, parent: HirPathIndex) -> HirPathIndex {
        let path_index = self.path_index_generator.generate();
        let child_path_indexes = module.submodules.iter().map(|v| self.module(v, path_index)).collect();

        let path_node = HirPathNode {
            id: module.id.clone(),
            kind: HirPathKind::Module,
            parent: Some(parent),
            children: child_path_indexes,
        };

        self.path_tree.add_node(&mut self.path_index_generator, Some(path_index), path_node, false);
        path_index
    }

    pub fn identifier(&mut self, node: &SyntaxNode) -> String {
        node.children.get_leaf(0).value.clone()
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

    pub fn item(&mut self, node: &SyntaxNode) -> (String, HirItem) {
        let content = node.children.get_node(0);

        match content.name.as_str() {
            "Function::function" => {
                let (id, function) = self.function(content);
                (id, HirItem::Function(function))
            },
            _ => unreachable!("unknown item content name"),
        }
    }

    pub fn function(&mut self, node: &SyntaxNode) -> (String, HirFunction) {
        let id = self.identifier(&node.children.find_node("Identifier::identifier"));
        let accessibility = self.accessibility(node.children.find_node("Main::accessibility"));

        let arguments = node.children.find_node("args").children.filter_nodes().iter()
            .map(|v| self.formal_argument(v)).collect();

        let expressions = node.children.find_node("exprs").children.filter_nodes().iter()
            .map(|v| self.expression(v)).collect();

        (id, HirFunction { accessibility, arguments, expressions })
    }

    pub fn formal_argument(&mut self, node: &SyntaxNode) -> HirIdentifierBinding<HirFormalArgument> {
        let id = self.identifier(node.children.find_node("Identifier::identifier"));
        let data_type = self.data_type(node.children.find_node("DataType::data_type"));

        HirIdentifierBinding::new(
            id.into(),
            HirFormalArgument {
                // todo: 構文ノードのmutabilityを反映させる
                mutability: HirMutability::Immutable,
                data_type,
            },
        )
    }

    pub fn expression(&mut self, node: &SyntaxNode) -> HirExpression {
        let content_node = node.children.get_node(0);

        match content_node.name.as_str() {
            "Literal::literal" => HirExpression::Literal(self.literal(content_node)),
            "Operation::operation" => HirExpression::Operation(Box::new(self.operation(content_node))),
            "DataType::data_type" => HirExpression::DataType(self.data_type(content_node)),
            "Identifier::identifier" => HirExpression::Identifier(self.identifier(content_node)),
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

    pub fn operation(&mut self, node: &SyntaxNode) -> HirOperation {
        let elements = &node.children;

        let mut get_term = |elements: &Vec<SyntaxChild>, index: usize| -> HirExpression {
            let target = elements.get_node(index);

            if target.name == "Expression::operation_term" {
                self.expression(target)
            } else {
                HirExpression::Operation(Box::new(self.operation(target)))
            }
        };

        // prefix and grouping operators
        if let Some(first_leaf) = elements.get_leaf_or_none(0) {
            // todo: add more operators
            match first_leaf.value.as_str() {
                "(" => return HirOperation::Group(get_term(elements, 1)),
                _ => (),
            }
        }

        // infix operators
        if let Some(second_leaf) = elements.get_leaf_or_none(1) {
            match second_leaf.value.as_str() {
                "+" => return HirOperation::Addition(
                    get_term(elements, 0),
                    get_term(elements, 2),
                ),
                "*" => return HirOperation::Multiplication(
                    get_term(elements, 0),
                    get_term(elements, 2),
                ),
                "." => return HirOperation::MemberAccess(
                    get_term(elements, 0),
                    get_term(elements, 2),
                ),
                "::" => {
                    let left = get_term(elements, 0);
                    let right = get_term(elements, 2);

                    // todo: reject expression grouping
                    match left {
                        left @ (HirExpression::DataType(_) | HirExpression::Identifier(_)) => {
                            let mut segments = vec![left];

                            match &right {
                                HirExpression::Operation(operation) => match &**operation {
                                    HirOperation::Path(path) => match &*path {
                                        HirPath::Resolved(_) => unreachable!(),
                                        HirPath::Unresolved(right_segments) => segments.append(&mut right_segments.clone()),
                                    },
                                    _ => segments.push(right),
                                },
                                _ => segments.push(right),
                            }

                            return HirOperation::Path(HirPath::Unresolved(segments));
                        },
                        _ => unimplemented!("error handling is not implemented"),
                    }
                },
                _ => (),
            }
        }

        unreachable!("unknown operator");
    }

    pub fn data_type(&mut self, node: &SyntaxNode) -> HirDataType {
        let content = node.children.get_node(0);

        match content.name.as_str() {
            "DataType::primitive" => HirDataType::Primitive(self.primitive_data_type(content)),
            "DataType::generic" => {
                let id = content.children.find_node("Identifier::identifier").children.get_leaf(0).value.clone();
                let argument_nodes = &content.children.find_node("args").children.filter_nodes();
                let arguments = argument_nodes.iter().map(|data_type_node| {
                    match data_type_node.name.as_str() {
                        // fix: replace identifier to path
                        "Identifier::identifier" => HirDataType::Identifier(data_type_node.children.get_leaf(0).value.clone().into()),
                        "DataType::data_type" => self.data_type(data_type_node),
                        _ => unreachable!("unknown argument format in generic data type"),
                    }
                }).collect();
                HirDataType::Generic(HirIdentifierBinding::new(id.into(), HirGenericDataType { arguments }))
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
