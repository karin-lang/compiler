use volt::{*, element::*};
use volt_derive::VoltModuleDefinition;

use super::{*, expr::Expression};

#[derive(VoltModuleDefinition)]
pub(super) struct Item {
    item: Element,
}

impl VoltModule for Item {
    fn new() -> Item {
        define_rules!{
            item := choice![Function::function()];
        }
    }
}

#[derive(VoltModuleDefinition)]
pub(super) struct Function {
    function: Element,
    formal_argument: Element,
}

impl VoltModule for Function {
    fn new() -> Function {
        define_rules!{
            function := seq![
                seq![Main::accessibility(), WHITESPACE()].optional(),
                str("fn").hide(), WHITESPACE(),
                Identifier::identifier(), WHITESPACE(),
                str("(").hide(), WHITESPACE(),
                Function::formal_argument().separate(str(",").separate_around(WHITESPACE()).hide()).optional().group("args"), WHITESPACE(),
                str(")").hide(), WHITESPACE(),
                str("{").hide(), WHITESPACE(),
                Expression::expression().separate_around(Symbol::expression_separator().min(0).hide()).optional().group("exprs"), WHITESPACE(),
                str("}").hide(),
            ];
            formal_argument := seq![Identifier::identifier(), WHITESPACE(), DataType::data_type()];
        }
    }
}
