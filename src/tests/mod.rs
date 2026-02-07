use insta::glob;
use std::fs;

use crate::{
    lexer::{Lexer, module::Module},
    parser::Parser,
};

#[test]
fn snapshot_tests() {
    glob!("cases/*.alloy", |path| {
        let input = fs::read_to_string(path).unwrap();

        let module = Module::new(input.to_string());
        let lexer = Lexer::new(module);

        let peeker = lexer.get_peeker();
        let tokens = peeker.all();
        insta::assert_debug_snapshot!(tokens);

        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap();
        //insta::assert_debug_snapshot!(ast);
    });
}
