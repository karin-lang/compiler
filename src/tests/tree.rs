use crate::{hir::*, tree::TreeAnalysis};
use speculate::speculate;
use volt::{*, tree::*};

speculate!{
    before {
        let mut tree_analysis = TreeAnalysis;
    }

    it "tree" {
        assert_eq!(
            tree_analysis.analyze(
                node!("Main::main" => [
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
                ]).into_node(),
            ),
            Hir {
                items: vec![
                    HirItem::Function(
                        HirFunction {
                            name: "f".to_string(),
                            accessibility: HirAccessibility::Private,
                            arguments: Vec::new(),
                            expressions: Vec::new(),
                        },
                    ),
                ],
            },
        );
    }

    describe "accessibility" {
        it "private" {
            assert_eq!(
                tree_analysis.accessibility(
                    node!("Main::accessibility" => []).into_node(),
                ),
                HirAccessibility::Private,
            );
        }

        it "public" {
            assert_eq!(
                tree_analysis.accessibility(
                    node!("Main::accessibility" => [
                        leaf!("pub"),
                    ]).into_node(),
                ),
                HirAccessibility::Public,
            );
        }

        it "public in hako" {
            assert_eq!(
                tree_analysis.accessibility(
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
                tree_analysis.function(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [
                            leaf!("f"),
                        ]),
                        node!("args" => [
                            node!("Function::argument" => [
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
                HirFunction {
                    name: "f".to_string(),
                    accessibility: HirAccessibility::Private,
                    arguments: vec![
                        HirFormalArgument {
                            name: "a".to_string(),
                            data_type: HirDataType::Primitive(HirPrimitiveDataType::Usize),
                        },
                    ],
                    expressions: vec![
                        HirExpression::Literal(HirLiteral::Boolean(true)),
                    ],
                },
            );
        }
    }

    it "expression" {
        assert_eq!(
            tree_analysis.expression(
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
                tree_analysis.literal(
                    node!("Literal::literal" => [
                        node!("Literal::boolean" => [leaf!("true")]),
                    ]).into_node(),
                ),
                HirLiteral::Boolean(true),
            );
        }

        it "integer" {
            assert_eq!(
                tree_analysis.literal(
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
                tree_analysis.literal(
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
                    tree_analysis.data_type(
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
                    tree_analysis.data_type(
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
                        HirGenericDataType {
                            name: "t".to_string(),
                            arguments: vec![
                                HirDataType::Generic(
                                    HirGenericDataType {
                                        name: "t".to_string(),
                                        arguments: vec![HirDataType::Identifier(HirUnresolvedIdentifier("T".to_string()))],
                                    },
                                ),
                                HirDataType::Identifier(HirUnresolvedIdentifier("T".to_string())),
                            ],
                        },
                    ),
                );
            }
        }
    }
}
