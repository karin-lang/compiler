#[derive(Clone, Debug, PartialEq)]
pub enum JsStatement {
    Expression(JsExpression),
}

impl Into<JsExpression> for JsStatement {
    fn into(self) -> JsExpression {
        if let JsStatement::Expression(expr) = self {
            expr
        } else {
            unreachable!("expected js expression");
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsExpression {
    Literal(JsLiteral),
    Operation(Box<JsOperation>),
    Identifier(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsLiteral {
    Boolean(bool),
    // todo: add JsIntegerLiteral
    Integer(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsOperation {
    Add(JsExpression, JsExpression),
    Subtract(JsExpression, JsExpression),
    Multiply(JsExpression, JsExpression),
    Not(JsExpression),
    BitNot(JsExpression),
    Negative(JsExpression),
    FunctionCall(JsExpression, Vec<JsExpression>),
    MemberAccess(JsExpression, JsExpression),
    Group(JsExpression),
}
