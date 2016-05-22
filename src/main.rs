mod lexer;
mod parser;
mod err_status;

use lexer::*;
use parser::*;
use std::path::Path;

fn main(){
    let ast:AST;
    {
        let mut tokens = get_tokens_from(Path::new("test/4.xt"));
        {
            println!("Tokens :");
            for &(ref t,ref l) in &tokens.vec_data {
                print!("line {}  {}",l,t);
                print!("\n");
            }
            println!("Tokens END");
        }
        if unsafe{!err_status::lexer_err} {
            ast = stmt(&mut tokens,&mut parser::Status::new(0,0,0));
            ast.print(0);
        }

    }
}
