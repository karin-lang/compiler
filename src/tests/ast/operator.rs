use speculate::speculate;
use crate::hir::expr::*;
use crate::ast::operator::OperationParser;

speculate!{
    before {
        let get_string_term = |s: &str|
            HirOperationToken::Term(
                HirExpression::Literal(
                    HirLiteral::String(s.to_string()),
                ),
            );

        let get_operator = |operator: HirOperator|
            HirOperationToken::Operator(operator);
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

        it "the same precedence operator" {
            assert_eq!(
                OperationParser::into_postfix_notation(
                    vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Add),
                        get_string_term("b"),
                        get_operator(HirOperator::Subtract),
                        get_string_term("c"),
                    ],
                ),
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
                OperationParser::into_postfix_notation(
                    vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Multiply),
                        get_string_term("b"),
                        get_operator(HirOperator::Add),
                        get_string_term("c"),
                    ],
                ),
                vec![
                    get_string_term("a"),
                    get_string_term("b"),
                    get_operator(HirOperator::Multiply),
                    get_string_term("c"),
                    get_operator(HirOperator::Add),
                ],
            );

            assert_eq!(
                OperationParser::into_postfix_notation(
                    vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Add),
                        get_string_term("b"),
                        get_operator(HirOperator::Multiply),
                        get_string_term("c"),
                    ],
                ),
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
                OperationParser::into_postfix_notation(
                    vec![
                        get_string_term("a"),
                        get_operator(HirOperator::Substitute),
                        get_string_term("b"),
                        get_operator(HirOperator::Add),
                        get_string_term("c"),
                    ],
                ),
                vec![
                    get_string_term("a"),
                    get_string_term("b"),
                    get_string_term("c"),
                    get_operator(HirOperator::Add),
                    get_operator(HirOperator::Substitute),
                ],
            );
        }

        it "" {
            assert_eq!(
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
                vec![
                    get_string_term("a"),
                    get_string_term("b"),
                    get_string_term("c"),
                    get_operator(HirOperator::Add),
                    get_operator(HirOperator::GroupEnd),
                    get_operator(HirOperator::GroupBegin),
                    get_operator(HirOperator::Multiply),
                ],
            );
        }
    }
}
