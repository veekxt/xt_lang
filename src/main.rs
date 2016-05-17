mod lexer;
mod parser;
use lexer::*;
use parser::*;
use std::path::Path;

fn main(){
    let mut ast:AST;
    {
        let mut tokens = get_tokens_from(Path::new("test/2.xt"));
        println!("Tokens :");
        for t in &tokens.vec_data {
            print!("  {}",t);
            print!("\n");
        }
        println!("Tokens END");
        ast = expe(&mut tokens);
    }
}
