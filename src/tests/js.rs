mod code;
mod generate;

use speculate::speculate;

use crate::*;
use crate::js::{JsTranspiler, JsTranspilerOptions};

speculate!{
    it "" {
        let compiler = JsTranspiler::new(JsTranspilerOptions);
        assert_eq!(compiler.compile("fn main(){}"), Ok("function f_2(){}".to_string()));
    }
}
