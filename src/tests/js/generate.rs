use std::collections::BTreeMap;
use crate::js::{generate::JsGenerator, ir::*};
use crate::hir::{*, expr::*, item::*, path::*};
use speculate::speculate;

speculate!{
    describe "item" {
        describe "function" {
            it "reflects identifier, arguments and statements" {
                let path_tree = HirPathTree {
                    hako_indexes: Vec::new(),
                    nodes: BTreeMap::from([(
                        HirPathIndex::from(0),
                        HirPathNode {
                            id: "f".into(),
                            kind: HirPathKind::Function,
                            parent: None,
                            children: Vec::new(),
                        },
                    )]),
                };

                let mut generator = JsGenerator::new(&path_tree);

                let item = HirItem::Function(
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
                            HirExpression::Literal(
                                HirLiteral::Boolean(true),
                            ),
                        ],
                    },
                );

                assert_eq!(
                    generator.item(&HirPathIndexBinding::new(0.into(), item)),
                    JsItem::Function(
                        JsFunction {
                            id: "f_0".to_string(),
                            arguments: vec![
                                "a".to_string(),
                            ],
                            statements: vec![
                                JsStatement::Expression(
                                    JsExpression::Literal(
                                        JsLiteral::Boolean(true),
                                    ),
                                ),
                            ],
                        },
                    ),
                );
            }
        }
    }

    describe "statement" {
        it "returns statement" {
            let path_tree = HirPathTree::new();
            let mut generator = JsGenerator::new(&path_tree);

            assert_eq!(
                generator.statement(
                    &HirExpression::Literal(
                        HirLiteral::Boolean(true),
                    ),
                ),
                JsStatement::Expression(
                    JsExpression::Literal(
                        JsLiteral::Boolean(true),
                    ),
                ),
            );
        }

        describe "literal" {
            it "boolean" {
                let path_tree = HirPathTree::new();
                let mut generator = JsGenerator::new(&path_tree);

                assert_eq!(
                    generator.literal(&HirLiteral::Boolean(true)),
                    JsLiteral::Boolean(true),
                );
            }
        }
    }
}
