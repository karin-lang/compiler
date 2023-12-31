mod expr;
mod item;

use crate::ast::syntax::Syntax;
use colored::*;
use speculate::speculate;
use volt::{*, parser::*, rule::*, tree::*};

speculate!{
    before {
        let volt = &mut Syntax::generate_volt(1024);

        let assert_ast = |input: &str, rule_id: &str, expected: ParserResult|
            assert_eq!(Parser::parse(volt, input, &RuleId(rule_id.to_string())), expected);

        #[allow(unused)]
        let expect_success = |input: &str, rule_id: &str|
            Parser::parse(volt, input, &RuleId(rule_id.to_string())).expect(&"parsing unexpectedly failed".red().to_string());

        #[allow(unused)]
        let expect_success_eq = |input: &str, rule_id: &str, expected: SyntaxTree|
            assert_ast(input, rule_id, Ok(expected));

        #[allow(unused)]
        let expect_failure = |input: &str, rule_id: &str|
            Parser::parse(volt, input, &RuleId(rule_id.to_string())).expect_err(&"parsing unexpectedly succeeded".cyan().to_string());

        #[allow(unused)]
        let expect_unmatch_failure = |input: &str, rule_id: &str|
            if Parser::parse(volt, input, &RuleId(rule_id.to_string())) != Err(ParserError::NoMatchedRule) {
                panic!("{}", "input unexpectedly matched syntax rule".cyan());
            };

        #[allow(unused)]
        let expect_failure_eq = |input: &str, rule_id: &str, expected: ParserError|
            assert_ast(input, rule_id, Err(expected));
    }

    describe "main" {
        it "separated by expression separator" {
            expect_success("\n", "Main::main");
            expect_success("fn f() {}\nfn f() {}", "Main::main");
            expect_success("\nfn f() {}\nfn f() {}\n", "Main::main");
        }

        it "rejects semicolon separator" {
            expect_unmatch_failure(";", "Main::main");
        }
    }

    describe "accessibility" {
        it "matches any accessibility optionally" {
            expect_success_eq("pub", "Main::accessibility", tree!(
                node!("Main::accessibility" => [leaf!("pub")])
            ));

            expect_success_eq("", "Main::accessibility", tree!(
                node!("Main::accessibility" => [])
            ));
        }
    }

    describe "identifier" {
        it "starts with alphabetic" {
            expect_success_eq("a", "Identifier::identifier", tree!(
                node!("Identifier::identifier" => [
                    leaf!("a"),
                ])
            ));
        }

        it "does not start with number" {
            expect_unmatch_failure("0", "Identifier::identifier");
        }

        it "has multiple kinds of character" {
            expect_success_eq("a0_", "Identifier::identifier", tree!(
                node!("Identifier::identifier" => [
                    leaf!("a0_"),
                ])
            ));
        }

        it "does not match reserved keywords" {
            expect_unmatch_failure("fn", "Identifier::identifier");
        }

        it "matches characters after reserved keyword" {
            expect_success_eq("fn_", "Identifier::identifier", tree!(
                node!("Identifier::identifier" => [
                    leaf!("fn_"),
                ])
            ));
        }
    }

    describe "symbol" {
        describe "whitespace" {
            it "matches any kind of whitespace" {
                expect_success_eq(" ", "Symbol::whitespace", tree!(
                    node!("Symbol::whitespace" => [leaf!(" ")])
                ));
            }
        }

        describe "expression separator" {
            it "accepts zero or more whitespaces around expression separator" {
                expect_success("\n", "Symbol::expression_separator");
                expect_success(";", "Symbol::expression_separator");
                expect_success("  \n  ", "Symbol::expression_separator");
            }
        }
    }
}
