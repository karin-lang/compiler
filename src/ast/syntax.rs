pub mod expr;
pub mod item;

use volt::{*, element::*};
use volt_derive::VoltModuleDefinition;
use self::{item::*, expr::{*, Expression}};

pub struct Syntax;

impl Syntax {
    pub fn generate_volt(max_recursion: usize) -> Volt {
        let mut volt = Volt::new();
        volt.set_max_recursion(max_recursion);
        volt.add_module(Main::new());
        volt.add_module(Symbol::new());
        volt.add_module(Identifier::new());
        volt.add_module(Item::new());
        volt.add_module(Function::new());
        volt.add_module(Expression::new());
        volt.add_module(Literal::new());
        volt.add_module(Operation::new());
        volt.add_module(DataType::new());
        volt
    }
}

const WHITESPACE: fn() -> Element = || Symbol::whitespace().min(0).hide();

#[derive(VoltModuleDefinition)]
struct Main {
    main: Element,
    item: Element,
    accessibility: Element,
}

impl VoltModule for Main {
    fn new() -> Main {
        define_rules!{
            main := choice![Item::item().separate_around(WHITESPACE()), WHITESPACE()];
            item := choice![Function::function()];
            accessibility := choice![str("pub@hako"), str("pub")].optional();
        }
    }
}

#[derive(VoltModuleDefinition)]
struct Symbol {
    whitespace: Element,
    expression_separator: Element,
    around_expression_separator: Element,
}

impl VoltModule for Symbol {
    fn new() -> Symbol {
        define_rules!{
            whitespace := choice![str(" "), str("\t"), str("\n")];
            expression_separator := choice![str("\n"), str(";")].around(Symbol::around_expression_separator().min(0));
            around_expression_separator := choice![str(" "), str("\t")];
        }
    }
}

#[derive(VoltModuleDefinition)]
struct Identifier {
    identifier: Element,
    reserved: Element,
}

impl VoltModule for Identifier {
    fn new() -> Identifier {
        define_rules!{
            identifier := choice![
                // identifier which is not reserved keyword like "id"
                seq![Identifier::reserved().neglook(), seq![chars(r"a-zA-Z_"), chars(r"a-zA-Z\d_").min(0)].join()],
                // reserved keyword which is followed by characters like "public" (not "pub")
                seq![Identifier::reserved(), chars(r"a-zA-Z\d_").min(1)].join(),
            ];
            reserved := choice![
                str("bool"), str("char"), str("fn"), str("hako"), str("none"), str("pub"), str("str"),
                Literal::boolean(),
                DataType::primitive_number(),
            ];
        }
    }
}
