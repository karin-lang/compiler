pub mod code;
pub mod ir;
pub mod jsify;

use volt::parser::ParserError;
use crate::hir::hirify::{TreeHirifier, AstHako, AstModule};
use crate::hir::type_check::DataTypeChecker;
use crate::{Compiler, ParserResult, Syntax, RuleId};
use crate::js::jsify::JsGenerator;
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

        let mut hir = TreeHirifier::hirify(vec![
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

        // todo: handle errors
        let data_type_errors = DataTypeChecker::check(&hir.path_tree, &mut hir.items);
        let js = JsGenerator::generate(&hir);
        let js_code = JsCodeGenerator::generate(&js);
        Ok(js_code)
    }

    fn parse(&self, input: &str) -> ParserResult {
        let volt = &mut Syntax::generate_volt(1024);
        volt.parse(input, &RuleId("Main::main".to_string()))
    }
}
