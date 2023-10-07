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
                            node!("name" => [
                                leaf!("f"),
                            ]),
                            node!("args" => []),
                            node!("exprs" => []),
                        ]),
                    ]),
                    node!("Item::item" => [
                        node!("Function::function" => [
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
                        },
                    ),
                    HirItem::Function(
                        HirFunction {
                            name: "f".to_string(),
                        },
                    ),
                ],
            },
        );
    }

    describe "item" {
        it "hirify function" {
            assert_eq!(
                tree_analysis.function(
                    node!("Function::function" => [
                        node!("name" => [
                            leaf!("f"),
                        ]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ]).into_node(),
                ),
                HirFunction {
                    name: "f".to_string(),
                },
            );
        }
    }
}
