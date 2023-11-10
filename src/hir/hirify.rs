use volt::tree::*;
use crate::ast::operator::OperationParser;
use super::*;
use super::ir::{expr::*, item::*, path::*};

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
pub enum TreeHirifierLog {
    Error(TreeHirifierError),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TreeHirifierError {
    PathSegmentMustLocateFirstPosition { path_segment: String },
    SelfArgumentMustLocateFirstPosition,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ItemHirifierResult {
    ItemPathIndex(HirPathIndex),
    UseDeclaration(HirPath),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TreeHirifier {
    path_index_generator: HirPathIndexGenerator,
    pub(crate) path_tree: HirPathTree,
    pub(crate) items: Vec<HirPathIndexBinding<HirItem>>,
    pub(crate) logs: Vec<TreeHirifierLog>,
}

impl TreeHirifier {
    pub fn new() -> TreeHirifier {
        TreeHirifier {
            path_index_generator: HirPathIndexGenerator::new(),
            path_tree: HirPathTree::new(),
            items: Vec::new(),
            logs: Vec::new(),
        }
    }

    pub fn hirify(hakos: Vec<&AstHako>) -> (Hir, Vec<TreeHirifierLog>) {
        let mut analyzer = TreeHirifier::new();

        for each_hako in &hakos {
            analyzer.hako(each_hako);
        }

        (
            Hir {
                path_tree: analyzer.path_tree,
                items: analyzer.items,
            },
            analyzer.logs,
        )
    }

    pub fn hako(&mut self, hako: &AstHako) {
        let path_index = self.path_index_generator.generate();
        let mut child_path_indexes = Vec::new();

        for each_module in &hako.modules {
            let module_path_index = self.module(each_module, path_index);
            child_path_indexes.push(module_path_index);
        }

        let path_node = HirPathNode {
            id: hako.id.clone().into(),
            kind: HirPathKind::Hako,
            parent: None,
            children: child_path_indexes,
        };

        self.path_tree.add_node(&mut self.path_index_generator, Some(path_index), path_node);
    }

    pub fn module(&mut self, module: &AstModule, parent: HirPathIndex) -> HirPathIndex {
        let path_index = self.path_index_generator.generate();
        let mut children = Vec::new();
        let mut use_declarations = Vec::new();

        let mut submodules: Vec<HirPathIndex> = module.submodules.iter().map(|v| self.module(v, path_index)).collect();
        children.append(&mut submodules);

        for each_subitem_node in module.node.children.filter_nodes() {
            match self.item(each_subitem_node, path_index) {
                ItemHirifierResult::ItemPathIndex(path_index) => children.push(path_index),
                ItemHirifierResult::UseDeclaration(path) => use_declarations.push(path),
            }
        }

        let path_node = HirPathNode {
            id: module.id.clone().into(),
            kind: HirPathKind::Module { use_declarations },
            parent: Some(parent),
            children,
        };

        self.path_tree.add_node(&mut self.path_index_generator, Some(path_index), path_node);
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

    pub fn item(&mut self, node: &SyntaxNode, parent: HirPathIndex) -> ItemHirifierResult {
        let path_index = self.path_index_generator.generate();
        let content = node.children.get_node(0);

        let (path_node, item) = match content.name.as_str() {
            "UseDeclaration::use_declaration" => return ItemHirifierResult::UseDeclaration(self.use_declaration(content)),
            "Function::function" => {
                let (id, function) = self.function(content);

                let path_node = HirPathNode {
                    id: id.clone().into(),
                    kind: HirPathKind::Function,
                    parent: Some(parent),
                    children: Vec::new(),
                };

                (path_node, HirItem::Function(function))
            },
            _ => unreachable!("unknown item content name"),
        };

        self.path_tree.add_node(&mut self.path_index_generator, Some(path_index), path_node);
        self.items.push(HirPathIndexBinding::new(path_index, item));

        ItemHirifierResult::ItemPathIndex(path_index)
    }

    pub fn use_declaration(&mut self, node: &SyntaxNode) -> HirPath {
        let mut segments = Vec::new();

        for (index, each_identifier) in node.children.filter_leaves().iter().enumerate() {
            let new_segment = each_identifier.value.clone();

            if index != 0 {
                match new_segment.as_str() {
                    "hako" | "self" => self.logs.push(
                        TreeHirifierLog::Error(TreeHirifierError::PathSegmentMustLocateFirstPosition { path_segment: new_segment.to_string() }),
                    ),
                    _ => (),
                }
            }

            segments.push(new_segment.into());
        }

        HirPath::Unresolved(segments)
    }

    pub fn function(&mut self, node: &SyntaxNode) -> (String, HirFunction) {
        let id = self.identifier(&node.children.find_node("Identifier::identifier"));
        let accessibility = self.accessibility(node.children.find_node("Main::accessibility"));

        let return_type = match node.children.find_node_or_none("DataType::data_type") {
            Some(v) => self.data_type(v),
            None => HirDataType::Primitive(HirPrimitiveDataType::None),
        };

        let arguments = node.children.find_node("args").children.filter_nodes().iter().enumerate()
            .map(|(i, v)| self.formal_argument(i, v)).collect();

        let expressions = node.children.find_node("exprs").children.filter_nodes().iter()
            .map(|v| self.expression(v)).collect();

        (id, HirFunction { accessibility, return_type, arguments, expressions })
    }

    pub fn formal_argument(&mut self, index: usize, node: &SyntaxNode) -> HirIdentifierBinding<HirFormalArgument> {
        let (id, data_type) = if let Some(id_node) = node.children.find_node_or_none("Identifier::identifier") {
            (self.identifier(id_node), self.data_type(node.children.find_node("DataType::data_type")))
        } else if node.children.has_leaf("self") {
            if index != 0 {
                self.logs.push(TreeHirifierLog::Error(TreeHirifierError::SelfArgumentMustLocateFirstPosition));
            }

            ("self".to_string(), HirDataType::Primitive(HirPrimitiveDataType::SelfType))
        } else {
            unreachable!("formal argument must have an identifier or self keyword");
        };

        let mutability = if node.children.has_leaf("mut") {
            HirMutability::Mutable
        } else {
            HirMutability::Immutable
        };

        HirIdentifierBinding::new(id.into(), HirFormalArgument { mutability, data_type })
    }

    pub fn expression(&mut self, node: &SyntaxNode) -> HirExpression {
        let content_node = node.children.get_node(0);

        match content_node.name.as_str() {
            "Operation::operation" => self.operation(content_node),
            "Literal::literal" => HirExpression::Literal(self.literal(content_node)),
            "Identifier::identifier" => HirExpression::Identifier(self.identifier(content_node).into()),
            "DataType::data_type" => HirExpression::DataType(self.data_type(content_node)),
            _ => unreachable!("unknown expression"),
        }
    }

    pub fn operation(&mut self, node: &SyntaxNode) -> HirExpression {
        let tokens = node.children.iter().map(|each_child| self.operation_token(each_child.into_node())).collect();

        match OperationParser::parse(tokens) {
            Ok(v) => v,
            Err(e) => unimplemented!("{:?}", e),
        }
    }

    pub fn operation_token(&mut self, node: &SyntaxNode) -> HirOperationToken {
        match node.name.as_ref() {
            "operator" => HirOperationToken::Operator(self.operator(node)),
            _ => HirOperationToken::Term(self.expression(node))
        }
    }

    pub fn operator(&mut self, node: &SyntaxNode) -> HirOperator {
        if let Some(operator_leaf) = node.children.get_leaf_or_none(0) {
            match operator_leaf.value.as_ref() {
                "=" => HirOperator::Substitute,
                "+" => HirOperator::Add,
                "-" => HirOperator::Subtract,
                "*" => HirOperator::Multiply,
                "!e" => HirOperator::Not,
                "~e" => HirOperator::BitNot,
                "-e" => HirOperator::Negative,
                "e!" => HirOperator::Nonnize,
                "e?" => HirOperator::Propagate,
                "." => HirOperator::MemberAccess,
                "::" => HirOperator::Path,
                "(" => HirOperator::GroupBegin,
                ")" => HirOperator::GroupEnd,
                _ => todo!("add more operators"),
            }
        } else {
            let operator_node = node.children.get_node(0);

            match operator_node.name.as_str() {
                "Operation::function_call_operator" => {
                    let arguments = operator_node.children.filter_nodes().iter().map(|v| self.expression(v)).collect();
                    HirOperator::FunctionCall(arguments)
                },
                _ => unreachable!("unknown format of operator node"),
            }
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
                    let data_type = match float_number_node.children.find_node_or_none("data_type_suffix") {
                        Some(v) => Some(self.primitive_data_type(v)),
                        None => None,
                    };

                    let integer = float_number_node.children.find_node("integer").children.get_leaf(0).value.clone();
                    let float = float_number_node.children.find_node("float").children.get_leaf(0).value.clone();
                    let value = format!("{}.{}", integer, float);

                    HirLiteral::Float(HirFloatLiteral { data_type, value })
                } else {
                    let data_type = match content.children.find_node_or_none("data_type_suffix") {
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
            "self" => HirLiteral::SelfValue,
            "none" => HirLiteral::None,
            _ => unreachable!("unknown literal"),
        }
    }

    pub fn data_type(&mut self, node: &SyntaxNode) -> HirDataType {
        let content = node.children.get_node(0);

        match content.name.as_str() {
            "DataType::primitive" => HirDataType::Primitive(self.primitive_data_type(content)),
            "DataType::generic" => {
                let id = content.children.find_node("Identifier::identifier").children.get_leaf(0).value.clone();
                let arguments = self.generic_arguments(&content.children.find_node("DataType::generic_arguments"));
                HirDataType::Generic(HirIdentifierBinding::new(id.into(), HirGenericDataType { arguments }))
            },
            _ => unreachable!("unknown data type"),
        }
    }

    pub fn primitive_data_type(&mut self, node: &SyntaxNode) -> HirPrimitiveDataType {
        match node.children.get_leaf(0).value.as_str() {
            "bool" => HirPrimitiveDataType::Boolean,
            "s8" => HirPrimitiveDataType::S8,
            "s16" => HirPrimitiveDataType::S16,
            "s32" => HirPrimitiveDataType::S32,
            "s64" => HirPrimitiveDataType::S64,
            "ssize" => HirPrimitiveDataType::Ssize,
            "u8" => HirPrimitiveDataType::U8,
            "u16" => HirPrimitiveDataType::U16,
            "u32" => HirPrimitiveDataType::U32,
            "u64" => HirPrimitiveDataType::U64,
            "usize" => HirPrimitiveDataType::Usize,
            "f32" => HirPrimitiveDataType::F32,
            "f64" => HirPrimitiveDataType::F64,
            "char" => HirPrimitiveDataType::Character,
            "str" => HirPrimitiveDataType::String,
            "Self" => HirPrimitiveDataType::SelfType,
            "none" => HirPrimitiveDataType::None,
            _ => unreachable!("unknown primitive data type"),
        }
    }

    pub fn generic_arguments(&mut self, node: &SyntaxNode) -> Vec<HirDataType> {
        node.children.filter_nodes().iter().map(|data_type_node| {
            match data_type_node.name.as_str() {
                // fix: replace identifier to path
                "Identifier::identifier" => HirDataType::Identifier(data_type_node.children.get_leaf(0).value.clone().into()),
                "DataType::data_type" => self.data_type(data_type_node),
                _ => unreachable!("unknown argument format in generic data type"),
            }
        }).collect()
    }
}
