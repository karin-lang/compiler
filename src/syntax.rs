use volt::{*, element::*, tree::*};
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
        volt.add_module(Expression::new());
        volt.add_module(Literal::new());
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
            main := choice![Main::item().separate_around(WHITESPACE()), WHITESPACE()];
            item := choice![Function::function()];
            accessibility := choice![str("pub@hako"), str("pub")].optional();
        }
    }
}

#[derive(VoltModuleDefinition)]
struct Symbol {
    expression_separator: Element,
    around_expression_separator: Element,
    whitespace: Element,
}

impl VoltModule for Symbol {
    fn new() -> Symbol {
        define_rules!{
            expression_separator := choice![str("\n"), str(";")].around(Symbol::around_expression_separator().min(0));
            around_expression_separator := choice![str(" "), str("\t")];
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
                seq![Main::accessibility(), WHITESPACE()].optional(),
                str("fn").hide(), WHITESPACE(),
                Identifier::identifier().expand_once().group("name"), WHITESPACE(),
                str("(").hide(), WHITESPACE(),
                Function::argument().separate(str(",").separate_around(WHITESPACE()).hide()).optional().group("args"), WHITESPACE(),
                str(")").hide(), WHITESPACE(),
                str("{").hide(), WHITESPACE(),
                Expression::expression().separate_around(Symbol::expression_separator().min(0).hide()).optional().group("exprs"), WHITESPACE(),
                str("}").hide(),
            ];
            argument := seq![Identifier::identifier(), WHITESPACE(), DataType::data_type()];
        }
    }
}

#[derive(VoltModuleDefinition)]
struct Expression {
    expression: Element,
}

impl VoltModule for Expression {
    fn new() -> Self {
        define_rules!{
            expression := choice![
                Literal::literal(),
                DataType::data_type(),
            ];
        }
    }
}

#[derive(VoltModuleDefinition)]
struct Literal {
    literal: Element,
    boolean: Element,
    number: Element,
    integer: Element,
    binary_number: Element,
    octal_number: Element,
    hexadecimal_number: Element,
    decimal_number: Element,
    decimal_number_value: Element,
    number_exponent: Element,
    float_number: Element,
    float_number_value: Element,
}

impl VoltModule for Literal {
    fn new() -> Self {
        let integer_reducer = |children: Vec<SyntaxChild>| {
            let leaf = children.get_leaf(0);
            let pure_value = leaf.value.replace('_', "");
            let new_leaf = SyntaxChild::leaf(leaf.start.clone(), pure_value.clone());
            let mut errors = Vec::new();

            if leaf.value.starts_with('_') || leaf.value.ends_with('_') {
                errors.push(
                    SyntaxChild::error(
                        "digit_separator_on_side".to_string(),
                        vec![new_leaf.clone()],
                    ),
                );
            }

            if pure_value.len() >= 2 && pure_value.starts_with('0') {
                errors.push(
                    SyntaxChild::error(
                        "starts_with_zero".to_string(),
                        vec![new_leaf.clone()],
                    ),
                );
            }

            for ch in pure_value.chars() {
                if let 'A'..='F' = ch {
                    errors.push(
                        SyntaxChild::error(
                            "has_capital_letter".to_string(),
                            vec![new_leaf.clone()],
                        ),
                    );

                    break;
                }
            }

            if errors.len() == 0 {
                vec![new_leaf]
            } else {
                errors
            }
        };

        let float_reducer = |children: Vec<SyntaxChild>| {
            let leaf = children.get_leaf(0);
            let pure_value = leaf.value.replace('_', "");
            let new_leaf = SyntaxChild::leaf(leaf.start.clone(), pure_value.clone());
            let mut errors = Vec::new();

            if leaf.value.starts_with('_') || leaf.value.ends_with('_') {
                errors.push(
                    SyntaxChild::error(
                        "digit_separator_on_side".to_string(),
                        vec![new_leaf.clone()],
                    ),
                );
            }

            if pure_value.len() >= 2 && pure_value.ends_with('0') {
                errors.push(
                    SyntaxChild::error(
                        "ends_with_zero".to_string(),
                        vec![new_leaf.clone()],
                    ),
                );
            }

            if errors.len() == 0 {
                vec![new_leaf]
            } else {
                errors
            }
        };

        let exponent_symbol_reducer = |children: Vec<SyntaxChild>| {
            let leaf = children.get_leaf(0);

            match leaf.value.as_str() {
                "e" => vec![SyntaxChild::leaf(leaf.start.clone(), "+".to_string())],
                value @ "e+" => vec![
                    SyntaxChild::error(
                        "explicit_plus_symbol".to_string(),
                        vec![SyntaxChild::leaf(leaf.start.clone(), value.to_string())]
                    )
                ],
                "e-" => vec![SyntaxChild::leaf(leaf.start.clone(), "-".to_string())],
                _ => unreachable!(),
            }
        };

        define_rules!{
            literal := choice![Literal::boolean(), Literal::number()];
            boolean := choice![str("true"), str("false")];
            number := choice![
                Literal::float_number(),
                Literal::integer().expand_once(),
            ];
            integer := seq![
                // Parses decimal number at the last not to consume '0' in base prefix.
                choice![
                    Literal::binary_number(),
                    Literal::octal_number(),
                    Literal::hexadecimal_number(),
                    Literal::decimal_number(),
                ].group("value"),
                Literal::number_exponent().optional(),
                // todo: add float type checker
                DataType::primitive_number().optional(),
            ];
            binary_number := seq![
                str("0b").hide(),
                choice![chars("0-1"), str("_")].min(1).join().reduce(integer_reducer),
            ];
            octal_number := seq![
                str("0o").hide(),
                choice![chars("0-7"), str("_")].min(1).join().reduce(integer_reducer),
            ];
            hexadecimal_number := seq![
                str("0x").hide(),
                choice![chars("0-9a-fA-F"), str("_")].min(1).join().reduce(integer_reducer),
            ];
            decimal_number := Literal::decimal_number_value().expand_once();
            decimal_number_value := choice![chars("0-9"), str("_")].min(1).join().reduce(integer_reducer);
            // todo: use replace()
            number_exponent := seq![choice![str("e+"), str("e-"), str("e")].reduce(exponent_symbol_reducer), Literal::decimal_number_value().expand_once().group("value")];
            float_number := seq![
                Literal::decimal_number_value().expand_once().group("integer"),
                str(".").hide(),
                Literal::float_number_value().expand_once().group("float"),
                // todo: add float type checker
                DataType::primitive_number().optional(),
            ];
            float_number_value := choice![chars("0-9"), str("_")].min(1).join().reduce(float_reducer);
        }
    }
}

#[derive(VoltModuleDefinition)]
struct DataType {
    data_type: Element,
    primitive: Element,
    primitive_number: Element,
    generic: Element,
}

impl VoltModule for DataType {
    fn new() -> DataType {
        define_rules!{
            data_type := choice![DataType::primitive(), DataType::generic()];
            primitive := choice![DataType::primitive_number().expand_once(), str("char"), str("str")];
            // add: types
            primitive_number := choice![str("usize"), str("f32")];
            generic := seq![
                Identifier::identifier().expand_once().group("name"), WHITESPACE(),
                str("<").hide(), WHITESPACE(),
                choice![DataType::data_type(), Identifier::identifier().expand_once().group("name")].separate(str(",").separate_around(WHITESPACE()).hide()).group("args"),
                str(">").hide(),
            ];
        }
    }
}
