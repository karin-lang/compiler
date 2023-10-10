use std::collections::HashMap;

pub trait JsElement {
    fn code(&self) -> String;
}

pub struct Js {
    pub items: HashMap<Vec<String>, JsItem>,
}

impl JsElement for Js {
    fn code(&self) -> String {
        self.items.iter().map(|v| v.code()).collect::<Vec<String>>().join("")
    }
}

pub enum JsItem {
    Function(JsFunction),
}

impl JsElement for (&Vec<String>, &JsItem) {
    fn code(&self) -> String {
        match &self.1 {
            JsItem::Function(function) => function.code(),
        }
    }
}

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
