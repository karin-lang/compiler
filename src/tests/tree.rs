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
                            node!("name" => [
                                leaf!("f"),
                            ]),
                            node!("args" => []),
                            node!("exprs" => []),
                        ]),
                    ]),
                    node!("Item::item" => [
                        node!("Function::function" => [
                            node!("Main::accessibility" => []),
                            node!("name" => [
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
                        },
                    ),
                    HirItem::Function(
                        HirFunction {
                            name: "f".to_string(),
                            accessibility: HirAccessibility::Private,
                            arguments: Vec::new(),
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
                        node!("name" => [
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
                        node!("exprs" => []),
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
                },
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
                                node!("name" => [
                                    leaf!("t"),
                                ]),
                                node!("args" => [
                                    node!("DataType::data_type" => [
                                        node!("DataType::generic" => [
                                            node!("name" => [
                                                leaf!("t"),
                                            ]),
                                            node!("args" => [
                                                node!("name" => [
                                                    leaf!("T"),
                                                ]),
                                            ]),
                                        ]),
                                    ]),
                                    node!("name" => [
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
