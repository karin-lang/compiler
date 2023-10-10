use crate::js::generate::JsCodeGenerator;
use speculate::speculate;

speculate!{
    it "a" {
        let generator = JsCodeGenerator;
        generator.generate();
    }
}
