use syntax::Syntax;
use volt::{rule::RuleId, parser::ParserResult};

pub mod hir;
pub mod syntax;
pub mod tree;
#[cfg(test)]
mod tests;

pub trait Compiler<Input, Output, Options> {
    fn new(options: Options) -> Self;

    fn compile(&self, input: Input) -> Output;

    fn parse(&self, input: Input) -> ParserResult;
}

pub struct JsTranspiler;

impl Compiler<&str, String, ()> for JsTranspiler {
    fn new(options: ()) -> Self {
        Self
    }

    fn compile(&self, input: &str) -> String {
        unimplemented!();
    }

    fn parse(&self, input: &str) -> ParserResult {
        let volt = &mut Syntax::generate_volt(1024);
        volt.parse(input, &RuleId("Main::main".to_string()))
    }
}
