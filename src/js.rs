pub mod code;
pub mod generate;
pub mod ir;

use volt::parser::ParserError;
use crate::ast::tree::{TreeAnalysis, AstHako, AstModule};
use crate::{Compiler, ParserResult, Syntax, RuleId};
use crate::js::generate::JsGenerator;
use crate::js::code::JsCodeGenerator;

#[derive(Clone, Debug, PartialEq)]
pub struct JsTranspilerOptions;

#[derive(Clone, Debug, PartialEq)]
pub enum JsTranspilerError {
    ParserError(ParserError),
}

pub struct JsTranspiler {
    options: JsTranspilerOptions,
}

impl Compiler<&str, Result<String, JsTranspilerError>, JsTranspilerOptions> for JsTranspiler {
    fn new(options: JsTranspilerOptions) -> Self {
        Self { options }
    }

    // todo: support multiple module files.
    fn compile(&self, input: &str) -> Result<String, JsTranspilerError> {
        let tree = match self.parse(input) {
            Ok(v) => v,
            Err(e) => return Err(JsTranspilerError::ParserError(e)),
        };

        let hir = TreeAnalysis::analyze(vec![
            &AstHako {
                id: "test".to_string(),
                modules: vec![
                    AstModule {
                        id: "main".to_string(),
                        node: &tree.root,
                        submodules: Vec::new(),
                    },
                ]
            },
        ]);

        let js = JsGenerator::generate(&hir);
        let js_code = JsCodeGenerator::generate(&js);
        Ok(js_code)
    }

    fn parse(&self, input: &str) -> ParserResult {
        let volt = &mut Syntax::generate_volt(1024);
        volt.parse(input, &RuleId("Main::main".to_string()))
    }
}
