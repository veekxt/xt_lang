mod lexer;
mod parser;
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
        tokens.i+=1;
        ast = exp(&mut tokens);
        ast.print(0);
    }
}
