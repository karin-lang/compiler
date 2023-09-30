use volt::{*, element::*, rule::*};
use volt_derive::VoltModuleDefinition;

pub struct Syntax;

impl Syntax {
    pub fn generate_volt(max_recursion: usize) -> Volt {
        let mut volt = Volt::new();
        volt.set_max_recursion(max_recursion);
        volt.add_module(Main::new());
        volt.add_module(Symbol::new());
        volt.add_module(Identifier::new());
        volt.add_module(Function::new());
        volt.add_module(DataType::new());
        volt
    }
}

const WHITESPACE: fn() -> Element = || Symbol::whitespace().min(0).hide();

#[derive(VoltModuleDefinition)]
struct Main {
    main: Element,
    item: Element,
}

impl VoltModule for Main {
    fn new() -> Main {
        define_rules!{
            main := choice![Main::item().separate_around(WHITESPACE()), WHITESPACE()];
            item := choice![Function::function()];
        }
    }
}

#[derive(VoltModuleDefinition)]
struct Symbol {
    statement_end: Element,
    statement_end_separator: Element,
    whitespace: Element,
}

impl VoltModule for Symbol {
    fn new() -> Symbol {
        define_rules!{
            statement_end := choice![str("\n"), str(";")].separate_around(Symbol::statement_end_separator().min(0));
            statement_end_separator := choice![str(" "), str("\t")];
            whitespace := choice![str(" "), str("\t"), str("\n")];
        }
    }
}

#[derive(VoltModuleDefinition)]
struct Identifier {
    identifier: Element,
}

impl VoltModule for Identifier {
    fn new() -> Identifier {
        define_rules!{
            identifier := seq![chars(r"a-zA-Z_"), chars(r"a-zA-Z\d_").min(0)].join();
        }
    }
}

#[derive(VoltModuleDefinition)]
struct Function {
    function: Element,
    argument: Element,
}

impl VoltModule for Function {
    fn new() -> Function {
        define_rules!{
            function := seq![
                seq![str("pub"), WHITESPACE()].optional(),
                str("fn").hide(), WHITESPACE(),
                Identifier::identifier().expand_once().group("id"), WHITESPACE(),
                str("(").hide(), WHITESPACE(),
                Function::argument().separate(str(",").separate_around(WHITESPACE()).hide()).group("args").optional(),
                str(")").hide(), WHITESPACE(),
                str("{").hide(), WHITESPACE(), str("}").hide(),
            ];
            argument := seq![Identifier::identifier(), WHITESPACE(), DataType::data_type()];
        }
    }
}

#[derive(VoltModuleDefinition)]
struct DataType {
    data_type: Element,
    primitive: Element,
}

impl VoltModule for DataType {
    fn new() -> DataType {
        define_rules!{
            data_type := choice![DataType::primitive()];
            primitive := choice![str("usize"), str("char"), str("str")];
        }
    }
}
