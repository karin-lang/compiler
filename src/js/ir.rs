pub mod item;
pub mod stmt;

use self::item::JsItem;

#[derive(Clone, Debug, PartialEq)]
pub struct Js {
    pub items: Vec<JsItem>,
}
