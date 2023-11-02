use std::collections::BTreeMap;
use crate::{hir::{*, expr::*, item::*, path::*}, ast::tree::*};
use speculate::speculate;
use volt::{*, tree::*};

speculate!{
    before {
        let new_analyzer = || TreeAnalysis::new();
    }

    it "tree" {
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

        assert_eq!(analyzer.items, Vec::new());
    }

    describe "module" {
        it "reflects path indexes for itself and parent" {
            let syntax_child = node!("Main::main" => []);
            let mut analyzer = new_analyzer();

            let path_index = analyzer.module(
                &AstModule {
                    id: "m".to_string(),
                    node: syntax_child.into_node(),
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

        it "reflects submodules" {
            let syntax_child = node!("Main::main" => []);
            let mut analyzer = new_analyzer();

            analyzer.module(
                &AstModule {
                    id: "m".to_string(),
                    node: syntax_child.into_node(),
                    submodules: vec![
                        AstModule {
                            id: "sm".to_string(),
                            node: syntax_child.into_node(),
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

        it "reflects subitems" {
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
                                arguments: Vec::new(),
                                expressions: Vec::new(),
                            },
                        ),
                    ),
                ],
            );
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
                    node!("Main::accessibility" => [
                        leaf!("pub"),
                    ]).into_node(),
                ),
                HirAccessibility::Public,
            );
        }

        it "public in hako" {
            assert_eq!(
                new_analyzer().accessibility(
                    node!("Main::accessibility" => [
                        leaf!("pub@hako"),
                    ]).into_node(),
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
                        node!("Identifier::identifier" => [
                            leaf!("f"),
                        ]),
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

        describe "function" {
            it "reflects accessibility, arguments and expressions" {
                assert_eq!(
                    new_analyzer().function(
                        node!("Function::function" => [
                            node!("Main::accessibility" => []),
                            node!("Identifier::identifier" => [
                                leaf!("f"),
                            ]),
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
        }
    }

    it "expression" {
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

    describe "literal" {
        it "boolean" {
            assert_eq!(
                new_analyzer().literal(
                    node!("Literal::literal" => [
                        node!("Literal::boolean" => [leaf!("true")]),
                    ]).into_node(),
                ),
                HirLiteral::Boolean(true),
            );
        }

        it "integer" {
            assert_eq!(
                new_analyzer().literal(
                    node!("Literal::literal" => [
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [
                                    leaf!("0"),
                                ]),
                            ]),
                            node!("Literal::number_exponent" => [
                                leaf!("+"),
                                node!("value" => [
                                    leaf!("1"),
                                ]),
                            ]),
                            node!("DataType::primitive_number" => [
                                leaf!("usize"),
                            ]),
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

        it "float" {
            assert_eq!(
                new_analyzer().literal(
                    node!("Literal::literal" => [
                        node!("Literal::number" => [
                            node!("Literal::float_number" => [
                                node!("integer" => [
                                    leaf!("0"),
                                ]),
                                node!("float" => [
                                    leaf!("0"),
                                ]),
                                node!("DataType::primitive_number" => [
                                    leaf!("f32"),
                                ]),
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

    describe "operation" {
        describe "treat the third element as an operation term" {
            it "contained term" {
                assert_eq!(
                    new_analyzer().operation(
                        node!("Operation::operation" => [
                            node!("Expression::operation_term" => [
                                node!("Literal::literal" => [
                                    node!("Literal::boolean" => [leaf!("true")]),
                                ]),
                            ]),
                            leaf!("+"),
                            node!("Operation::arithmetic1" => [
                                node!("Expression::operation_term" => [
                                    node!("Literal::literal" => [
                                        node!("Literal::boolean" => [leaf!("true")]),
                                    ]),
                                ]),
                                leaf!("*"),
                                node!("Expression::operation_term" => [
                                    node!("Literal::literal" => [
                                        node!("Literal::boolean" => [leaf!("true")]),
                                    ]),
                                ]),
                            ]),
                        ]).into_node(),
                    ),
                    HirExpression::Operation(
                        Box::new(
                            HirOperation::Add(
                                HirExpression::Literal(HirLiteral::Boolean(true)),
                                HirExpression::Operation(
                                    Box::new(
                                        HirOperation::Multiply(
                                            HirExpression::Literal(HirLiteral::Boolean(true)),
                                            HirExpression::Literal(HirLiteral::Boolean(true)),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                );
            }
        }

        describe "prefix operator" {
            it "prioritizes the last operator" {
                assert_eq!(
                    new_analyzer().operation(
                        node!("Operation::operation" => [
                            leaf!("!e"),
                            leaf!("-e"),
                            node!("Expression::operation_term" => [
                                node!("Literal::literal" => [
                                    node!("Literal::boolean" => [leaf!("true")]),
                                ]),
                            ]),
                        ]).into_node(),
                    ),
                    HirExpression::Operation(
                        Box::new(
                            HirOperation::Not(
                                HirExpression::Operation(
                                    Box::new(
                                        HirOperation::Negative(
                                            HirExpression::Literal(HirLiteral::Boolean(true)),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                );
            }

            it "treat the last element as an operation term" {
                assert_eq!(
                    new_analyzer().operation(
                        node!("Operation::operation" => [
                            leaf!("!e"),
                            node!("Operation::arithmetic1" => [
                                node!("Expression::operation_term" => [
                                    node!("Literal::literal" => [
                                        node!("Literal::boolean" => [leaf!("true")]),
                                    ]),
                                ]),
                                leaf!("+"),
                                node!("Expression::operation_term" => [
                                    node!("Literal::literal" => [
                                        node!("Literal::boolean" => [leaf!("true")]),
                                    ]),
                                ]),
                            ]),
                        ]).into_node(),
                    ),
                    HirExpression::Operation(
                        Box::new(
                            HirOperation::Not(
                                HirExpression::Operation(
                                    Box::new(
                                        HirOperation::Add(
                                            HirExpression::Literal(HirLiteral::Boolean(true)),
                                            HirExpression::Literal(HirLiteral::Boolean(true)),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                );
            }
        }

        describe "postfix operator" {
            it "prioritizes the last operator" {
                assert_eq!(
                    new_analyzer().operation(
                        node!("Operation::operation" => [
                            leaf!("e?"),
                            leaf!("e!"),
                            node!("Expression::operation_term" => [
                                node!("Literal::literal" => [
                                    node!("Literal::boolean" => [leaf!("true")]),
                                ]),
                            ]),
                        ]).into_node(),
                    ),
                    HirExpression::Operation(
                        Box::new(
                            HirOperation::Propagate(
                                HirExpression::Operation(
                                    Box::new(
                                        HirOperation::Nonnize(
                                            HirExpression::Literal(HirLiteral::Boolean(true)),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                );
            }

            it "treat the last element as an operation term" {
                assert_eq!(
                    new_analyzer().operation(
                        node!("Operation::operation" => [
                            leaf!("e!"),
                            node!("Operation::arithmetic1" => [
                                node!("Expression::operation_term" => [
                                    node!("Literal::literal" => [
                                        node!("Literal::boolean" => [leaf!("true")]),
                                    ]),
                                ]),
                                leaf!("+"),
                                node!("Expression::operation_term" => [
                                    node!("Literal::literal" => [
                                        node!("Literal::boolean" => [leaf!("true")]),
                                    ]),
                                ]),
                            ]),
                        ]).into_node(),
                    ),
                    HirExpression::Operation(
                        Box::new(
                            HirOperation::Nonnize(
                                HirExpression::Operation(
                                    Box::new(
                                        HirOperation::Add(
                                            HirExpression::Literal(HirLiteral::Boolean(true)),
                                            HirExpression::Literal(HirLiteral::Boolean(true)),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                );
            }
        }

        it "addition" {
            assert_eq!(
                new_analyzer().operation(
                    node!("Operation::operation" => [
                        node!("Expression::operation_term" => [
                            node!("Literal::literal" => [
                                node!("Literal::boolean" => [leaf!("true")]),
                            ]),
                        ]),
                        leaf!("+"),
                        node!("Expression::operation_term" => [
                            node!("Literal::literal" => [
                                node!("Literal::boolean" => [leaf!("true")]),
                            ]),
                        ]),
                        leaf!("+"),
                        node!("Expression::operation_term" => [
                            node!("Literal::literal" => [
                                node!("Literal::boolean" => [leaf!("true")]),
                            ]),
                        ]),
                    ]).into_node(),
                ),
                HirExpression::Operation(
                    Box::new(
                        HirOperation::Add(
                            HirExpression::Operation(
                                Box::new(
                                    HirOperation::Add(
                                        HirExpression::Literal(HirLiteral::Boolean(true)),
                                        HirExpression::Literal(HirLiteral::Boolean(true)),
                                    ),
                                ),
                            ),
                            HirExpression::Literal(HirLiteral::Boolean(true)),
                        ),
                    ),
                ),
            );
        }

        it "path resolution" {
            assert_eq!(
                new_analyzer().operation(
                    node!("Operation::operation" => [
                        node!("Expression::operation_term" => [
                            node!("Identifier::identifier" => [leaf!("a")]),
                        ]),
                        leaf!("::"),
                        node!("Operation::arithmetic1" => [
                            node!("Expression::operation_term" => [
                                node!("Identifier::identifier" => [leaf!("b")]),
                            ]),
                            leaf!("::"),
                            node!("Expression::operation_term" => [
                                node!("Identifier::identifier" => [leaf!("c")]),
                            ]),
                        ]),
                    ]).into_node(),
                ),
                HirExpression::Operation(
                    Box::new(
                        HirOperation::Path(HirPath::Unresolved(vec![
                            "a".into(),
                            "b".into(),
                            "c".into(),
                        ])),
                    ),
                )
            );
        }

        it "grouping" {
            assert_eq!(
                new_analyzer().operation(
                    node!("Operation::operation" => [
                        leaf!("("),
                        node!("Expression::operation_term" => [
                            node!("Literal::literal" => [
                                node!("Literal::boolean" => [leaf!("true")]),
                            ]),
                        ]),
                    ]).into_node(),
                ),
                HirExpression::Operation(
                    Box::new(HirOperation::Group(HirExpression::Literal(HirLiteral::Boolean(true)))),
                )
            );
        }
    }

    describe "data type" {
        describe "primitive" {
            it "primitive data type" {
                assert_eq!(
                    new_analyzer().data_type(
                        node!("DataType::data_type" => [
                            node!("DataType::primitive" => [
                                leaf!("usize"),
                            ]),
                        ]).into_node(),
                    ),
                    HirDataType::Primitive(HirPrimitiveDataType::Usize),
                );
            }

            it "generic data type" {
                assert_eq!(
                    new_analyzer().data_type(
                        node!("DataType::data_type" => [
                            node!("DataType::generic" => [
                                node!("Identifier::identifier" => [
                                    leaf!("t"),
                                ]),
                                node!("args" => [
                                    node!("DataType::data_type" => [
                                        node!("DataType::generic" => [
                                            node!("Identifier::identifier" => [
                                                leaf!("t"),
                                            ]),
                                            node!("args" => [
                                                node!("Identifier::identifier" => [
                                                    leaf!("T"),
                                                ]),
                                            ]),
                                        ]),
                                    ]),
                                    node!("Identifier::identifier" => [
                                        leaf!("T"),
                                    ]),
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
                                                arguments: vec![
                                                    HirDataType::Identifier("T".into()),
                                                ],
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
        }
    }
}