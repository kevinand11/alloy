use insta::glob;
use std::fs;

use crate::lexer::{Lexer, module::Module};

#[test]
fn snapshot_tests() {
    glob!("cases/*.alloy", |path| {
        let input = fs::read_to_string(path).unwrap();

        let module = Module::new(&input);

        let lexer = Lexer::new(&module);
        let tokens = lexer.collect::<Vec<_>>();
        insta::assert_debug_snapshot!(tokens);
    });
}
