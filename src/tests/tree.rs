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
                        },
                    ),
                    HirItem::Function(
                        HirFunction {
                            name: "f".to_string(),
                            accessibility: HirAccessibility::Private,
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
        it "hirify function" {
            assert_eq!(
                tree_analysis.function(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("name" => [
                            leaf!("f"),
                        ]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ]).into_node(),
                ),
                HirFunction {
                    name: "f".to_string(),
                    accessibility: HirAccessibility::Private,
                },
            );
        }
    }
}
