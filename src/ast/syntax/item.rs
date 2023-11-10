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
            item := choice![UseDeclaration::use_declaration(), Function::function()];
        }
    }
}

#[derive(VoltModuleDefinition)]
pub(super) struct UseDeclaration {
    use_declaration: Element,
}

impl VoltModule for UseDeclaration {
    fn new() -> UseDeclaration {
        define_rules!{
            use_declaration := seq![
                str("use").hide(), WHITESPACE_REQUIRED(),
                choice![
                    str("hako"),
                    str("self"),
                    Identifier::identifier().expand_once(),
                ].separate(seq![WHITESPACE(), str("::").hide(), WHITESPACE()]),
            ];
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
                DataType::data_type().optional(), WHITESPACE(),
                str("{").hide(), WHITESPACE(),
                Expression::expression().separate_around(Symbol::expression_separator().min(0).hide()).optional().group("exprs"), WHITESPACE(),
                str("}").hide(),
            ];
            formal_argument := seq![
                seq![str("mut"), WHITESPACE_REQUIRED()].optional(),
                choice![
                    str("self"),
                    seq![
                        Identifier::identifier(),
                        WHITESPACE_REQUIRED(),
                        DataType::data_type(),
                    ],
                ],
            ];
        }
    }
}
