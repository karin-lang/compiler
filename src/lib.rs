pub mod hir;
pub mod ast;
pub mod js;
#[cfg(test)]
mod tests;

use ast::syntax::Syntax;
use volt::{rule::RuleId, parser::ParserResult};

pub trait Compiler<Input, Output, Options> {
    fn new(options: Options) -> Self;

    fn compile(&self, input: Input) -> Output;

    fn parse(&self, input: Input) -> ParserResult;
}
