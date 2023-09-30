use speculate::speculate;
use volt::{*, parser::*, rule::*, tree::*};
use crate::syntax::Syntax;

speculate!{
    before {
        let volt = &mut Syntax::generate_volt(1024);

        let assert_ast = |input: &str, rule_id: &str, expected: ParserResult|
            assert_eq!(Parser::parse(volt, input, &RuleId(rule_id.to_string())), expected);

        #[allow(unused)]
        let expect_success = |input: &str, rule_id: &str|
            assert!(Parser::parse(volt, input, &RuleId(rule_id.to_string())).is_ok());

        #[allow(unused)]
        let expect_success_eq = |input: &str, rule_id: &str, expected: SyntaxTree|
            assert_ast(input, rule_id, Ok(expected));

        #[allow(unused)]
        let expect_failure = |input: &str, rule_id: &str|
            assert!(Parser::parse(volt, input, &RuleId(rule_id.to_string())).is_err());

        #[allow(unused)]
        let expect_failure_eq = |input: &str, rule_id: &str, expected: ParserError|
            assert_ast(input, rule_id, Err(expected));
    }

    describe "main" {
        it "separated by statement end" {
            expect_success("\n", "Main::main");
            expect_success("fn f(){}\nfn f(){}", "Main::main");
            expect_success("\nfn f(){}\nfn f(){}\n", "Main::main");
        }

        it "rejects semicolon separator" {
            expect_failure(";", "Main::main");
        }
    }

    describe "function" {
        it "can specify pub keyword optionally" {
            expect_success("pub fn f(){}", "Function::function");

            expect_success("fn f(){}", "Function::function");
        }

        it "accepts zero arguments" {
            expect_success_eq("fn f(){}", "Function::function", tree!{
                node!{
                    "Function::function" => vec![
                        node!{
                            "id" => vec![
                                leaf!("f"),
                            ]
                        },
                    ]
                }
            });
        }

        it "accepts multiple arguments and separator at the end" {
            expect_success_eq("fn f(a usize, b usize, ){}", "Function::function", tree!{
                node!{
                    "Function::function" => vec![
                        node!{
                            "id" => vec![
                                leaf!("f"),
                            ]
                        },
                        node!{
                            "args" => vec![
                                node!{
                                    "Function::argument" => vec![
                                        node!{
                                            "Identifier::identifier" => vec![
                                                leaf!("a"),
                                            ]
                                        },
                                        node!{
                                            "DataType::data_type" => vec![
                                                node!{
                                                    "DataType::primitive" => vec![
                                                        leaf!("usize"),
                                                    ]
                                                },
                                            ]
                                        },
                                    ]
                                },
                                node!{
                                    "Function::argument" => vec![
                                        node!{
                                            "Identifier::identifier" => vec![
                                                leaf!("b"),
                                            ]
                                        },
                                        node!{
                                            "DataType::data_type" => vec![
                                                node!{
                                                    "DataType::primitive" => vec![
                                                        leaf!("usize"),
                                                    ]
                                                },
                                            ]
                                        },
                                    ]
                                },
                            ]
                        },
                    ]
                }
            });
        }

        it "can contain expression and statement" {
            expect_success_eq("fn f(){}", "Function::function", tree!{
                node!{
                    "Function::function" => vec![
                        node!{
                            "id" => vec![
                                leaf!("f"),
                            ]
                        },
                    ]
                }
            });
        }
    }

    describe "identifier" {
        it "accepts alphabetic start" {
            expect_success_eq("a", "Identifier::identifier", tree!{
                node!{
                    "Identifier::identifier" => vec![
                        leaf!("a"),
                    ]
                }
            });
        }

        it "accepts multiple characters" {
            expect_success_eq("a0_", "Identifier::identifier", tree!{
                node!{
                    "Identifier::identifier" => vec![
                        leaf!("a0_"),
                    ]
                }
            });
        }

        it "rejects numeric start" {
            expect_failure_eq("0", "Identifier::identifier", ParserError::NoMatchedRule);
        }
    }

    describe "symbol" {
        describe "statement end" {
            it "accepts multiple whitespaces around statement end optionally" {
                expect_success("\n", "Symbol::statement_end");
                expect_success(";", "Symbol::statement_end");
                expect_success("  \n  ", "Symbol::statement_end");
            }
        }
    }

    describe "data type" {
        it "primitive type" {
            expect_success_eq("usize", "DataType::data_type", tree!{
                node!{
                    "DataType::data_type" => vec![
                        node!{
                            "DataType::primitive" => vec![
                                leaf!("usize"),
                            ]
                        },
                    ]
                }
            });
        }
    }
}
