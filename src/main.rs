mod lexer;
mod parser;
mod err_status;

use lexer::*;
use parser::*;
use std::path::Path;

fn main(){
    let mut ast:AST=AST::NULL;
    {
        let mut tokens = get_tokens_from(Path::new("test/args.xt"));
        {
            println!("Tokens :");
            for &(ref t,ref l) in &tokens.vec_data {
                print!("line {}  {}",l,t);
                print!("\n");
            }
            println!("Tokens END");
        }
        if unsafe{!err_status::lexer_err} {
            ast = stmt(&mut tokens,&mut parser::Status::new(0,0,0,0));
        }
        if unsafe{!err_status::parser_err} {
            ast.print(0);
        }
    }
}
