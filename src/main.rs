use libslang::lexer::Lexer;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar, "/libslang/grammar.rs");

use grammar::ScriptParser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let source_code = std::fs::read_to_string(&args[1]).unwrap();
        let lexer = Lexer::new(&source_code[..]);
        let parser = ScriptParser::new();
        let ast = parser.parse(lexer).unwrap();

        println!("{:?}", ast);
    } else {
        println!("no source file provided");
    }
}
