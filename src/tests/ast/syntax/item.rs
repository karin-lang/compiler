use crate::ast::syntax::Syntax;
use colored::*;
use speculate::speculate;
use volt::{*, parser::*, rule::*, tree::*};

speculate!{
    before {
        let volt = &mut Syntax::generate_volt(1024);

        let assert_ast = |input: &str, rule_id: &str, expected: ParserResult|
            assert_eq!(Parser::parse(volt, input, &RuleId(rule_id.to_string())), expected);

        #[allow(unused)]
        let expect_success = |input: &str, rule_id: &str|
            Parser::parse(volt, input, &RuleId(rule_id.to_string())).expect(&"parsing unexpectedly failed".red().to_string());

        #[allow(unused)]
        let expect_success_eq = |input: &str, rule_id: &str, expected: SyntaxTree|
            assert_ast(input, rule_id, Ok(expected));

        #[allow(unused)]
        let expect_failure = |input: &str, rule_id: &str|
            Parser::parse(volt, input, &RuleId(rule_id.to_string())).expect_err(&"parsing unexpectedly succeeded".cyan().to_string());

        #[allow(unused)]
        let expect_unmatch_failure = |input: &str, rule_id: &str|
            if Parser::parse(volt, input, &RuleId(rule_id.to_string())) != Err(ParserError::NoMatchedRule) {
                panic!("{}", "input unexpectedly matched syntax rule".cyan());
            };

        #[allow(unused)]
        let expect_failure_eq = |input: &str, rule_id: &str, expected: ParserError|
            assert_ast(input, rule_id, Err(expected));
    }

    describe "item" {
        it "matches function" {
            expect_success_eq("fn f() {}", "Item::item", tree!(
                node!("Item::item" => [
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ]),
                ])
            ));
        }
    }

    describe "function" {
        describe "accessibility" {
            it "specifies accessibility optionally" {
                expect_success("pub fn f() {}", "Function::function");
                expect_success("fn f() {}", "Function::function");
            }

            it "has accessibility node" {
                expect_success_eq("fn f() {}", "Function::function", tree!(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ])
                ));

                expect_success_eq("pub fn f() {}", "Function::function", tree!(
                    node!("Function::function" => [
                        node!("Main::accessibility" => [leaf!("pub")]),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ])
                ));
            }
        }

        describe "argument" {
            it "has no argument and expression" {
                expect_success_eq("fn f() {}", "Function::function", tree!(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ])
                ));
            }

            it "has zero or more arguments" {
                expect_success_eq("fn f() {}", "Function::function", tree!(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ])
                ));

                expect_success_eq("fn f(a usize, b usize) {}", "Function::function", tree!(
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
                            node!("Function::formal_argument" => [
                                node!("Identifier::identifier" => [
                                    leaf!("b"),
                                ]),
                                node!("DataType::data_type" => [
                                    node!("DataType::primitive" => [
                                        leaf!("usize"),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("exprs" => []),
                    ])
                ));
            }

            it "does not allow separator with no argument" {
                expect_unmatch_failure("fn f(,) {}", "Function::function");
            }

            it "does not allow separator at start" {
                expect_unmatch_failure("fn f(, a usize) {}", "Function::function");
            }

            it "allows separator at the end" {
                expect_success_eq("fn f(a usize, b usize, ) {}", "Function::function", tree!(
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
                            node!("Function::formal_argument" => [
                                node!("Identifier::identifier" => [
                                    leaf!("b"),
                                ]),
                                node!("DataType::data_type" => [
                                    node!("DataType::primitive" => [
                                        leaf!("usize"),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("exprs" => []),
                    ])
                ));
            }
        }

        describe "expression" {
            it "has zero or more expressions" {
                expect_success_eq("fn f() {}", "Function::function", tree!(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ])
                ));

                expect_success_eq("fn f() {0}", "Function::function", tree!(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [
                            leaf!("f"),
                        ]),
                        node!("args" => []),
                        node!("exprs" => [
                            node!("Expression::expression" => [
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
                        ]),
                    ])
                ));

                expect_success_eq("fn f() {0\n0}", "Function::function", tree!(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("exprs" => [
                            node!("Expression::expression" => [
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
                            node!("Expression::expression" => [
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
                        ]),
                    ])
                ));
            }

            it "separated by expression separator" {
                expect_success("fn f() {0\n0}", "Function::function");
                expect_success("fn f() {0;0}", "Function::function");
            }

            it "allows separators and whitespaces around expression" {
                expect_success("fn f() { ;\n0 ;\n}", "Function::function");
            }
        }

        describe "return type" {
            it "has return type optionally" {
                expect_success_eq("fn f() {}", "Function::function", tree!(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("exprs" => []),
                    ])
                ));

                expect_success_eq("fn f() usize {}", "Function::function", tree!(
                    node!("Function::function" => [
                        node!("Main::accessibility" => []),
                        node!("Identifier::identifier" => [leaf!("f")]),
                        node!("args" => []),
                        node!("DataType::data_type" => [
                            node!("DataType::primitive" => [
                                leaf!("usize"),
                            ]),
                        ]),
                        node!("exprs" => []),
                    ])
                ));
            }
        }
    }
}
