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

    describe "expression" {
        it "matches operation" {
            expect_success_eq("0 + 1", "Expression::expression", tree!(
                node!("Expression::expression" => [
                    node!("Operation::operation" => [
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("0")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("operator" => [leaf!("+")]),
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("1")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                    ]),
                ])
            ));
        }

        it "matches literal" {
            expect_success_eq("true", "Expression::expression", tree!(
                node!("Expression::expression" => [
                    node!("Literal::literal" => [
                        node!("Literal::boolean" => [leaf!("true")]),
                    ]),
                ])
            ));
        }

        it "matches identifier" {
            expect_success_eq("id", "Expression::expression", tree!(
                node!("Expression::expression" => [
                    node!("Identifier::identifier" => [leaf!("id")]),
                ])
            ));
        }

        it "matches data type" {
            expect_success_eq("usize", "Expression::expression", tree!(
                node!("Expression::expression" => [
                    node!("DataType::data_type" => [
                        node!("DataType::primitive" => [leaf!("usize")]),
                    ]),
                ])
            ));
        }
    }

    describe "operation" {
        describe "infix operator" {
            it "has two or more terms" {
                expect_success_eq("0 + 1", "Operation::operation", tree!(
                    node!("Operation::operation" => [
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("0")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("operator" => [leaf!("+")]),
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("1")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                    ])
                ));

                expect_success_eq("0 + 1 + 2", "Operation::operation", tree!(
                    node!("Operation::operation" => [
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("0")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("operator" => [leaf!("+")]),
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("1")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("operator" => [leaf!("+")]),
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("2")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                    ])
                ));
            }

            it "allows group as term" {
                expect_success_eq("0 + (1)", "Operation::operation", tree!(
                    node!("Operation::operation" => [
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("0")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("operator" => [leaf!("+")]),
                        node!("operator" => [leaf!("(")]),
                        node!("Expression::expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("1")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("operator" => [leaf!(")")]),
                    ])
                ));
            }
        }

        describe "prefix/postfix operator" {
            it "has zero or more operators" {
                expect_success_eq("!0", "Operation::operation", tree!(
                    node!("Operation::operation" => [
                        node!("operator" => [leaf!("!e")]),
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("0")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                    ])
                ));

                expect_success_eq("!-0", "Operation::operation", tree!(
                    node!("Operation::operation" => [
                        node!("operator" => [leaf!("!e")]),
                        node!("operator" => [leaf!("-e")]),
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("0")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                    ])
                ));

                expect_success_eq("0?", "Operation::operation", tree!(
                    node!("Operation::operation" => [
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("0")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("operator" => [leaf!("e?")]),
                    ])
                ));

                expect_success_eq("0?!", "Operation::operation", tree!(
                    node!("Operation::operation" => [
                        node!("Expression::pure_expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("0")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("operator" => [leaf!("e?")]),
                        node!("operator" => [leaf!("e!")]),
                    ])
                ));
            }

            it "allows prefix/postfix operator around group term" {
                expect_success_eq("!(0)?", "Operation::operation", tree!(
                    node!("Operation::operation" => [
                        node!("operator" => [leaf!("!e")]),
                        node!("operator" => [leaf!("(")]),
                        node!("Expression::expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("0")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("operator" => [leaf!(")")]),
                        node!("operator" => [leaf!("e?")]),
                    ])
                ));
            }

            describe "function call operator" {
                it "has zero or more expressions" {
                    expect_success_eq("0()", "Operation::operation", tree!(
                        node!("Operation::operation" => [
                            node!("Expression::pure_expression" => [
                                node!("Literal::literal" => [
                                    node!("Literal::number" => [
                                        node!("value" => [
                                            node!("Literal::decimal_number" => [leaf!("0")]),
                                        ]),
                                    ]),
                                ]),
                            ]),
                            node!("operator" => [
                                node!("Operation::function_call_operator" => []),
                            ]),
                        ])
                    ));

                    expect_success_eq("0(1)", "Operation::operation", tree!(
                        node!("Operation::operation" => [
                            node!("Expression::pure_expression" => [
                                node!("Literal::literal" => [
                                    node!("Literal::number" => [
                                        node!("value" => [
                                            node!("Literal::decimal_number" => [leaf!("0")]),
                                        ]),
                                    ]),
                                ]),
                            ]),
                            node!("operator" => [
                                node!("Operation::function_call_operator" => [
                                    node!("Expression::expression" => [
                                        node!("Literal::literal" => [
                                            node!("Literal::number" => [
                                                node!("value" => [
                                                    node!("Literal::decimal_number" => [leaf!("1")]),
                                                ]),
                                            ]),
                                        ]),
                                    ]),
                                ]),
                            ]),
                        ])
                    ));
                }

                it "separates multiple expressions" {
                    expect_success_eq("0(1,)", "Operation::operation", tree!(
                        node!("Operation::operation" => [
                            node!("Expression::pure_expression" => [
                                node!("Literal::literal" => [
                                    node!("Literal::number" => [
                                        node!("value" => [
                                            node!("Literal::decimal_number" => [leaf!("0")]),
                                        ]),
                                    ]),
                                ]),
                            ]),
                            node!("operator" => [
                                node!("Operation::function_call_operator" => [
                                    node!("Expression::expression" => [
                                        node!("Literal::literal" => [
                                            node!("Literal::number" => [
                                                node!("value" => [
                                                    node!("Literal::decimal_number" => [leaf!("1")]),
                                                ]),
                                            ]),
                                        ]),
                                    ]),
                                ]),
                            ]),
                        ])
                    ));

                    expect_success_eq("0(1, 2)", "Operation::operation", tree!(
                        node!("Operation::operation" => [
                            node!("Expression::pure_expression" => [
                                node!("Literal::literal" => [
                                    node!("Literal::number" => [
                                        node!("value" => [
                                            node!("Literal::decimal_number" => [leaf!("0")]),
                                        ]),
                                    ]),
                                ]),
                            ]),
                            node!("operator" => [
                                node!("Operation::function_call_operator" => [
                                    node!("Expression::expression" => [
                                        node!("Literal::literal" => [
                                            node!("Literal::number" => [
                                                node!("value" => [
                                                    node!("Literal::decimal_number" => [leaf!("1")]),
                                                ]),
                                            ]),
                                        ]),
                                    ]),
                                    node!("Expression::expression" => [
                                        node!("Literal::literal" => [
                                            node!("Literal::number" => [
                                                node!("value" => [
                                                    node!("Literal::decimal_number" => [leaf!("2")]),
                                                ]),
                                            ]),
                                        ]),
                                    ]),
                                ]),
                            ]),
                        ])
                    ));
                }

                it "can be enclosed by parentheses" {
                    expect_success_eq("0( 1 )", "Operation::operation", tree!(
                        node!("Operation::operation" => [
                            node!("Expression::pure_expression" => [
                                node!("Literal::literal" => [
                                    node!("Literal::number" => [
                                        node!("value" => [
                                            node!("Literal::decimal_number" => [leaf!("0")]),
                                        ]),
                                    ]),
                                ]),
                            ]),
                            node!("operator" => [
                                node!("Operation::function_call_operator" => [
                                    node!("Expression::expression" => [
                                        node!("Literal::literal" => [
                                            node!("Literal::number" => [
                                                node!("value" => [
                                                    node!("Literal::decimal_number" => [leaf!("1")]),
                                                ]),
                                            ]),
                                        ]),
                                    ]),
                                ]),
                            ]),
                        ])
                    ));
                }
            }
        }

        describe "group term" {
            it "encloses an expression term with parentheses" {
                expect_success_eq("(0)", "Operation::operation", tree!(
                    node!("Operation::operation" => [
                        node!("operator" => [leaf!("(")]),
                        node!("Expression::expression" => [
                            node!("Literal::literal" => [
                                node!("Literal::number" => [
                                    node!("value" => [
                                        node!("Literal::decimal_number" => [leaf!("0")]),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("operator" => [leaf!(")")]),
                    ])
                ));

                expect_success_eq("(0 + 1)", "Operation::operation", tree!(
                    node!("Operation::operation" => [
                        node!("operator" => [leaf!("(")]),
                        node!("Expression::expression" => [
                            node!("Operation::operation" => [
                                node!("Expression::pure_expression" => [
                                    node!("Literal::literal" => [
                                        node!("Literal::number" => [
                                            node!("value" => [
                                                node!("Literal::decimal_number" => [leaf!("0")]),
                                            ]),
                                        ]),
                                    ]),
                                ]),
                                node!("operator" => [leaf!("+")]),
                                node!("Expression::pure_expression" => [
                                    node!("Literal::literal" => [
                                        node!("Literal::number" => [
                                            node!("value" => [
                                                node!("Literal::decimal_number" => [leaf!("1")]),
                                            ]),
                                        ]),
                                    ]),
                                ]),
                            ]),
                        ]),
                        node!("operator" => [leaf!(")")]),
                    ])
                ));
            }
        }
    }

    describe "literal" {
        describe "boolean" {
            it "matches true or false" {
                expect_success_eq("true", "Literal::boolean", tree!(
                    node!("Literal::boolean" => [leaf!("true")])
                ));

                expect_success_eq("false", "Literal::boolean", tree!(
                    node!("Literal::boolean" => [leaf!("false")])
                ));
            }
        }

        describe "integer number" {
            describe "data type suffix" {
                it "ends with integer type suffix optionally" {
                    expect_success_eq("0", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [leaf!("0")]),
                            ]),
                        ])
                    ));

                    expect_success_eq("0usize", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [leaf!("0")]),
                            ]),
                            node!("data_type_suffix" => [leaf!("usize")]),
                        ])
                    ));
                }

                it "ends with float type suffix optionally" {
                    expect_success_eq("0", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [leaf!("0")]),
                            ]),
                        ])
                    ));

                    expect_success_eq("0f32", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [leaf!("0")]),
                            ]),
                            node!("data_type_suffix" => [leaf!("f32")]),
                        ])
                    ));
                }
            }

            describe "binary" {
                it "requires binary prefix" {
                    expect_success_eq("0b0", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::binary_number" => [leaf!("0")]),
                            ]),
                        ])
                    ));
                }

                it "allows zero to one number" {
                    expect_success_eq("0b10", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::binary_number" => [leaf!("10")]),
                            ]),
                        ])
                    ));

                    expect_unmatch_failure("0b2", "Literal::number");
                }

                it "reduced by integer reducer" {
                    expect_success_eq("0b_1", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::binary_number" => [
                                    error!("digit_separator_on_side", [leaf!("1")]),
                                ]),
                            ]),
                        ])
                    ));
                }
            }

            describe "octal" {
                it "allows zero to seven number" {
                    expect_success_eq("0o107", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::octal_number" => [leaf!("107")]),
                            ]),
                        ])
                    ));

                    expect_unmatch_failure("0o8", "Literal::number");
                }

                it "reduced by integer reducer" {
                    expect_success_eq("0o_1", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::octal_number" => [
                                    error!("digit_separator_on_side", [leaf!("1")]),
                                ]),
                            ]),
                        ])
                    ));
                }
            }

            describe "hexadecimal" {
                it "accepts zero to nine and A to F number" {
                    expect_success_eq("0x90af", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::hexadecimal_number" => [leaf!("90af")]),
                            ]),
                        ])
                    ));
                }

                it "reduced by integer reducer" {
                    expect_success_eq("0x_1", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::hexadecimal_number" => [
                                    error!("digit_separator_on_side", [leaf!("1")]),
                                ]),
                            ]),
                        ])
                    ));
                }
            }

            describe "decimal" {
                it "allows zero to nine number" {
                    expect_success_eq("109", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [leaf!("109")]),
                            ]),
                        ])
                    ));
                }

                it "reduced by integer reducer" {
                    expect_success_eq("_1", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [
                                    error!("digit_separator_on_side", [leaf!("1")]),
                                ]),
                            ]),
                        ])
                    ));
                }
            }

            describe "exponent" {
                it "has exponent suffix optionally" {
                    expect_success("0", "Literal::number");

                    expect_success_eq("0e1", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [leaf!("0")]),
                            ]),
                            node!("Literal::number_exponent" => [
                                leaf!("+"),
                                node!("value" => [leaf!("1")]),
                            ]),
                        ])
                    ));

                    expect_success_eq("0e-1", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [leaf!("0")]),
                            ]),
                            node!("Literal::number_exponent" => [
                                leaf!("-"),
                                node!("value" => [leaf!("1")]),
                            ]),
                        ])
                    ));
                }

                it "does not allow plus symbol" {
                    expect_success_eq("0e+1", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [leaf!("0")]),
                            ]),
                            node!("Literal::number_exponent" => [
                                error!("explicit_plus_symbol", [leaf!("e+")]),
                                node!("value" => [leaf!("1")]),
                            ]),
                        ])
                    ));
                }

                it "reduced by integer reducer" {
                    expect_success_eq("0e_1", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [
                                    leaf!("0"),
                                ]),
                            ]),
                            node!("Literal::number_exponent" => [
                                leaf!("+"),
                                node!("value" => [
                                    error!("digit_separator_on_side", [leaf!("1")]),
                                ]),
                            ]),
                        ])
                    ));
                }
            }

            describe "integer reducer" {
                it "hides digit separator" {
                    expect_success_eq("1_2", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [
                                    leaf!("12"),
                                ]),
                            ]),
                        ])
                    ));
                }

                it "does not allow digit separator on side" {
                    expect_success_eq("_1", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [
                                    error!("digit_separator_on_side", [leaf!("1")]),
                                ]),
                            ]),
                        ])
                    ));

                    expect_success_eq("1_", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [
                                    error!("digit_separator_on_side", [leaf!("1")]),
                                ]),
                            ]),
                        ])
                    ));
                }

                it "does not start with zero" {
                    expect_success("0", "Literal::number");

                    expect_success_eq("01", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::decimal_number" => [
                                    error!("starts_with_zero", [
                                        leaf!("01"),
                                    ]),
                                ]),
                            ]),
                        ])
                    ));
                }

                it "generates only one capital letter error" {
                    expect_success_eq("0xA", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::hexadecimal_number" => [
                                    error!("has_capital_letter", [leaf!("A")]),
                                ]),
                            ]),
                        ])
                    ));

                    expect_success_eq("0xF", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::hexadecimal_number" => [
                                    error!("has_capital_letter", [leaf!("F")]),
                                ]),
                            ]),
                        ])
                    ));
                }

                it "occurs multiple kinds of error all together" {
                    expect_success_eq("0x_00A", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("value" => [
                                node!("Literal::hexadecimal_number" => [
                                    error!("digit_separator_on_side", [
                                        leaf!("00A"),
                                    ]),
                                    error!("starts_with_zero", [
                                        leaf!("00A"),
                                    ]),
                                    error!("has_capital_letter", [
                                        leaf!("00A"),
                                    ]),
                                ]),
                            ]),
                        ])
                    ));
                }
            }
        }

        describe "float number" {
            describe "data type suffix" {
                it "ends with float type suffix optionally" {
                    expect_success_eq("0.0", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("Literal::float_number" => [
                                node!("integer" => [leaf!("0")]),
                                node!("float" => [leaf!("0")]),
                            ]),
                        ])
                    ));

                    expect_success_eq("0.0f32", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("Literal::float_number" => [
                                node!("integer" => [leaf!("0")]),
                                node!("float" => [leaf!("0")]),
                                node!("data_type_suffix" => [leaf!("f32")]),
                            ]),
                        ])
                    ));
                }

                it "rejects integer type suffix" {
                    expect_failure("0.0usize", "Literal::number");
                }
            }

            it "has integer and decimal part" {
                expect_success_eq("0.0", "Literal::number", tree!(
                    node!("Literal::number" => [
                        node!("Literal::float_number" => [
                            node!("integer" => [leaf!("0")]),
                            node!("float" => [leaf!("0")]),
                        ]),
                    ])
                ));
            }

            it "requires both of integer and decimal part" {
                expect_unmatch_failure("0.", "Literal::number");
                expect_unmatch_failure(".0", "Literal::number");
            }

            it "allows data type suffix" {
                expect_success_eq("0.0f32", "Literal::number", tree!(
                    node!("Literal::number" => [
                        node!("Literal::float_number" => [
                            node!("integer" => [leaf!("0")]),
                            node!("float" => [leaf!("0")]),
                            node!("data_type_suffix" => [leaf!("f32")]),
                        ]),
                    ])
                ));
            }

            it "integer part is reduced by integer reducer" {
                expect_success_eq("_0.0", "Literal::number", tree!(
                    node!("Literal::number" => [
                        node!("Literal::float_number" => [
                            node!("integer" => [
                                error!("digit_separator_on_side", [leaf!("0")]),
                            ]),
                            node!("float" => [leaf!("0")]),
                        ]),
                    ])
                ));
            }

            it "float part is reduced by float reducer" {
                expect_success_eq("0._0", "Literal::number", tree!(
                    node!("Literal::number" => [
                        node!("Literal::float_number" => [
                            node!("integer" => [leaf!("0")]),
                            node!("float" => [
                                error!("digit_separator_on_side", [leaf!("0")]),
                            ]),
                        ]),
                    ])
                ));
            }

            describe "float reducer" {
                it "hides digit separator" {
                    expect_success_eq("0.0_1", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("Literal::float_number" => [
                                node!("integer" => [leaf!("0")]),
                                node!("float" => [leaf!("01")]),
                            ]),
                        ])
                    ));
                }

                it "rejects digit separator on side" {
                    expect_success_eq("0._1", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("Literal::float_number" => [
                                node!("integer" => [leaf!("0")]),
                                node!("float" => [
                                    error!("digit_separator_on_side", [leaf!("1")]),
                                ]),
                            ]),
                        ])
                    ));

                    expect_success_eq("0.1_", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("Literal::float_number" => [
                                node!("integer" => [leaf!("0")]),
                                node!("float" => [
                                    error!("digit_separator_on_side", [leaf!("1")]),
                                ]),
                            ]),
                        ])
                    ));
                }

                it "rejects zero at the end" {
                    expect_success("0.0", "Literal::number");

                    expect_success_eq("0.10", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("Literal::float_number" => [
                                node!("integer" => [leaf!("0")]),
                                node!("float" => [
                                    error!("ends_with_zero", [leaf!("10")]),
                                ]),
                            ]),
                        ])
                    ));
                }

                it "occurs multiple errors all together" {
                    expect_success_eq("0._10", "Literal::number", tree!(
                        node!("Literal::number" => [
                            node!("Literal::float_number" => [
                                node!("integer" => [leaf!("0")]),
                                node!("float" => [
                                    error!("digit_separator_on_side", [leaf!("10")]),
                                    error!("ends_with_zero", [leaf!("10")]),
                                ]),
                            ]),
                        ])
                    ));
                }
            }
        }
    }

    describe "data type" {
        describe "primitive" {
            it "matches any kind of primitive data type" {
                expect_success_eq("char", "DataType::data_type", tree!(
                    node!("DataType::data_type" => [
                        node!("DataType::primitive" => [leaf!("char")]),
                    ])
                ));
            }

            it "expands integer and float type node" {
                expect_success_eq("usize", "DataType::data_type", tree!(
                    node!("DataType::data_type" => [
                        node!("DataType::primitive" => [leaf!("usize")]),
                    ])
                ));

                expect_success_eq("f32", "DataType::data_type", tree!(
                    node!("DataType::data_type" => [
                        node!("DataType::primitive" => [leaf!("f32")]),
                    ])
                ));
            }
        }

        describe "generic" {
            it "separated by comma" {
                expect_success_eq("t<T1, T2>", "DataType::data_type", tree!(
                    node!("DataType::data_type" => [
                        node!("DataType::generic" => [
                            node!("Identifier::identifier" => [leaf!("t")]),
                            node!("DataType::generic_arguments" => [
                                node!("Identifier::identifier" => [leaf!("T1")]),
                                node!("Identifier::identifier" => [leaf!("T2")]),
                            ]),
                        ]),
                    ])
                ));
            }

            it "can contain generic type in arguments" {
                expect_success_eq("t<t<T>, T>", "DataType::data_type", tree!(
                    node!("DataType::data_type" => [
                        node!("DataType::generic" => [
                            node!("Identifier::identifier" => [leaf!("t")]),
                            node!("DataType::generic_arguments" => [
                                node!("DataType::data_type" => [
                                    node!("DataType::generic" => [
                                        node!("Identifier::identifier" => [leaf!("t")]),
                                        node!("DataType::generic_arguments" => [
                                            node!("Identifier::identifier" => [leaf!("T")]),
                                        ]),
                                    ]),
                                ]),
                                node!("Identifier::identifier" => [leaf!("T")]),
                            ]),
                        ]),
                    ])
                ));
            }

            it "rejects zero argument" {
                expect_unmatch_failure("a<>", "DataType::generic");
            }
        }
    }
}
