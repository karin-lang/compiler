use crate::hir::expr::*;

pub type OperationParserResult<T> = Result<T, OperationParserError>;

#[derive(Clone, Debug, PartialEq)]
pub enum OperationParserError {
    InvalidOperatorFix,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OperatorPrecedenceMode {
    InputPrecedence,
    StackPrecedence,
}

pub struct OperationParser;

impl OperationParser {
    // pub fn parse(input: HirOperationNew<HirOperatorSymbol>) -> HirOperationNew<HirOperator> {
    //     let fixed = OperationParser::fix_operators(input);
    //     OperationParser::into_postfix_notation(fixed)
    // }

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
                    HirOperationToken::Term(_) => {
                        // Send input token to output directly for performance.
                        output.push(input.pop().unwrap());
                        continue;
                    },
                },
                None => 0,
            };

            let stack_precedence = match next_stack {
                Some(v) => match v {
                    HirOperationToken::Operator(operator) => OperationParser::get_operator_precedence(operator, OperatorPrecedenceMode::StackPrecedence) + 1,
                    HirOperationToken::Term(_) => unreachable!("input must already be sent to output"),
                },
                None => 0,
            };

            if input_precedence <= stack_precedence {
                output.push(stack.pop().unwrap());
            }

            if stack_precedence <= input_precedence {
                stack.push(input.pop().unwrap());
            }
        }
    }

    /*
    pub fn fix_operators(input: HirOperation<HirOperatorSymbol>) -> OperationParserResult<HirOperation<HirOperator>> {
        let mut output = Vec::new();
        // Recognize the first token as a prefix operator when it was an operator.
        let mut was_operator = true;

        // !!a+!a
        // a.a
        // a+a!!
        // a!+!a

        for each_token in input {
            match each_token {
                HirOperationToken::Term(term) => {
                    was_operator = false;
                    output.push(HirOperationToken::Term(term));
                },
                HirOperationToken::Operator(operator) => {
                    let fixed = if was_operator {
                        operator.fix(HirOperatorFix::Prefix)
                    } else {
                        if postfix {
                            operator.fix(HirOperatorFix::Infix)
                        } else {
                            was_operator = true;
                            operator.fix(HirOperatorFix::Infix)
                        }
                    };

                    match fixed {
                        Some(v) => output.push(HirOperationToken::Operator(v)),
                        None => return Err(OperationParserError::InvalidOperatorFix),
                    }
                },
            }
        }

        Ok(output)
    }
    */

    pub fn get_operator_precedence(operator: &HirOperator, mode: OperatorPrecedenceMode) -> usize {
        let is_input_mode = mode == OperatorPrecedenceMode::InputPrecedence;
        let is_stack_mode = mode == OperatorPrecedenceMode::StackPrecedence;

        match operator {
            HirOperator::Substitute if is_input_mode => 3,
            HirOperator::Substitute if is_stack_mode => 2,
            HirOperator::Add if is_input_mode => 4,
            HirOperator::Add if is_stack_mode => 5,
            HirOperator::Subtract if is_input_mode => 4,
            HirOperator::Subtract if is_stack_mode => 5,
            HirOperator::Multiply if is_input_mode => 6,
            HirOperator::Multiply if is_stack_mode => 7,
            HirOperator::GroupBegin if is_input_mode => 8,
            HirOperator::GroupBegin if is_stack_mode => 0,
            HirOperator::GroupEnd if is_input_mode => 1,
            HirOperator::GroupEnd if is_stack_mode => 1,
            _ => unimplemented!(),
        }
    }
}
