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
            HirOperationToken::Term(
                HirExpression::Literal(
                    HirLiteral::String(s.to_string()),
                ),
            );
    }

    describe "parsing" {
        it "arrange the order of operation terms" {
            assert_eq!(
                OperationParser::parse(
                    vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Add),
                        get_string_term("b"),
                    ],
                ),
                Ok(vec![
                    get_string_term("a"),
                    get_string_term("b"),
                    get_operator(HirOperator::Add),
                ]),
            );
        }

        describe "infix operator" {
            it "the same precedence operator" {
                assert_eq!(
                    // left: a + b - c
                    OperationParser::parse(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Add),
                            get_string_term("b"),
                            get_operator(HirOperator::Subtract),
                            get_string_term("c"),
                        ],
                    ),
                    // right: a b + c -
                    Ok(vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_operator(HirOperator::Add),
                        get_string_term("c"),
                        get_operator(HirOperator::Subtract),
                    ]),
                );
            }

            it "precedes higher precedence operator" {
                assert_eq!(
                    // left: a * b + c
                    OperationParser::parse(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Multiply),
                            get_string_term("b"),
                            get_operator(HirOperator::Add),
                            get_string_term("c"),
                        ],
                    ),
                    // right: a b * c +
                    Ok(vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_operator(HirOperator::Multiply),
                        get_string_term("c"),
                        get_operator(HirOperator::Add),
                    ]),
                );

                assert_eq!(
                    // left: a + b * c
                    OperationParser::parse(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Add),
                            get_string_term("b"),
                            get_operator(HirOperator::Multiply),
                            get_string_term("c"),
                        ],
                    ),
                    // right: a b c * +
                    Ok(vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_string_term("c"),
                        get_operator(HirOperator::Multiply),
                        get_operator(HirOperator::Add),
                    ]),
                );
            }

            it "reflects right-associativity" {
                assert_eq!(
                    // left: a = b + c
                    OperationParser::parse(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Substitute),
                            get_string_term("b"),
                            get_operator(HirOperator::Add),
                            get_string_term("c"),
                        ],
                    ),
                    // right: a b c + =
                    Ok(vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_string_term("c"),
                        get_operator(HirOperator::Add),
                        get_operator(HirOperator::Substitute),
                    ]),
                );
            }

            it "group" {
                assert_eq!(
                    // left: a * (b + c)
                    OperationParser::parse(
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
                    Ok(vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_string_term("c"),
                        get_operator(HirOperator::Add),
                        get_operator(HirOperator::GroupBegin),
                        get_operator(HirOperator::GroupEnd),
                        get_operator(HirOperator::Multiply),
                    ]),
                );

                assert_eq!(
                    // left: a * (b + c) * d
                    OperationParser::parse(
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
                    Ok(vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_string_term("c"),
                        get_operator(HirOperator::Add),
                        get_operator(HirOperator::GroupBegin),
                        get_operator(HirOperator::GroupEnd),
                        get_operator(HirOperator::Multiply),
                        get_string_term("d"),
                        get_operator(HirOperator::Multiply),
                    ]),
                );
            }
        }

        describe "prefix operator" {
            it "reverses the order of tokens" {
                assert_eq!(
                    // left: !a
                    OperationParser::parse(
                        vec![
                            get_operator(HirOperator::Not),
                            get_string_term("a"),
                        ],
                    ),
                    // right: a !
                    Ok(vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Not),
                    ]),
                );
            }

            it "multiple" {
                assert_eq!(
                    // left: -!a
                    OperationParser::parse(
                        vec![
                            get_operator(HirOperator::Negative),
                            get_operator(HirOperator::Not),
                            get_string_term("a"),
                        ],
                    ),
                    // right: a ! -
                    Ok(vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Not),
                        get_operator(HirOperator::Negative),
                    ]),
                );
            }

            it "mixes with infix operator" {
                assert_eq!(
                    // left: !a + !b
                    OperationParser::parse(
                        vec![
                            get_operator(HirOperator::Not),
                            get_string_term("a"),
                            get_operator(HirOperator::Add),
                            get_operator(HirOperator::Not),
                            get_string_term("b"),
                        ],
                    ),
                    // right: a ! b ! +
                    Ok(vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Not),
                        get_string_term("b"),
                        get_operator(HirOperator::Not),
                        get_operator(HirOperator::Add),
                    ]),
                );
            }

            it "mixes with group operators" {
                assert_eq!(
                    // left: !(a + b)
                    OperationParser::parse(
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
                    Ok(vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_operator(HirOperator::Add),
                        get_operator(HirOperator::GroupBegin),
                        get_operator(HirOperator::GroupEnd),
                        get_operator(HirOperator::Not),
                    ]),
                );
            }
        }

        describe "postfix operator" {
            it "reverses the order of tokens" {
                assert_eq!(
                    // left: a!
                    OperationParser::parse(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Nonnize),
                        ],
                    ),
                    // right: a !
                    Ok(vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Nonnize),
                    ]),
                );
            }

            it "multiple" {
                assert_eq!(
                    // left: a!?
                    OperationParser::parse(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Nonnize),
                            get_operator(HirOperator::Propagate),
                        ],
                    ),
                    // right: a ! ?
                    Ok(vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Nonnize),
                        get_operator(HirOperator::Propagate),
                    ]),
                );
            }

            it "mixes with infix operator" {
                assert_eq!(
                    // left: a! + b
                    OperationParser::parse(
                        vec![
                            get_string_term("a"),
                            get_operator(HirOperator::Nonnize),
                            get_operator(HirOperator::Add),
                            get_string_term("b"),
                        ],
                    ),
                    // right: a ! b +
                    Ok(vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Nonnize),
                        get_string_term("b"),
                        get_operator(HirOperator::Add),
                    ]),
                );
            }

            it "group" {
                assert_eq!(
                    // left: (a + b)!
                    OperationParser::parse(
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
                    Ok(vec![
                        get_string_term("a"),
                        get_string_term("b"),
                        get_operator(HirOperator::Add),
                        get_operator(HirOperator::GroupBegin),
                        get_operator(HirOperator::GroupEnd),
                        get_operator(HirOperator::Nonnize),
                    ]),
                );
            }
        }

        describe "mixed operator" {
            it "" {
                assert_eq!(
                    // left: -a! + b * c
                    OperationParser::parse(
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
                    Ok(vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Nonnize),
                        get_operator(HirOperator::Negative),
                        get_string_term("b"),
                        get_string_term("c"),
                        get_operator(HirOperator::Multiply),
                        get_operator(HirOperator::Add),
                    ]),
                );
            }
        }
    }

    describe "precedence index" {
        // todo: add test case to test group precedence index
    }
}
