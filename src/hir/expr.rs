use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum HirExpression {
    Literal(HirLiteral),
    Operation(Box<HirOperation>),
    DataType(HirDataType),
    Identifier(HirIdentifier),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirLiteral {
    Boolean(bool),
    Integer(HirIntegerLiteral),
    Float(HirFloatLiteral),
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirIntegerBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirIntegerLiteral {
    pub data_type: Option<HirPrimitiveDataType>,
    pub base: HirIntegerBase,
    pub value: String,
    pub exponent: Option<HirIntegerExponent>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirIntegerExponent {
    pub positive: bool,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirFloatLiteral {
    pub data_type: Option<HirPrimitiveDataType>,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirOperation {
    Add(HirExpression, HirExpression),
    Multiply(HirExpression, HirExpression),
    Negative(HirExpression),
    Not(HirExpression),
    BitNot(HirExpression),
    Nonnize(HirExpression),
    Propagate(HirExpression),
    MemberAccess(HirExpression, HirExpression),
    Path(HirPath),
    Group(HirExpression),
}

pub type HirOperationNew<Operator> = Vec<HirOperationToken<Operator>>;

#[derive(Clone, Debug, PartialEq)]
pub enum HirOperationToken<Operator> {
    Operator(Operator),
    Term(HirExpression),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirOperatorFix {
    Prefix,
    Infix,
    Postfix,
    Parenthesis,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirOperator {
    Substitute,
    Add,
    Subtract,
    Multiply,
    Negative,
    Not,
    BitNot,
    Nonnize,
    Propagate,
    MemberAccess,
    Path,
    GroupBegin,
    GroupEnd,
}

// Please modify the match patterns in HirOperatorSymbol::to_operator() when operator symbols changed.
#[derive(Clone, Debug, PartialEq)]
pub enum HirOperatorSymbol {
    Asterisk,
    Dot,
    DoubleColon,
    Equal,
    Exclamation,
    Minus,
    LeftParenthesis,
    RightParenthesis,
    Plus,
    Question,
    Tilde,
}

impl HirOperatorSymbol {
    pub fn to_operator(&self, fix: HirOperatorFix) -> Option<HirOperator> {
        let operator = match fix {
            HirOperatorFix::Prefix => match self {
                HirOperatorSymbol::Exclamation => HirOperator::Not,
                HirOperatorSymbol::Minus => HirOperator::Negative,
                HirOperatorSymbol::Tilde => HirOperator::BitNot,
                _ => return None,
            },
            HirOperatorFix::Infix => match self {
                HirOperatorSymbol::Asterisk => HirOperator::Multiply,
                HirOperatorSymbol::Dot => HirOperator::MemberAccess,
                HirOperatorSymbol::DoubleColon => HirOperator::Path,
                HirOperatorSymbol::Equal => HirOperator::Substitute,
                HirOperatorSymbol::Minus => HirOperator::Subtract,
                HirOperatorSymbol::Plus => HirOperator::Add,
                _ => return None,
            },
            HirOperatorFix::Postfix => match self {
                HirOperatorSymbol::Exclamation => HirOperator::Nonnize,
                HirOperatorSymbol::Question => HirOperator::Propagate,
                _ => return None,
            },
            HirOperatorFix::Parenthesis => match self {
                HirOperatorSymbol::LeftParenthesis => HirOperator::GroupBegin,
                HirOperatorSymbol::RightParenthesis => HirOperator::GroupEnd,
                _ => return None,
            },
        };

        Some(operator)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirDataType {
    Primitive(HirPrimitiveDataType),
    Generic(HirIdentifierBinding<HirGenericDataType>),
    Identifier(HirIdentifier),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HirPrimitiveDataType {
    Usize,
    F32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HirGenericDataType {
    pub arguments: Vec<HirDataType>,
}
