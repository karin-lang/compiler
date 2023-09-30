pub mod hir;
pub mod syntax;
#[cfg(test)]
mod tests;

pub trait Compiler<Input, Output, Options> {
    fn new(options: Options, output: Output) -> Self;

    fn compile(&self, input: Input) -> Output;
}
