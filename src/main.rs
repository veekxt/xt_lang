mod lexer;
mod parser;
mod run;
mod xt_type;

use lexer::*;
use parser::*;
use run::*;
use std::path::Path;

fn main() {
    {

        let (mut tokens, lexer_err) = get_tokens_from(Path::new("test/args.xt"));
//        {
//            println!("Tokens :");
//            for &(ref t, ref l) in &tokens.vec_data {
//                print!("line {}  {}", l, t);
//                print!("\n");
//            }
//            println!("Tokens END");
//        }
        if !lexer_err {
            println!("Parser :");
            let (ast, parser_err) = stmt(&mut tokens, &mut parser::Status::new(false, 0, 0, 0, 0));
            if !parser_err {
                ast.print(0);
                println!("\nInterpreter :");
                interprete(ast);
            }
        }
    }
}
