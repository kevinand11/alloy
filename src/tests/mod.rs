use insta::glob;
use std::fs;

use crate::{checking::Checker, lexing::Lexer, module::module::Module, parsing::Parser};

#[test]
fn snapshot_tests() {
    glob!("cases/*.alloy", |path| {
        let input = fs::read_to_string(path).unwrap();

        let module = Module::new(input.to_string(), path.to_path_buf());
        let lexer = Lexer::new(&module);
        let mut parser = Parser::new(lexer);
        let mut checker = Checker::new();

        let tokens = Lexer::new(&module).collect::<Vec<_>>();
        insta::assert_debug_snapshot!(tokens);

        let ast = parser.parse();
        if ast.is_err() {
            println!("{:?} for path {:?}", ast, path);
        }
        assert!(ast.is_ok());
        insta::assert_debug_snapshot!(Parser::new(Lexer::new(&module)).parse().unwrap());

        let checked = checker.check(ast.unwrap());
        if checked.is_err() {
            println!("{:?} for path {:?}", checked, path);
        }
        assert!(checked.is_ok());
        insta::assert_debug_snapshot!(checked.unwrap());
    });
}
