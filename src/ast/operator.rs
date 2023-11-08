use crate::hir::ir::{expr::*, path::HirPath};

pub type OperationParserResult<T> = Result<T, OperationParserError>;

#[derive(Clone, Debug, PartialEq)]
pub enum OperationParserError {
    InvalidKindOfTerm,
    InvalidLengthOfTerm,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OperatorPrecedenceMode {
    InputPrecedence,
    StackPrecedence,
}

struct IndexedToken<T>(usize, T);

impl<T> IndexedToken<T> {
    fn new(index: usize, value: T) -> IndexedToken<T> {
        IndexedToken(index, value)
    }

    fn index(&self) -> usize {
        self.0
    }

    fn value(self) -> T {
        self.1
    }
}

pub struct OperationParser;

impl OperationParser {
    // 入力のtopとスタックのtopの優先度を比較する
    // 　入力側の優先度が高い：入力→スタック送り
    // 　スタック側の優先度が高い：スタック→出力送り
    // 　優先度が同じ：入力・スタックどちらも出力送り
    //   ※数値/IDの場合は必ずスタック送りなので入力→スタック送りして最適化
    // ※前置/中置/後置で重複した記号の演算子に注意（事前に演算子の位置を判断して分類する）
    // 比較が不可能なケースのエラーも実装
    pub fn parse(input: HirOperationSequence) -> OperationParserResult<HirExpression> {
        let output = OperationParser::into_postfix_notation(input)?;
        OperationParser::construct_expression(output)
    }

    pub fn into_postfix_notation(mut input: HirOperationSequence) -> OperationParserResult<HirOperationSequence> {
        input.reverse();
        let mut stack = Vec::new();
        let mut output = Vec::new();

        loop {
            let next_input = input.last();
            let next_stack = stack.last();

            // Finish when consumed all tokens.
            if next_input.is_none() && next_stack.is_none() {
                break Ok(output);
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
            HirOperator::FunctionCall(_) if is_input_mode => 11,
            HirOperator::FunctionCall(_) if is_stack_mode => 12,
            HirOperator::MemberAccess if is_input_mode => 13,
            HirOperator::MemberAccess if is_stack_mode => 14,
            HirOperator::Path if is_input_mode => 15,
            HirOperator::Path if is_stack_mode => 16,
            HirOperator::GroupBegin if is_input_mode => 17,
            HirOperator::GroupBegin if is_stack_mode => 1,
            HirOperator::GroupEnd if is_input_mode => 1,
            HirOperator::GroupEnd if is_stack_mode => unreachable!(),
            _ => unimplemented!(),
        }
    }

    pub fn construct_expression(input: HirOperationSequence) -> OperationParserResult<HirExpression> {
        let mut stack: Vec<IndexedToken<HirExpression>> = Vec::new();

        let pop_term = |stack: &mut Vec<IndexedToken<HirExpression>>| match stack.pop() {
            Some(v) => Ok(v),
            None => Err(OperationParserError::InvalidLengthOfTerm),
        };

        let pop_two_terms = |operator_index: i32, stack: &mut Vec<IndexedToken<HirExpression>>| {
            let term1 = pop_term(stack)?;
            let term2 = pop_term(stack)?;

            let indexed_terms = if term1.index() < term2.index() {
                (operator_index as usize, term1.value(), term2.value())
            } else {
                (operator_index as usize, term2.value(), term1.value())
            };

            Ok(indexed_terms)
        };

        let mut token_index = -1i32;

        for each_token in input {
            token_index += 1;

            let (output_token_index, operation) = match each_token {
                HirOperationToken::Operator(operator) => match operator {
                    HirOperator::Substitute => {
                        let (index, left, right) = pop_two_terms(token_index, &mut stack)?;
                        (index, HirOperation::Substitute(left, right))
                    },
                    HirOperator::Add => {
                        let (index, left, right) = pop_two_terms(token_index, &mut stack)?;
                        (index, HirOperation::Add(left, right))
                    },
                    HirOperator::Subtract => {
                        let (index, left, right) = pop_two_terms(token_index, &mut stack)?;
                        (index, HirOperation::Subtract(left, right))
                    },
                    HirOperator::Multiply => {
                        let (index, left, right) = pop_two_terms(token_index, &mut stack)?;
                        (index, HirOperation::Multiply(left, right))
                    },
                    HirOperator::Not => (token_index as usize, HirOperation::Not(pop_term(&mut stack)?.value())),
                    HirOperator::BitNot => (token_index as usize, HirOperation::BitNot(pop_term(&mut stack)?.value())),
                    HirOperator::Negative => (token_index as usize, HirOperation::Negative(pop_term(&mut stack)?.value())),
                    HirOperator::Nonnize => (token_index as usize, HirOperation::Nonnize(pop_term(&mut stack)?.value())),
                    HirOperator::Propagate => (token_index as usize, HirOperation::Propagate(pop_term(&mut stack)?.value())),
                    HirOperator::FunctionCall(arguments) => (token_index as usize, HirOperation::FunctionCall(pop_term(&mut stack)?.value(), arguments)),
                    HirOperator::MemberAccess => {
                        let (index, left, right) = pop_two_terms(token_index, &mut stack)?;
                        (index, HirOperation::MemberAccess(left, right))
                    },
                    HirOperator::Path => {
                        let (index, left, right) = pop_two_terms(token_index, &mut stack)?;

                        let mut segments =
                            if let HirExpression::Identifier(v) = left {
                                vec![v]
                            } else if let HirExpression::Operation(v) = left {
                                if let HirOperation::Path(HirPath::Unresolved(v)) = *v {
                                    v
                                } else {
                                    return Err(OperationParserError::InvalidKindOfTerm);
                                }
                            } else {
                                return Err(OperationParserError::InvalidKindOfTerm);
                            };

                        if let HirExpression::Identifier(v) = right {
                            segments.push(v);
                        } else {
                            return Err(OperationParserError::InvalidKindOfTerm);
                        }

                        (index, HirOperation::Path(HirPath::Unresolved(segments)))
                    },
                    HirOperator::GroupBegin => (token_index as usize, HirOperation::Group(pop_term(&mut stack)?.value())),
                    HirOperator::GroupEnd => continue,
                },
                HirOperationToken::Term(term) => {
                    stack.push(IndexedToken::new(token_index as usize, term));
                    continue;
                },
            };

            let new_operation = HirExpression::Operation(Box::new(operation));
            stack.push(IndexedToken(output_token_index, new_operation));
        }

        Ok(pop_term(&mut stack)?.value())
    }
}
