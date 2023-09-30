use crate::syntax::Syntax;
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

        // todo: マッチしなかった場合のみテストを通すための関数を作る
        #[allow(unused)]
        let expect_failure = |input: &str, rule_id: &str|
            Parser::parse(volt, input, &RuleId(rule_id.to_string())).expect_err(&"parsing unexpectedly succeeded".cyan().to_string());

        #[allow(unused)]
        let expect_failure_eq = |input: &str, rule_id: &str, expected: ParserError|
            assert_ast(input, rule_id, Err(expected));
    }

    describe "main" {
        it "separated by statement end" {
            expect_success("\n", "Main::main");
            expect_success("fn f() {}\nfn f() {}", "Main::main");
            expect_success("\nfn f() {}\nfn f() {}\n", "Main::main");
        }

        it "rejects semicolon separator" {
            expect_failure(";", "Main::main");
        }
    }

    describe "function" {
        it "can specify pub keyword optionally" {
            expect_success("pub fn f() {}", "Function::function");

            expect_success("fn f() {}", "Function::function");
        }

        it "accepts zero arguments and zero expressions" {
            expect_success_eq("fn f() {}", "Function::function", tree!{
                node!{
                    "Function::function" => vec![
                        node!{
                            "id" => vec![
                                leaf!("f"),
                            ]
                        },
                        node!{
                            "args" => vec![]
                        },
                        node!{
                            "exprs" => vec![]
                        },
                    ]
                }
            });
        }

        it "rejects only one argument separator" {
            expect_failure("fn f(,) {}", "Function::function");
        }

        it "accepts multiple arguments" {
            expect_success_eq("fn f(a usize, b usize, ) {}", "Function::function", tree!{
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
                        node!{
                            "exprs" => vec![]
                        },
                    ]
                }
            });
        }

        it "accepts argument separator at the end" {
            expect_success_eq("fn f(a usize, b usize, ) {}", "Function::function", tree!{
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
                        node!{
                            "exprs" => vec![]
                        },
                    ]
                }
            });
        }

        it "can contain a single expression in a line" {
            expect_success_eq("fn f() {expr}", "Function::function", tree!{
                node!{
                    "Function::function" => vec![
                        node!{
                            "id" => vec![
                                leaf!("f"),
                            ]
                        },
                        node!{
                            "args" => vec![]
                        },
                        node!{
                            "exprs" => vec![
                                node!{
                                    "Expression::expression" => vec![
                                        leaf!("expr"),
                                    ]
                                },
                            ]
                        },
                    ]
                }
            });
        }

        it "accepts expression separators around a single expression" {
            expect_success("fn f() { ;\nexpr ;\n}", "Function::function");
        }

        it "can contain multiple expressions in lines" {
            expect_success_eq("fn f() {expr\nexpr}", "Function::function", tree!{
                node!{
                    "Function::function" => vec![
                        node!{
                            "id" => vec![
                                leaf!("f"),
                            ]
                        },
                        node!{
                            "args" => vec![]
                        },
                        node!{
                            "exprs" => vec![
                                node!{
                                    "Expression::expression" => vec![
                                        leaf!("expr"),
                                    ]
                                },
                                node!{
                                    "Expression::expression" => vec![
                                        leaf!("expr"),
                                    ]
                                },
                            ]
                        },
                    ]
                }
            });
        }

        it "accepts expression separators around multiple expressions" {
            expect_success("fn f() {\nexpr\nexpr\n}", "Function::function");
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
