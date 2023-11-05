use crate::js::{code::JsCodeGenerator, ir::*};
use speculate::speculate;

speculate!{
    it "joins items with empty string" {
        let js = Js {
            items: vec![
                JsItem::Function(
                    JsFunction {
                        id: "i_0".to_string(),
                        arguments: Vec::new(),
                        statements: Vec::new(),
                    },
                ),
                JsItem::Function(
                    JsFunction {
                        id: "i_1".to_string(),
                        arguments: Vec::new(),
                        statements: Vec::new(),
                    },
                ),
            ],
        };

        assert_eq!(
            JsCodeGenerator::generate(&js),
            "function i_0(){}function i_1(){}".to_string(),
        );
    }

    describe "item" {
        it "reflects containing item" {
            let item = JsItem::Function(
                JsFunction {
                    id: "i_0".to_string(),
                    arguments: Vec::new(),
                    statements: Vec::new(),
                },
            );

            assert_eq!(
                JsCodeGenerator::item(&item),
                "function i_0(){}".to_string(),
            );
        }

        describe "function" {
            it "separates arguments and statements" {
                let function = JsFunction {
                    id: "i_0".to_string(),
                    arguments: vec![
                        "a".to_string(),
                        "b".to_string(),
                    ],
                    statements: vec![
                        JsStatement::Expression(
                            JsExpression::Literal(
                                JsLiteral::Boolean(true),
                            ),
                        ),
                        JsStatement::Expression(
                            JsExpression::Literal(
                                JsLiteral::Boolean(true),
                            ),
                        ),
                    ],
                };

                assert_eq!(
                    JsCodeGenerator::function(&function),
                    "function i_0(a,b){true;true}".to_string(),
                );
            }
        }
    }

    describe "statement" {
        describe "expression" {
            it "literal" {
                let expr = JsStatement::Expression(
                    JsExpression::Literal(
                        JsLiteral::Boolean(true),
                    ),
                );

                assert_eq!(
                    JsCodeGenerator::statement(&expr),
                    "true".to_string(),
                );

                let expr = JsStatement::Expression(
                    JsExpression::Literal(
                        JsLiteral::Boolean(false),
                    ),
                );

                assert_eq!(
                    JsCodeGenerator::statement(&expr),
                    "false".to_string(),
                );
            }
        }
    }
}
