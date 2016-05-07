mod lexer;
use lexer::*;
use std::path::Path;

fn main(){
    let tokens = get_tokens_from(Path::new("test/1.xt"));
    for t in tokens {
        t.print();
        print!("\n");
    }
}
