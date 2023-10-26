use std::collections::HashMap;

pub trait JsElement {
    fn code(&self) -> String;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Js {
    pub items: HashMap<String, JsItem>,
}

impl JsElement for Js {
    fn code(&self) -> String {
        self.items.iter().map(|v| v.code()).collect::<Vec<String>>().join("")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsItem {
    Function(JsFunction),
}

impl JsElement for (&String, &JsItem) {
    fn code(&self) -> String {
        match &self.1 {
            JsItem::Function(function) => function.code(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct JsFunction {
    pub id: String,
    pub arguments: Vec<String>,
    pub statements: Vec<JsStatement>,
}

impl JsElement for JsFunction {
    fn code(&self) -> String {
        format!(
            "function {id}({args}){{{stmts}}}",
            id = self.id,
            args = self.arguments.join(","),
            stmts = self.statements.iter().map(|v| v.code()).collect::<Vec<String>>().join(";"),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsStatement {
    Expression(JsExpression),
}

impl JsElement for JsStatement {
    fn code(&self) -> String {
        match self {
            JsStatement::Expression(expr) => expr.code(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsExpression {
    Literal(JsLiteral),
}

impl JsElement for JsExpression {
    fn code(&self) -> String {
        match self {
            JsExpression::Literal(literal) => literal.code(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsLiteral {
    Boolean(bool),
    // todo: add JsIntegerLiteral
    Integer(u64),
}

impl JsElement for JsLiteral {
    fn code(&self) -> String {
        match self {
            JsLiteral::Boolean(boolean) => boolean.to_string(),
            JsLiteral::Integer(integer) => integer.to_string(),
        }
    }
}
