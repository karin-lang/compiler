pub mod ast;
pub mod hir;
pub mod js;
#[cfg(test)]
mod tests;

use volt::{rule::RuleId, parser::ParserResult};
use ast::syntax::Syntax;

pub trait Compiler<Input, Output, Options> {
    fn new(options: Options) -> Self;

    fn compile(&self, input: Input) -> Output;

    fn parse(&self, input: Input) -> ParserResult;
}
