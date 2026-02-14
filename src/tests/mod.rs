use std::env;

use insta::glob;

use crate::{
    checking::Checker,
    lexing::Lexer,
    module::{module::Module, tree::ModuleTree},
    parsing::Parser,
};

fn lexing(module: &Module) {
    let lexer = Lexer::new(module);
    let tokens = lexer.collect::<Vec<_>>();

    insta::assert_debug_snapshot!(tokens);
}

fn parsing(module: &Module) {
    let lexer = Lexer::new(module);
    let mut parser = Parser::new(lexer);

    let ast = parser.parse();

    if ast.is_err() {
        println!("{:?}", ast);
    }
    assert!(ast.is_ok());
    insta::assert_debug_snapshot!(ast.unwrap());
}

fn checking(module: &Module) {
    let lexer = Lexer::new(module);
    let mut parser = Parser::new(lexer);
    let mut checker = Checker::new();

    let ast = parser.parse();
    let checked = checker.check(ast.unwrap());

    if checked.is_err() {
        println!("{:?}", checked);
    }
    assert!(checked.is_ok());
    insta::assert_debug_snapshot!(checked.unwrap());
}

#[test]
fn snapshot_tests() {
    let root_dir = env::current_dir().unwrap();
    let cases_dir = root_dir.join("src").join("tests").join("cases");

    glob!("cases/*.alloy", |path| {
        let module_tree = ModuleTree::new(&cases_dir, path.file_name());
        let entry = module_tree.entry();

        lexing(entry);
        parsing(entry);
        checking(entry);
    });
}
