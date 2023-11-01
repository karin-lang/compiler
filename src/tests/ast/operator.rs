use speculate::speculate;
use crate::hir::expr::*;
use crate::ast::operator::{OperationParser, OperationParserError};

speculate!{
    before {
        #[allow(unused)]
        let get_operator = |operator: HirOperator|
            HirOperationToken::Operator(operator);

        #[allow(unused)]
        let get_string_term = |s: &str|
            HirOperationToken::<HirOperator>::Term(
                HirExpression::Literal(
                    HirLiteral::String(s.to_string()),
                ),
            );

        #[allow(unused)]
        let get_operator_symbol = |operator_symbol: HirOperatorSymbol|
            HirOperationToken::Operator(operator_symbol);

        #[allow(unused)]
        let get_string_term_alt = |s: &str|
            HirOperationToken::<HirOperatorSymbol>::Term(
                HirExpression::Literal(
                    HirLiteral::String(s.to_string()),
                ),
            );
    }

    it "converts to postfix notation" {
        assert_eq!(
            // left: a = b + c * d + !(e?)
            OperationParser::parse(vec![
                get_string_term_alt("a"),
                get_operator_symbol(HirOperatorSymbol::Equal),
                get_string_term_alt("b"),
                get_operator_symbol(HirOperatorSymbol::Plus),
                get_string_term_alt("c"),
                get_operator_symbol(HirOperatorSymbol::Asterisk),
                get_string_term_alt("d"),
                get_operator_symbol(HirOperatorSymbol::Plus),
                get_operator_symbol(HirOperatorSymbol::Exclamation),
                get_operator_symbol(HirOperatorSymbol::LeftParenthesis),
                get_string_term_alt("e"),
                get_operator_symbol(HirOperatorSymbol::Question),
                get_operator_symbol(HirOperatorSymbol::RightParenthesis),
            ]),
            // right: a b c d * + e ? () ! + =
            Ok(vec![
                get_string_term("a"),
                get_string_term("b"),
                get_string_term("c"),
                get_string_term("d"),
                get_operator(HirOperator::Multiply),
                get_operator(HirOperator::Add),
                get_string_term("e"),
                get_operator(HirOperator::Propagate),
                get_operator(HirOperator::GroupBegin),
                get_operator(HirOperator::GroupEnd),
                get_operator(HirOperator::Not),
                get_operator(HirOperator::Add),
                get_operator(HirOperator::Substitute),
            ]),
        );
    }

    describe "operator symbol conversion" {
        describe "parenthesis operator" {
            it "converts at any position" {
                assert_eq!(
                    OperationParser::fix_operators(vec![
                        get_operator_symbol(HirOperatorSymbol::LeftParenthesis),
                        get_operator_symbol(HirOperatorSymbol::RightParenthesis),
                    ]),
                    Ok(vec![
                        get_operator(HirOperator::GroupBegin),
                        get_operator(HirOperator::GroupEnd),
                    ]),
                );
            }
        }

        describe "prefix operator" {
            it "converts at the start position" {
                assert_eq!(
                    OperationParser::fix_operators(vec![
                        get_operator_symbol(HirOperatorSymbol::Exclamation),
                    ]),
                    Ok(vec![
                        get_operator(HirOperator::Not),
                    ]),
                );
            }

            it "converts after infix operator" {
                assert_eq!(
                    OperationParser::fix_operators(vec![
                        get_string_term_alt("a"),
                        get_operator_symbol(HirOperatorSymbol::Plus),
                        get_operator_symbol(HirOperatorSymbol::Exclamation),
                    ]),
                    Ok(vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Add),
                        get_operator(HirOperator::Not),
                    ]),
                );
            }

            it "converts after parenthesis operator" {
                assert_eq!(
                    OperationParser::fix_operators(vec![
                        get_operator_symbol(HirOperatorSymbol::LeftParenthesis),
                        get_operator_symbol(HirOperatorSymbol::Exclamation),
                    ]),
                    Ok(vec![
                        get_operator(HirOperator::GroupBegin),
                        get_operator(HirOperator::Not),
                    ]),
                );
            }
        }

        describe "infix operator" {
            it "does not convert at the start position" {
                assert_eq!(
                    OperationParser::fix_operators(vec![
                        get_operator_symbol(HirOperatorSymbol::Plus),
                    ]),
                    Err(OperationParserError::UnknownOperatorAtThisPosition(HirOperatorSymbol::Plus)),
                );
            }

            it "converts after term" {
                assert_eq!(
                    OperationParser::fix_operators(vec![
                        get_string_term_alt("a"),
                        get_operator_symbol(HirOperatorSymbol::Plus),
                    ]),
                    Ok(vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Add),
                    ]),
                );
            }

            it "converts after postfix operator" {
                assert_eq!(
                    OperationParser::fix_operators(vec![
                        get_string_term_alt("a"),
                        get_operator_symbol(HirOperatorSymbol::Question),
                        get_operator_symbol(HirOperatorSymbol::Plus),
                    ]),
                    Ok(vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Propagate),
                        get_operator(HirOperator::Add),
                    ]),
                );
            }

            it "converts after parenthesis operator" {
                assert_eq!(
                    OperationParser::fix_operators(vec![
                        get_operator_symbol(HirOperatorSymbol::LeftParenthesis),
                        get_operator_symbol(HirOperatorSymbol::Plus),
                    ]),
                    Ok(vec![
                        get_operator(HirOperator::GroupBegin),
                        get_operator(HirOperator::Add),
                    ]),
                );
            }
        }

        describe "postfix operator" {
            it "does not convert at the start position" {
                assert_eq!(
                    OperationParser::fix_operators(vec![
                        get_operator_symbol(HirOperatorSymbol::Question),
                    ]),
                    Err(OperationParserError::UnknownOperatorAtThisPosition(HirOperatorSymbol::Question)),
                );
            }

            it "converts after term" {
                assert_eq!(
                    OperationParser::fix_operators(vec![
                        get_string_term_alt("a"),
                        get_operator_symbol(HirOperatorSymbol::Question),
                    ]),
                    Ok(vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Propagate),
                    ]),
                );
            }

            it "converts after postfix operator" {
                assert_eq!(
                    OperationParser::fix_operators(vec![
                        get_string_term_alt("a"),
                        get_operator_symbol(HirOperatorSymbol::Question),
                    ]),
                    Ok(vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Propagate),
                    ]),
                );
            }

            it "converts after parenthesis operator" {
                assert_eq!(
                    OperationParser::fix_operators(vec![
                        get_operator_symbol(HirOperatorSymbol::LeftParenthesis),
                        get_operator_symbol(HirOperatorSymbol::Question),
                    ]),
                    Ok(vec![
                        get_operator(HirOperator::GroupBegin),
                        get_operator(HirOperator::Propagate),
                    ]),
                );
            }
        }
    }

    describe "reverse polish notation" {
        it "arrange the order of operation terms" {
            assert_eq!(
                OperationParser::into_postfix_notation(
                    vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Add),
                        get_string_term("b"),
                    ],
                ),
                vec![
                    get_string_term("a"),
                    get_string_term("b"),
                    get_operator(HirOperator::Add),
                ],
            );
        }

        describe "infix operator" {
            it "the same precedence operator" {
                assert_eq!(
                    // left: a + b - c
                    OperationParser::into_postfix_notation(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Add),
                            get_string_term("b"),
                            get_operator(HirOperator::Subtract),
                            get_string_term("c"),
                        ],
                    ),
                    // right: a b + c -
                    vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_operator(HirOperator::Add),
                        get_string_term("c"),
                        get_operator(HirOperator::Subtract),
                    ],
                );
            }

            it "precedes higher precedence operator" {
                assert_eq!(
                    // left: a * b + c
                    OperationParser::into_postfix_notation(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Multiply),
                            get_string_term("b"),
                            get_operator(HirOperator::Add),
                            get_string_term("c"),
                        ],
                    ),
                    // right: a b * c +
                    vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_operator(HirOperator::Multiply),
                        get_string_term("c"),
                        get_operator(HirOperator::Add),
                    ],
                );

                assert_eq!(
                    // left: a + b * c
                    OperationParser::into_postfix_notation(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Add),
                            get_string_term("b"),
                            get_operator(HirOperator::Multiply),
                            get_string_term("c"),
                        ],
                    ),
                    // right: a b c * +
                    vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_string_term("c"),
                        get_operator(HirOperator::Multiply),
                        get_operator(HirOperator::Add),
                    ],
                );
            }

            it "reflects right-associativity" {
                assert_eq!(
                    // left: a = b + c
                    OperationParser::into_postfix_notation(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Substitute),
                            get_string_term("b"),
                            get_operator(HirOperator::Add),
                            get_string_term("c"),
                        ],
                    ),
                    // right: a b c + =
                    vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_string_term("c"),
                        get_operator(HirOperator::Add),
                        get_operator(HirOperator::Substitute),
                    ],
                );
            }

            it "group" {
                assert_eq!(
                    // left: a * (b + c)
                    OperationParser::into_postfix_notation(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Multiply),
                            get_operator(HirOperator::GroupBegin),
                            get_string_term("b"),
                            get_operator(HirOperator::Add),
                            get_string_term("c"),
                            get_operator(HirOperator::GroupEnd),
                        ],
                    ),
                    // right: a b c + () *
                    vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_string_term("c"),
                        get_operator(HirOperator::Add),
                        get_operator(HirOperator::GroupBegin),
                        get_operator(HirOperator::GroupEnd),
                        get_operator(HirOperator::Multiply),
                    ],
                );

                assert_eq!(
                    // left: a * (b + c) * d
                    OperationParser::into_postfix_notation(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Multiply),
                            get_operator(HirOperator::GroupBegin),
                            get_string_term("b"),
                            get_operator(HirOperator::Add),
                            get_string_term("c"),
                            get_operator(HirOperator::GroupEnd),
                            get_operator(HirOperator::Multiply),
                            get_string_term("d"),
                        ],
                    ),
                    // right: a b c + () * d *
                    vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_string_term("c"),
                        get_operator(HirOperator::Add),
                        get_operator(HirOperator::GroupBegin),
                        get_operator(HirOperator::GroupEnd),
                        get_operator(HirOperator::Multiply),
                        get_string_term("d"),
                        get_operator(HirOperator::Multiply),
                    ],
                );
            }
        }

        describe "prefix operator" {
            it "reverses the order of tokens" {
                assert_eq!(
                    // left: !a
                    OperationParser::into_postfix_notation(
                        vec![
                            get_operator(HirOperator::Not),
                            get_string_term("a"),
                        ],
                    ),
                    // right: a !
                    vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Not),
                    ],
                );
            }

            it "multiple" {
                assert_eq!(
                    // left: -!a
                    OperationParser::into_postfix_notation(
                        vec![
                            get_operator(HirOperator::Negative),
                            get_operator(HirOperator::Not),
                            get_string_term("a"),
                        ],
                    ),
                    // right: a ! -
                    vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Not),
                        get_operator(HirOperator::Negative),
                    ],
                );
            }

            it "mixes with infix operator" {
                assert_eq!(
                    // left: !a + !b
                    OperationParser::into_postfix_notation(
                        vec![
                            get_operator(HirOperator::Not),
                            get_string_term("a"),
                            get_operator(HirOperator::Add),
                            get_operator(HirOperator::Not),
                            get_string_term("b"),
                        ],
                    ),
                    // right: a ! b ! +
                    vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Not),
                        get_string_term("b"),
                        get_operator(HirOperator::Not),
                        get_operator(HirOperator::Add),
                    ],
                );
            }

            it "mixes with group operators" {
                assert_eq!(
                    // left: !(a + b)
                    OperationParser::into_postfix_notation(
                        vec![
                            get_operator(HirOperator::Not),
                            get_operator(HirOperator::GroupBegin),
                            get_string_term("a"),
                            get_operator(HirOperator::Add),
                            get_string_term("b"),
                            get_operator(HirOperator::GroupEnd),
                        ],
                    ),
                    // right: a b + () !
                    vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_operator(HirOperator::Add),
                        get_operator(HirOperator::GroupBegin),
                        get_operator(HirOperator::GroupEnd),
                        get_operator(HirOperator::Not),
                    ],
                );
            }
        }

        describe "postfix operator" {
            it "reverses the order of tokens" {
                assert_eq!(
                    // left: a!
                    OperationParser::into_postfix_notation(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Nonnize),
                        ],
                    ),
                    // right: a !
                    vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Nonnize),
                    ],
                );
            }

            it "multiple" {
                assert_eq!(
                    // left: a!?
                    OperationParser::into_postfix_notation(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Nonnize),
                            get_operator(HirOperator::Propagate),
                        ],
                    ),
                    // right: a ! ?
                    vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Nonnize),
                        get_operator(HirOperator::Propagate),
                    ],
                );
            }

            it "mixes with infix operator" {
                assert_eq!(
                    // left: a! + b
                    OperationParser::into_postfix_notation(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Nonnize),
                            get_operator(HirOperator::Add),
                            get_string_term("b"),
                        ],
                    ),
                    // right: a ! b +
                    vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Nonnize),
                        get_string_term("b"),
                        get_operator(HirOperator::Add),
                    ],
                );
            }

            it "group" {
                assert_eq!(
                    // left: (a + b)!
                    OperationParser::into_postfix_notation(
                        vec![
                            get_operator(HirOperator::GroupBegin),
                            get_string_term("a"),
                            get_operator(HirOperator::Add),
                            get_string_term("b"),
                            get_operator(HirOperator::GroupEnd),
                            get_operator(HirOperator::Nonnize),
                        ],
                    ),
                    // right: a b + () !
                    vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_operator(HirOperator::Add),
                        get_operator(HirOperator::GroupBegin),
                        get_operator(HirOperator::GroupEnd),
                        get_operator(HirOperator::Nonnize),
                    ],
                );
            }
        }

        describe "mixed operator" {
            it "" {
                assert_eq!(
                    // left: -a! + b * c
                    OperationParser::into_postfix_notation(
                        vec![
                            get_operator(HirOperator::Negative),
                            get_string_term("a"),
                            get_operator(HirOperator::Nonnize),
                            get_operator(HirOperator::Add),
                            get_string_term("b"),
                            get_operator(HirOperator::Multiply),
                            get_string_term("c"),
                        ],
                    ),
                    // right: a ! - b c * +
                    vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Nonnize),
                        get_operator(HirOperator::Negative),
                        get_string_term("b"),
                        get_string_term("c"),
                        get_operator(HirOperator::Multiply),
                        get_operator(HirOperator::Add),
                    ],
                );
            }
        }
    }
}
