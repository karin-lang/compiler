use std::collections::BTreeMap;
use crate::{hir::{*, expr::*, item::*, path::*}, tree::*};
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
                nodes: BTreeMap::from([
                    (
                        HirPathIndex::from(0),
                        HirPathNode {
                            id: "h".to_string(),
                            kind: HirPathKind::Hako,
                            parent: None,
                            children: Vec::new(),
                        },
                    ),
                ]),
            },
        );

        assert_eq!(analyzer.items, Vec::new());
    }

    describe "module" {
        it "no children" {
            let syntax_child = node!("Main::main" => []);
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
                                id: "m".to_string(),
                                kind: HirPathKind::Module,
                                parent: Some(HirPathIndex::from(100)),
                                children: Vec::new(),
                            },
                        )
                    ]),
                },
            );

            assert_eq!(
                analyzer.items,
                Vec::new(),
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
        it "function" {
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
