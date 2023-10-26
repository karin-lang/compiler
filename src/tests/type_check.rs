use crate::hir::{*, expr::*, item::*, path::*, type_check::*};
use speculate::speculate;

fn check<F: FnMut(&mut DataTypeChecker)>(
    path_nodes: Vec<HirPathNode>,
    mut modify: F,
) -> Vec<DataTypeError> {
    let mut path_tree = HirPathTree::new();
    let mut index_generator = HirPathIndexGenerator::new();

    for each_node in path_nodes {
        path_tree.add_node(&mut index_generator, None, each_node);
    }

    let mut checker: DataTypeChecker<'_> = DataTypeChecker::new(&path_tree);
    modify(&mut checker);
    checker.errors
}

speculate!{
    describe "item" {
        it "checks path expression in function" {
            let mut item = HirItem::Function(
                HirFunction {
                    accessibility: HirAccessibility::Private,
                    arguments: Vec::new(),
                    expressions: vec![
                        HirExpression::Operation(
                            Box::new(
                                HirOperation::Path(
                                    HirPath::Unresolved(vec!["unknown".into()]),
                                ),
                            ),
                        ),
                    ],
                },
            );

            let errors = check(
                Vec::new(),
                |checker| checker.item(&mut item),
            );

            assert_eq!(errors, vec![DataTypeError::UnknownIdentifier]);
        }
    }

    describe "expression" {
        it "checks path expression" {
            let mut expr = HirExpression::Operation(
                Box::new(
                    HirOperation::Path(
                        HirPath::Unresolved(vec!["unknown".into()]),
                    ),
                ),
            );

            let errors = check(
                Vec::new(),
                |checker| checker.expression(&mut expr),
            );

            assert_eq!(errors, vec![DataTypeError::UnknownIdentifier]);
        }
    }

    describe "path" {
        it "resolves existing identifier" {
            let mut path = HirPath::Unresolved(vec!["existing".into()]);

            let errors = check(
                vec![
                    HirPathNode {
                        id: "existing".into(),
                        kind: HirPathKind::Hako,
                        parent: None,
                        children: Vec::new(),
                    },
                ],
                |checker| checker.path(&mut path),
            );

            assert_eq!(errors, Vec::new());
            assert_eq!(path, HirPath::Resolved(0.into()));
        }

        it "detects unknown identifier" {
            let mut path = HirPath::Unresolved(vec!["unknown".into()]);

            let errors = check(
                Vec::new(),
                |checker| checker.path(&mut path),
            );

            assert_eq!(errors, vec![DataTypeError::UnknownIdentifier]);
        }
    }
}
