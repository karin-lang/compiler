use crate::hir::expr::*;

pub type OperationParserResult<T> = Result<T, OperationParserError>;

#[derive(Clone, Debug, PartialEq)]
pub enum OperationParserError {
    UnknownOperatorAtThisPosition(HirOperatorSymbol),
}

#[derive(Clone, Debug, PartialEq)]
pub enum OperatorPrecedenceMode {
    InputPrecedence,
    StackPrecedence,
}

#[derive(Clone, Debug, PartialEq)]
enum OperationTokenKind {
    Initial,
    Term,
    PrefixOperator,
    InfixOperator,
    PostfixOperator,
    ParenthesisOperator,
}

pub struct OperationParser;

impl OperationParser {
    pub fn parse(input: HirOperationNew<HirOperatorSymbol>) -> OperationParserResult<HirOperationNew<HirOperator>> {
        let fixed = OperationParser::fix_operators(input)?;
        Ok(OperationParser::into_postfix_notation(fixed))
    }

    // 入力のtopとスタックのtopの優先度を比較する
    // 　入力側の優先度が高い：入力→スタック送り
    // 　スタック側の優先度が高い：スタック→出力送り
    // 　優先度が同じ：入力・スタックどちらも出力送り
    //   ※数値/IDの場合は必ずスタック送りなので入力→スタック送りして最適化
    // ※前置/中置/後置で重複した記号の演算子に注意（事前に演算子の位置を判断して分類する）
    // 比較が不可能なケースのエラーも実装
    pub fn into_postfix_notation(mut input: HirOperationNew<HirOperator>) -> HirOperationNew<HirOperator> {
        input.reverse();
        let mut stack = Vec::new();
        let mut output = Vec::new();

        loop {
            let next_input = input.last();
            let next_stack = stack.last();

            // Finish when consumed all tokens.
            if next_input.is_none() && next_stack.is_none() {
                break output;
            }

            let input_precedence = match next_input {
                Some(v) => match v {
                    HirOperationToken::Operator(operator) => OperationParser::get_operator_precedence(operator, OperatorPrecedenceMode::InputPrecedence) + 1,
                    HirOperationToken::Term(_) => usize::MAX,
                },
                None => 0,
            };

            let stack_precedence = match next_stack {
                Some(v) => match v {
                    HirOperationToken::Operator(operator) => OperationParser::get_operator_precedence(operator, OperatorPrecedenceMode::StackPrecedence) + 1,
                    HirOperationToken::Term(_) => usize::MAX,
                },
                None => 0,
            };

            if input_precedence < stack_precedence {
                output.push(stack.pop().unwrap());
            } else if stack_precedence < input_precedence {
                stack.push(input.pop().unwrap());
            } else {
                output.push(stack.pop().unwrap());
                output.push(input.pop().unwrap());
            }
        }
    }

    pub fn get_operator_precedence(operator: &HirOperator, mode: OperatorPrecedenceMode) -> usize {
        let is_input_mode = mode == OperatorPrecedenceMode::InputPrecedence;
        let is_stack_mode = mode == OperatorPrecedenceMode::StackPrecedence;

        match operator {
            HirOperator::Substitute if is_input_mode => 4,
            HirOperator::Substitute if is_stack_mode => 3,
            HirOperator::Add if is_input_mode => 5,
            HirOperator::Add if is_stack_mode => 6,
            HirOperator::Subtract if is_input_mode => 5,
            HirOperator::Subtract if is_stack_mode => 6,
            HirOperator::Multiply if is_input_mode => 7,
            HirOperator::Multiply if is_stack_mode => 8,
            HirOperator::Negative if is_input_mode => 10,
            HirOperator::Negative if is_stack_mode => 9,
            HirOperator::Not if is_input_mode => 10,
            HirOperator::Not if is_stack_mode => 9,
            HirOperator::Nonnize if is_input_mode => 11,
            HirOperator::Nonnize if is_stack_mode => 12,
            HirOperator::Propagate if is_input_mode => 11,
            HirOperator::Propagate if is_stack_mode => 12,
            HirOperator::GroupBegin if is_input_mode => 13,
            HirOperator::GroupBegin if is_stack_mode => 1,
            HirOperator::GroupEnd if is_input_mode => 1,
            HirOperator::GroupEnd if is_stack_mode => unreachable!(),
            _ => unimplemented!(),
        }
    }

    pub fn fix_operators(input: HirOperationNew<HirOperatorSymbol>) -> OperationParserResult<HirOperationNew<HirOperator>> {
        let mut output = Vec::new();
        let mut latest_token_kind = OperationTokenKind::Initial;

        for each_token in input {
            // term parsing
            let operator_symbol = match each_token {
                HirOperationToken::Operator(operator) => operator,
                HirOperationToken::Term(term) => {
                    latest_token_kind = OperationTokenKind::Term;
                    output.push(HirOperationToken::Term(term));
                    continue;
                },
            };

            // parenthesis operator parsing
            if let Some(operator) = operator_symbol.to_operator(HirOperatorFix::Parenthesis) {
                latest_token_kind = OperationTokenKind::ParenthesisOperator;
                output.push(HirOperationToken::Operator(operator));
                continue;
            }

            // prefix operator parsing
            if latest_token_kind == OperationTokenKind::Initial || latest_token_kind == OperationTokenKind::InfixOperator || latest_token_kind == OperationTokenKind::ParenthesisOperator {
                if let Some(operator) = operator_symbol.to_operator(HirOperatorFix::Prefix) {
                    latest_token_kind = OperationTokenKind::PrefixOperator;
                    output.push(HirOperationToken::Operator(operator));
                    continue;
                }
            }

            // infix operator parsing
            if latest_token_kind == OperationTokenKind::Term || latest_token_kind == OperationTokenKind::PostfixOperator || latest_token_kind == OperationTokenKind::ParenthesisOperator {
                if let Some(operator) = operator_symbol.to_operator(HirOperatorFix::Infix) {
                    latest_token_kind = OperationTokenKind::InfixOperator;
                    output.push(HirOperationToken::Operator(operator));
                    continue;
                }
            }

            // postfix operator parsing
            if latest_token_kind == OperationTokenKind::Term || latest_token_kind == OperationTokenKind::PostfixOperator || latest_token_kind == OperationTokenKind::ParenthesisOperator {
                if let Some(operator) = operator_symbol.to_operator(HirOperatorFix::Postfix) {
                    latest_token_kind = OperationTokenKind::PostfixOperator;
                    output.push(HirOperationToken::Operator(operator));
                    continue;
                }
            }

            return Err(OperationParserError::UnknownOperatorAtThisPosition(operator_symbol));
        }

        Ok(output)
    }
}
