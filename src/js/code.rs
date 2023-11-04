use crate::js::ir::*;

pub struct JsCodeGenerator {}

impl JsCodeGenerator {
    pub fn generate(js: &Js) -> String {
        js.items.iter().map(|v| JsCodeGenerator::item(v)).collect::<Vec<String>>().join("")
    }

    pub fn item(item: &JsItem) -> String {
        match item {
            JsItem::Function(function) => JsCodeGenerator::function(function),
        }
    }

    pub fn function(function: &JsFunction) -> String {
        format!(
            "function {id}({args}){{{stmts}}}",
            id = function.id,
            args = function.arguments.join(","),
            stmts = function.statements.iter().map(|v| JsCodeGenerator::statement(v)).collect::<Vec<String>>().join(";"),
        )
    }

    pub fn statement(statement: &JsStatement) -> String {
        match statement {
            JsStatement::Expression(expr) => JsCodeGenerator::expression(expr),
        }
    }

    pub fn expression(expression: &JsExpression) -> String {
        match expression {
            JsExpression::Literal(literal) => JsCodeGenerator::literal(literal),
            JsExpression::Operation(operation) => JsCodeGenerator::operation(operation),
        }
    }

    pub fn literal(literal: &JsLiteral) -> String {
        match literal {
            JsLiteral::Boolean(boolean) => boolean.to_string(),
            JsLiteral::Integer(integer) => integer.to_string(),
        }
    }

    pub fn operation(operation: &JsOperation) -> String {
        match operation {
            JsOperation::Add(left, right) => format!("{}+{}", JsCodeGenerator::expression(left), JsCodeGenerator::expression(right)),
            JsOperation::Subtract(left, right) => format!("{}-{}", JsCodeGenerator::expression(left), JsCodeGenerator::expression(right)),
            JsOperation::Multiply(left, right) => format!("{}*{}", JsCodeGenerator::expression(left), JsCodeGenerator::expression(right)),
            JsOperation::Not(term) => format!("!{}", JsCodeGenerator::expression(term)),
            JsOperation::BitNot(term) => format!("~{}", JsCodeGenerator::expression(term)),
            JsOperation::Negative(term) => format!("-{}", JsCodeGenerator::expression(term)),
            JsOperation::MemberAccess(left, right) => format!("{}.{}", JsCodeGenerator::expression(left), JsCodeGenerator::expression(right)),
            JsOperation::Group(term) => format!("({})", JsCodeGenerator::expression(term)),
        }
    }
}
