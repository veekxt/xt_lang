mod lexer;
mod parser;
mod status;

use lexer::*;
use parser::*;
use std::path::Path;

fn main(){
    let mut ast:AST;
    {
        let mut tokens = get_tokens_from(Path::new("test/3.xt"));
        {
            println!("Tokens :");
            for &(ref t,ref l) in &tokens.vec_data {
                print!("line {}  {}",l,t);
                print!("\n");
            }
            println!("Tokens END");
        }
        if unsafe{!status::lexer_err} {
            tokens.i+=1;
            ast = stmt(&mut tokens);
            ast.print(0);
        }

    }
}
