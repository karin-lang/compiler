use volt::{*, element::*, tree::*};
use volt_derive::VoltModuleDefinition;
use super::*;

#[derive(VoltModuleDefinition)]
pub(super) struct Expression {
    expression: Element,
    pure_expression: Element,
}

impl VoltModule for Expression {
    fn new() -> Self {
        let expression_reducer = |mut children: Vec<SyntaxChild>| {
            let expose = {
                if let Some(operation_node) = children.get_node_or_none(0) {
                    operation_node.children.len() == 1
                } else {
                    false
                }
            };

            if expose {
                if let SyntaxChild::Node(mut node) = children.pop().unwrap() {
                    vec![node.children.pop().unwrap().into_node().children.to_owned().pop().unwrap()]
                } else {
                    unreachable!();
                }
            } else {
                children
            }
        };

        define_rules!{
            expression := Operation::operation().reduce(expression_reducer);
            pure_expression := choice![
                Literal::literal(),
                DataType::data_type(),
                Identifier::identifier(),
            ];
        }
    }
}

#[derive(VoltModuleDefinition)]
pub(super) struct Literal {
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
pub(super) struct Operation {
    operation: Element,
    term: Element,
    infix: Element,
    group: Element,
    prefix_operator: Element,
    postfix_operator: Element,
    function_call_operator: Element,
    infix_operator: Element,
}

impl VoltModule for Operation {
    fn new() -> Self {
        define_rules!{
            operation := choice![
                Operation::infix().expand_once(),
                Operation::term().expand_once(),
            ];
            term := seq![
                Operation::prefix_operator().expand_once().group("operator").min(0),
                WHITESPACE(),
                choice![
                    Operation::group().expand_once(),
                    Expression::pure_expression(),
                ],
                WHITESPACE(),
                Operation::postfix_operator().expand_once().group("operator").min(0),
            ];
            infix := seq![
                Operation::term().expand_once(),
                seq![
                    WHITESPACE(),
                    Operation::infix_operator().expand_once().group("operator"),
                    WHITESPACE(),
                    Operation::term().expand_once(),
                ].min(1),
            ];
            group := seq![
                str("(").group("operator"),
                WHITESPACE(),
                Expression::expression(),
                WHITESPACE(),
                str(")").group("operator"),
            ];
            prefix_operator := choice![
                str("!"), str("~"), str("-"),
            ].reduce(|mut v| match v.pop().unwrap() {
                SyntaxChild::Leaf(mut leaf) => {
                    leaf.set_value(format!("{}e", leaf.value));
                    vec![SyntaxChild::Leaf(leaf)]
                },
                _ => unreachable!(),
            });
            postfix_operator := choice![
                choice![
                    str("!"), str("?"),
                ].reduce(|mut v| match v.pop().unwrap() {
                    SyntaxChild::Leaf(mut leaf) => {
                        leaf.set_value(format!("e{}", leaf.value));
                        vec![SyntaxChild::Leaf(leaf)]
                    },
                    _ => unreachable!(),
                }),
                Operation::function_call_operator(),
            ];
            function_call_operator := seq![
                str("(").hide(),
                WHITESPACE(),
                Expression::expression().separate(str(",").separate_around(WHITESPACE()).hide()).optional(),
                WHITESPACE(),
                str(")").hide(),
            ];
            infix_operator := choice![
                str("="), str("+"), str("-"), str("*"), str("."), str("::"),
            ];
        }
    }
}

#[derive(VoltModuleDefinition)]
pub(super) struct DataType {
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
                Identifier::identifier().expand_once().group("Identifier::identifier"), WHITESPACE(),
                str("<").hide(), WHITESPACE(),
                choice![DataType::data_type(), Identifier::identifier().expand_once().group("Identifier::identifier")].separate(str(",").separate_around(WHITESPACE()).hide()).group("args"),
                str(">").hide(),
            ];
        }
    }
}
