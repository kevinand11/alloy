use insta::glob;
use std::fs;

use crate::lexer::Lexer;

#[test]
fn snapshot_tests() {
    glob!("cases/*.alloy", |path| {
        let input = fs::read_to_string(path).unwrap();

        let lexer = Lexer::new(&input);
        let tokens = lexer.collect::<Vec<_>>();
        insta::assert_debug_snapshot!(tokens);
    });
}
