#[derive(Clone, Debug, PartialEq)]
pub struct Js {
    pub items: Vec<JsItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsItem {
    Function(JsFunction),
}

#[derive(Clone, Debug, PartialEq)]
pub struct JsFunction {
    pub id: String,
    pub arguments: Vec<String>,
    pub statements: Vec<JsStatement>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsStatement {
    Expression(JsExpression),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsExpression {
    Literal(JsLiteral),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsLiteral {
    Boolean(bool),
    // todo: add JsIntegerLiteral
    Integer(u64),
}
