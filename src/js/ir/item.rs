use super::stmt::JsStatement;

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
