use std::collections::BTreeMap;
use crate::hir::*;
use crate::hir::hirify::*;
use crate::hir::ir::{expr::*, item::*, path::*};
use speculate::speculate;
use volt::{*, tree::*};

speculate!{
    before {
        #[allow(unused)]
        let new_analyzer = || TreeHirifier::new();

        #[allow(unused)]
        let get_operator = |operator: HirOperator|
            HirOperationToken::Operator(operator);

        #[allow(unused)]
        let get_integer_expression = |value: usize|
            HirExpression::Literal(
                HirLiteral::Integer(
                    HirIntegerLiteral {
                        data_type: None,
                        base: HirIntegerBase::Decimal,
                        value: value.to_string(),
                        exponent: None,
                    },
                ),
            );

        #[allow(unused)]
        let empty_tree = || tree!(node!("Main::main" => []));
    }

    describe "hirify" {
        it "generates child paths" {
            let (hir, _) = TreeHirifier::hirify(vec![
                &AstHako {
                    id: "h".to_string(),
                    modules: Vec::new(),
                },
            ]);

            assert_eq!(
                hir.path_tree,
                HirPathTree {
                    hako_indexes: vec![
                        HirPathIndex::from(0),
                    ],
                    nodes: BTreeMap::from([(
                        HirPathIndex::from(0),
                        HirPathNode {
                            id: "h".into(),
                            kind: HirPathKind::Hako,
                            parent: None,
                            children: Vec::new(),
                        },
                    )]),
                },
            );
        }

        it "generates submodule paths" {
            let (hir, _) = TreeHirifier::hirify(vec![
                &AstHako {
                    id: "h".to_string(),
                    modules: vec![
                        AstModule {
                            id: "m".to_string(),
                            node: &empty_tree().root,
                            submodules: Vec::new(),
                        },
                    ],
                },
            ]);

            assert_eq!(
                hir.path_tree,
                HirPathTree {
                    hako_indexes: vec![
                        HirPathIndex::from(0),
                    ],
                    nodes: BTreeMap::from([
                        (
                            HirPathIndex::from(0),
                            HirPathNode {
                                id: "h".into(),
                                kind: HirPathKind::Hako,
                                parent: None,
                                children: vec![1.into()],
                            },
                        ),
                        (
                            HirPathIndex::from(1),
                            HirPathNode {
                                id: "m".into(),
                                kind: HirPathKind::Module,
                                parent: Some(0.into()),
                                children: Vec::new(),
                            },
                        ),
                    ]),
                },
            );
        }

        it "generates subitems in submodule and their paths" {
            let syntax_child = node!("Main::main" => [
                node!("Item::item" => [
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ]),
                ]),
            ]);

            let (hir, _) = TreeHirifier::hirify(vec![
                &AstHako {
                    id: "h".to_string(),
                    modules: vec![
                        AstModule {
                            id: "m".to_string(),
                            node: syntax_child.into_node(),
                            submodules: Vec::new(),
                        },
                    ],
                },
            ]);

            assert_eq!(
                hir.path_tree,
                HirPathTree {
                    hako_indexes: vec![
                        HirPathIndex::from(0),
                    ],
                    nodes: BTreeMap::from([
                        (
                            HirPathIndex::from(0),
                            HirPathNode {
                                id: "h".into(),
                                kind: HirPathKind::Hako,
                                parent: None,
                                children: vec![1.into()],
                            },
                        ),
                        (
                            HirPathIndex::from(1),
                            HirPathNode {
                                id: "m".into(),
                                kind: HirPathKind::Module,
                                parent: Some(0.into()),
                                children: vec![2.into()],
                            },
                        ),
                        (
                            HirPathIndex::from(2),
                            HirPathNode {
                                id: "f".into(),
                                kind: HirPathKind::Function,
                                parent: Some(1.into()),
                                children: Vec::new(),
                            },
                        ),
                    ]),
                },
            );

            assert_eq!(hir.items, vec![
                HirPathIndexBinding::new(
                    2.into(),
                    HirItem::Function(
                        HirFunction {
                            accessibility: HirAccessibility::Private,
                            return_type: HirDataType::Primitive(HirPrimitiveDataType::None),
                            arguments: Vec::new(),
                            expressions: Vec::new(),
                        }
                    ),
                ),
            ]);
        }
    }

    describe "hako" {
        it "generates a hako path" {
            let mut analyzer = new_analyzer();

            analyzer.hako(
                &AstHako {
                    id: "h".to_string(),
                    modules: Vec::new(),
                },
            );

            assert_eq!(
                analyzer.path_tree,
                HirPathTree {
                    hako_indexes: vec![HirPathIndex::from(0)],
                    nodes: BTreeMap::from([(
                        HirPathIndex::from(0),
                        HirPathNode {
                            id: "h".into(),
                            kind: HirPathKind::Hako,
                            parent: None,
                            children: Vec::new(),
                        },
                    )]),
                },
            );
        }

        it "hirify submodules" {
            let mut analyzer = new_analyzer();

            analyzer.hako(
                &AstHako {
                    id: "h".to_string(),
                    modules: Vec::new(),
                },
            );

            assert_eq!(
                analyzer.path_tree,
                HirPathTree {
                    hako_indexes: vec![HirPathIndex::from(0)],
                    nodes: BTreeMap::from([(
                        HirPathIndex::from(0),
                        HirPathNode {
                            id: "h".into(),
                            kind: HirPathKind::Hako,
                            parent: None,
                            children: Vec::new(),
                        },
                    )]),
                },
            );
        }
    }

    describe "module" {
        it "inherit parent path and generates module path" {
            let mut analyzer = new_analyzer();

            let path_index = analyzer.module(
                &AstModule {
                    id: "m".to_string(),
                    node: &empty_tree().root,
                    submodules: Vec::new(),
                },
                HirPathIndex::from(100),
            );

            assert_eq!(path_index, 0.into());

            assert_eq!(
                analyzer.path_tree,
                HirPathTree {
                    hako_indexes: vec![],
                    nodes: BTreeMap::from([(
                        HirPathIndex::from(0),
                        HirPathNode {
                            id: "m".into(),
                            kind: HirPathKind::Module,
                            parent: Some(HirPathIndex::from(100)),
                            children: Vec::new(),
                        },
                    )]),
                },
            );
        }

        it "generates submodule path" {
            let mut analyzer = new_analyzer();

            analyzer.module(
                &AstModule {
                    id: "m".to_string(),
                    node: &empty_tree().root,
                    submodules: vec![
                        AstModule {
                            id: "sm".to_string(),
                            node: &empty_tree().root,
                            submodules: Vec::new(),
                        },
                    ],
                },
                HirPathIndex::from(100),
            );

            assert_eq!(
                analyzer.path_tree,
                HirPathTree {
                    hako_indexes: vec![],
                    nodes: BTreeMap::from([
                        (
                            HirPathIndex::from(0),
                            HirPathNode {
                                id: "m".into(),
                                kind: HirPathKind::Module,
                                parent: Some(HirPathIndex::from(100)),
                                children: vec![1.into()],
                            },
                        ),
                        (
                            HirPathIndex::from(1),
                            HirPathNode {
                                id: "sm".into(),
                                kind: HirPathKind::Module,
                                parent: Some(HirPathIndex::from(0)),
                                children: Vec::new(),
                            },
                        ),
                    ]),
                },
            );
        }

        it "generates subitem paths and their structure" {
            let syntax_child = node!("Main::main" => [
                node!("Item::item" => [
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [
                            leaf!("f"),
                        ]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ]),
                ]),
            ]);

            let mut analyzer = new_analyzer();

            analyzer.module(
                &AstModule {
                    id: "m".to_string(),
                    node: syntax_child.into_node(),
                    submodules: Vec::new(),
                },
                HirPathIndex::from(100),
            );

            assert_eq!(
                analyzer.path_tree,
                HirPathTree {
                    hako_indexes: vec![],
                    nodes: BTreeMap::from([
                        (
                            HirPathIndex::from(0),
                            HirPathNode {
                                id: "m".into(),
                                kind: HirPathKind::Module,
                                parent: Some(HirPathIndex::from(100)),
                                children: vec![1.into()],
                            },
                        ),
                        (
                            HirPathIndex::from(1),
                            HirPathNode {
                                id: "f".into(),
                                kind: HirPathKind::Function,
                                parent: Some(HirPathIndex::from(0)),
                                children: Vec::new(),
                            },
                        ),
                    ]),
                },
            );

            assert_eq!(
                analyzer.items,
                vec![
                    HirPathIndexBinding::new(
                        1.into(),
                        HirItem::Function(
                            HirFunction {
                                accessibility: HirAccessibility::Private,
                                return_type: HirDataType::Primitive(HirPrimitiveDataType::None),
                                arguments: Vec::new(),
                                expressions: Vec::new(),
                            },
                        ),
                    ),
                ],
            );
        }
    }

    describe "identifier" {
        it "generates identifier" {
            let syntax_child = node!("Identifier::identifier" => [leaf!("id")]);
            let mut analyzer = new_analyzer();
            assert_eq!(analyzer.identifier(syntax_child.into_node()), "id".to_string());
        }
    }

    describe "accessibility" {
        it "private" {
            assert_eq!(
                new_analyzer().accessibility(
                    node!("Main::accessibility" => []).into_node(),
                ),
                HirAccessibility::Private,
            );
        }

        it "public" {
            assert_eq!(
                new_analyzer().accessibility(
                    node!("Main::accessibility" => [leaf!("pub")]).into_node(),
                ),
                HirAccessibility::Public,
            );
        }

        it "public in hako" {
            assert_eq!(
                new_analyzer().accessibility(
                    node!("Main::accessibility" => [leaf!("pub@hako")]).into_node(),
                ),
                HirAccessibility::PublicInHako,
            );
        }
    }

    describe "item" {
        it "reflects path indexes for itself and parent" {
            let mut analyzer = new_analyzer();

            let path_index = analyzer.item(
                node!("Item::item" => [
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ]),
                ]).into_node(),
                100.into(),
            );

            assert_eq!(path_index, 0.into());

            assert_eq!(
                analyzer.path_tree,
                HirPathTree {
                    hako_indexes: vec![],
                    nodes: BTreeMap::from([(
                        HirPathIndex::from(0),
                        HirPathNode {
                            id: "f".into(),
                            kind: HirPathKind::Function,
                            parent: Some(HirPathIndex::from(100)),
                            children: Vec::new(),
                        },
                    )]),
                },
            );
        }

        it "hirifies function" {
            assert_eq!(
                new_analyzer().function(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ]).into_node(),
                ),
                (
                    "f".to_string(),
                    HirFunction {
                        accessibility: HirAccessibility::Private,
                        return_type: HirDataType::Primitive(HirPrimitiveDataType::None),
                        arguments: Vec::new(),
                        expressions: Vec::new(),
                    },
                ),
            );
        }
    }

    describe "function" {
        it "reflects accessibility, arguments and expressions" {
            assert_eq!(
                new_analyzer().function(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => [
                            node!("Function::formal_argument" => [
                                node!("Identifier::identifier" => [
                                    leaf!("a"),
                                ]),
                                node!("DataType::data_type" => [
                                    node!("DataType::primitive" => [
                                        leaf!("usize"),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("exprs" => [
                            node!("Expression::expression" => [
                                node!("Literal::literal" => [
                                    node!("Literal::boolean" => [leaf!("true")]),
                                ]),
                            ]),
                        ]),
                    ]).into_node(),
                ),
                (
                    "f".to_string(),
                    HirFunction {
                        accessibility: HirAccessibility::Private,
                        return_type: HirDataType::Primitive(HirPrimitiveDataType::None),
                        arguments: vec![
                            HirIdentifierBinding::new(
                                "a".into(),
                                HirFormalArgument {
                                    mutability: HirMutability::Immutable,
                                    data_type: HirDataType::Primitive(HirPrimitiveDataType::Usize),
                                },
                            ),
                        ],
                        expressions: vec![
                            HirExpression::Literal(HirLiteral::Boolean(true)),
                        ],
                    },
                ),
            );
        }

        it "has return type optionally" {
            assert_eq!(
                new_analyzer().function(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ]).into_node(),
                ),
                (
                    "f".to_string(),
                    HirFunction {
                        accessibility: HirAccessibility::Private,
                        return_type: HirDataType::Primitive(HirPrimitiveDataType::None),
                        arguments: Vec::new(),
                        expressions: Vec::new(),
                    },
                ),
            );

            assert_eq!(
                new_analyzer().function(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("DataType::data_type" => [
                            node!("DataType::primitive" => [leaf!("usize")]),
                        ]),
                        node!("exprs" => []),
                    ]).into_node(),
                ),
                (
                    "f".to_string(),
                    HirFunction {
                        accessibility: HirAccessibility::Private,
                        return_type: HirDataType::Primitive(HirPrimitiveDataType::Usize),
                        arguments: Vec::new(),
                        expressions: Vec::new(),
                    },
                ),
            );
        }

        describe "formal argument" {
            it "has name and data type" {
                assert_eq!(
                    new_analyzer().formal_argument(
                        0,
                        node!("Function::formal_argument" => [
                            node!("Identifier::identifier" => [leaf!("a")]),
                            node!("DataType::data_type" => [
                                node!("DataType::primitive" => [leaf!("usize")]),
                            ]),
                        ]).into_node(),
                    ),
                    HirIdentifierBinding::new(
                        "a".into(),
                        HirFormalArgument {
                            mutability: HirMutability::Immutable,
                            data_type: HirDataType::Primitive(HirPrimitiveDataType::Usize),
                        },
                    ),
                );
            }

            it "identifies argument as immutable by default" {
                assert_eq!(
                    new_analyzer().formal_argument(
                        0,
                        node!("Function::formal_argument" => [
                            node!("Identifier::identifier" => [leaf!("a")]),
                            node!("DataType::data_type" => [
                                node!("DataType::primitive" => [leaf!("usize")]),
                            ]),
                        ]).into_node(),
                    ),
                    HirIdentifierBinding::new(
                        "a".into(),
                        HirFormalArgument {
                            mutability: HirMutability::Immutable,
                            data_type: HirDataType::Primitive(HirPrimitiveDataType::Usize),
                        },
                    ),
                );

                assert_eq!(
                    new_analyzer().formal_argument(
                        0,
                        node!("Function::formal_argument" => [
                            leaf!("mut"),
                            node!("Identifier::identifier" => [leaf!("a")]),
                            node!("DataType::data_type" => [
                                node!("DataType::primitive" => [leaf!("usize")]),
                            ]),
                        ]).into_node(),
                    ),
                    HirIdentifierBinding::new(
                        "a".into(),
                        HirFormalArgument {
                            mutability: HirMutability::Mutable,
                            data_type: HirDataType::Primitive(HirPrimitiveDataType::Usize),
                        },
                    ),
                );
            }

            it "associates Self type with self argument" {
                assert_eq!(
                    new_analyzer().formal_argument(
                        0,
                        node!("Function::formal_argument" => [leaf!("self")]).into_node(),
                    ),
                    HirIdentifierBinding::new(
                        "self".into(),
                        HirFormalArgument {
                            mutability: HirMutability::Immutable,
                            data_type: HirDataType::Primitive(HirPrimitiveDataType::SelfType),
                        },
                    ),
                );
            }

            it "allows self argument in first position only" {
                let mut analyzer1 = new_analyzer();

                analyzer1.formal_argument(
                    1,
                    node!("Function::formal_argument" => [leaf!("self")]).into_node(),
                );

                assert_eq!(
                    analyzer1.logs,
                    vec![TreeHirifierLog::Error(TreeHirifierError::SelfArgumentMustLocateFirstPosition)],
                );
            }
        }
    }

    describe "expression" {
        it "hirifies operation" {
            assert_eq!(
                new_analyzer().expression(
                    node!("Expression::expression" => [
                        node!("Operation::operation" => [
                            node!("Expression::pure_expression" => [
                                node!("Literal::literal" => [
                                    node!("Literal::number" => [
                                        node!("value" => [
                                            node!("Literal::decimal_number" => [
                                                leaf!("0"),
                                            ]),
                                        ]),
                                    ]),
                                ]),
                            ]),
                            node!("operator" => [leaf!("+")]),
                            node!("Expression::pure_expression" => [
                                node!("Literal::literal" => [
                                    node!("Literal::number" => [
                                        node!("value" => [
                                            node!("Literal::decimal_number" => [
                                                leaf!("1"),
                                            ]),
                                        ]),
                                    ]),
                                ]),
                            ]),
                        ]),
                    ]).into_node(),
                ),
                HirExpression::Operation(
                    Box::new(
                        HirOperation::Add(
                            get_integer_expression(0),
                            get_integer_expression(1),
                        ),
                    ),
                ),
            );
        }

        it "hirifies literal" {
            assert_eq!(
                new_analyzer().expression(
                    node!("Expression::expression" => [
                        node!("Literal::literal" => [
                            node!("Literal::boolean" => [leaf!("true")]),
                        ]),
                    ]).into_node(),
                ),
                HirExpression::Literal(HirLiteral::Boolean(true)),
            );
        }

        it "hirifies identifier" {
            assert_eq!(
                new_analyzer().expression(
                    node!("Expression::expression" => [
                        node!("Identifier::identifier" => [leaf!("id")]),
                    ]).into_node(),
                ),
                HirExpression::Identifier("id".into()),
            );
        }

        it "hirifies data type" {
            assert_eq!(
                new_analyzer().expression(
                    node!("Expression::expression" => [
                        node!("DataType::data_type" => [
                            node!("DataType::primitive" => [
                                leaf!("usize"),
                            ]),
                        ]),
                    ]).into_node(),
                ),
                HirExpression::DataType(HirDataType::Primitive(HirPrimitiveDataType::Usize)),
            );
        }
    }

    describe "operation" {
        it "converts to postfix notation" {
            assert_eq!(
                new_analyzer().operation(
                    node!("Operation::operation" => [
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("0")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("operator" => [leaf!("+")]),
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("1")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                    ]).into_node(),
                ),
                HirExpression::Operation(
                    Box::new(
                        HirOperation::Add(
                            get_integer_expression(0),
                            get_integer_expression(1),
                        ),
                    ),
                ),
            );
        }

        describe "operator" {
            // todo: add operators

            describe "function call" {
                it "converts zero or more arguments" {
                    assert_eq!(
                        new_analyzer().operation(
                            node!("Operation::operation" => [
                                node!("Expression::pure_expression" => [
                                    node!("Literal::literal" => [
                                        node!("Literal::number" => [
                                            node!("value" => [
                                                node!("Literal::decimal_number" => [leaf!("0")]),
                                            ]),
                                        ]),
                                    ]),
                                ]),
                                node!("operator" => [
                                    node!("Operation::function_call_operator" => []),
                                ]),
                            ]).into_node(),
                        ),
                        HirExpression::Operation(
                            Box::new(
                                HirOperation::FunctionCall(
                                    get_integer_expression(0),
                                    Vec::new(),
                                ),
                            ),
                        ),
                    );

                    assert_eq!(
                        new_analyzer().operation(
                            node!("Operation::operation" => [
                                node!("Expression::pure_expression" => [
                                    node!("Literal::literal" => [
                                        node!("Literal::number" => [
                                            node!("value" => [
                                                node!("Literal::decimal_number" => [
                                                    leaf!("0"),
                                                ]),
                                            ]),
                                        ]),
                                    ]),
                                ]),
                                node!("operator" => [
                                    node!("Operation::function_call_operator" => [
                                        node!("Expression::expression" => [
                                            node!("Literal::literal" => [
                                                node!("Literal::number" => [
                                                    node!("value" => [
                                                        node!("Literal::decimal_number" => [
                                                            leaf!("1"),
                                                        ]),
                                                    ]),
                                                ]),
                                            ]),
                                        ]),
                                        node!("Expression::expression" => [
                                            node!("Literal::literal" => [
                                                node!("Literal::number" => [
                                                    node!("value" => [
                                                        node!("Literal::decimal_number" => [
                                                            leaf!("2"),
                                                        ]),
                                                    ]),
                                                ]),
                                            ]),
                                        ]),
                                    ]),
                                ]),
                            ]).into_node(),
                        ),
                        HirExpression::Operation(
                            Box::new(
                                HirOperation::FunctionCall(
                                    get_integer_expression(0),
                                    vec![
                                        get_integer_expression(1),
                                        get_integer_expression(2),
                                    ],
                                ),
                            ),
                        ),
                    );
                }
            }
        }
    }

    describe "literal" {
        describe "boolean" {
            it "expects true or false" {
                assert_eq!(
                    new_analyzer().literal(
                        node!("Literal::literal" => [
                            node!("Literal::boolean" => [leaf!("true")]),
                        ]).into_node(),
                    ),
                    HirLiteral::Boolean(true),
                );

                assert_eq!(
                    new_analyzer().literal(
                        node!("Literal::literal" => [
                            node!("Literal::boolean" => [leaf!("false")]),
                        ]).into_node(),
                    ),
                    HirLiteral::Boolean(false),
                );
            }
        }

        describe "integer number" {
            it "applies default exponent and data type suffix" {
                assert_eq!(
                    new_analyzer().literal(
                        node!("Literal::literal" => [
                            node!("Literal::number" => [
                                node!("value" => [
                                    node!("Literal::decimal_number" => [leaf!("0")]),
                                ]),
                            ]),
                        ]).into_node(),
                    ),
                    HirLiteral::Integer(
                        HirIntegerLiteral {
                            data_type: None,
                            base: HirIntegerBase::Decimal,
                            value: "0".to_string(),
                            exponent: None,
                        },
                    ),
                );
            }

            it "hirifies exponent and data type suffix" {
                assert_eq!(
                    new_analyzer().literal(
                        node!("Literal::literal" => [
                            node!("Literal::number" => [
                                node!("value" => [
                                    node!("Literal::decimal_number" => [leaf!("0")]),
                                ]),
                                node!("Literal::number_exponent" => [
                                    leaf!("+"),
                                    node!("value" => [leaf!("1")]),
                                ]),
                                node!("data_type_suffix" => [leaf!("usize")]),
                            ]),
                        ]).into_node(),
                    ),
                    HirLiteral::Integer(
                        HirIntegerLiteral {
                            data_type: Some(HirPrimitiveDataType::Usize),
                            base: HirIntegerBase::Decimal,
                            value: "0".to_string(),
                            exponent: Some(
                                HirIntegerExponent {
                                    positive: true,
                                    value: "1".to_string(),
                                },
                            ),
                        },
                    ),
                );
            }
        }

        describe "float number" {
            it "applies default exponent and data type suffix" {
                assert_eq!(
                    new_analyzer().literal(
                        node!("Literal::literal" => [
                            node!("Literal::number" => [
                                node!("Literal::float_number" => [
                                    node!("integer" => [leaf!("0")]),
                                    node!("float" => [leaf!("0")]),
                                ]),
                            ]),
                        ]).into_node(),
                    ),
                    HirLiteral::Float(
                        HirFloatLiteral {
                            data_type: None,
                            value: "0.0".to_string(),
                        },
                    ),
                );
            }

            it "hirifies data type suffix" {
                assert_eq!(
                    new_analyzer().literal(
                        node!("Literal::literal" => [
                            node!("Literal::number" => [
                                node!("Literal::float_number" => [
                                    node!("integer" => [leaf!("0")]),
                                    node!("float" => [leaf!("0")]),
                                    node!("data_type_suffix" => [leaf!("f32")]),
                                ]),
                            ]),
                        ]).into_node(),
                    ),
                    HirLiteral::Float(
                        HirFloatLiteral {
                            data_type: Some(HirPrimitiveDataType::F32),
                            value: "0.0".to_string(),
                        },
                    ),
                );
            }
        }

        describe "self literal" {
            it "hirifies self" {
                assert_eq!(
                    new_analyzer().literal(
                        node!("Literal::literal" => [
                            node!("self" => [leaf!("self")]),
                        ]).into_node(),
                    ),
                    HirLiteral::SelfValue,
                );
            }
        }

        describe "none" {
            it "hirifies none" {
                assert_eq!(
                    new_analyzer().literal(
                        node!("Literal::literal" => [
                            node!("none" => [leaf!("none")]),
                        ]).into_node(),
                    ),
                    HirLiteral::None,
                );
            }
        }
    }

    describe "data type" {
        it "hirifies primitive data type" {
            assert_eq!(
                new_analyzer().data_type(
                    node!("DataType::data_type" => [
                        node!("DataType::primitive" => [leaf!("usize")]),
                    ]).into_node(),
                ),
                HirDataType::Primitive(HirPrimitiveDataType::Usize),
            );
        }

        it "hirifies generic data type" {
            assert_eq!(
                new_analyzer().data_type(
                    node!("DataType::data_type" => [
                        node!("DataType::generic" => [
                            node!("Identifier::identifier" => [leaf!("t")]),
                            node!("DataType::generic_arguments" => [
                                node!("DataType::data_type" => [
                                    node!("DataType::generic" => [
                                        node!("Identifier::identifier" => [leaf!("t")]),
                                        node!("DataType::generic_arguments" => [
                                            node!("Identifier::identifier" => [leaf!("T")]),
                                        ]),
                                    ]),
                                ]),
                                node!("Identifier::identifier" => [leaf!("T")]),
                            ]),
                        ]),
                    ]).into_node(),
                ),
                HirDataType::Generic(
                    HirIdentifierBinding::new(
                        "t".into(),
                        HirGenericDataType {
                            arguments: vec![
                                HirDataType::Generic(
                                    HirIdentifierBinding::new(
                                        "t".into(),
                                        HirGenericDataType {
                                            arguments: vec![HirDataType::Identifier("T".into())],
                                        },
                                    ),
                                ),
                                HirDataType::Identifier("T".into()),
                            ],
                        },
                    ),
                ),
            );
        }

        it "hirifies generic arguments" {
            assert_eq!(
                new_analyzer().generic_arguments(
                    node!("DataType::generic_arguments" => [
                        node!("DataType::data_type" => [
                            node!("DataType::generic" => [
                                node!("Identifier::identifier" => [leaf!("t")]),
                                node!("DataType::generic_arguments" => [
                                    node!("Identifier::identifier" => [leaf!("T")]),
                                ]),
                            ]),
                        ]),
                        node!("Identifier::identifier" => [leaf!("T")]),
                    ]).into_node(),
                ),
                vec![
                    HirDataType::Generic(
                        HirIdentifierBinding::new(
                            "t".into(),
                            HirGenericDataType {
                                arguments: vec![HirDataType::Identifier("T".into())],
                            },
                        ),
                    ),
                    HirDataType::Identifier("T".into()),
                ],
            );
        }
    }
}
