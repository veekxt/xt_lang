use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

enum Token{
    STR(String),
    NUM(String),
    LPAR,
    RPAR,
    LSQB,
    RSQB,
    COMMA,
    LBRACE,
    RBRACE,
}


fn get_char_vec(path:&Path) -> Vec<char> {
    let mut f = match File::open(path){
        Ok(r) => r,
        _ => panic!("can't open file !"),
    }; 
    let mut buffer = String::new();
    match f.read_to_string(&mut buffer){
        Ok(r) => r,
        _ => panic!("can't convent file to string !"),
    };
    let str_vec: Vec<char> = buffer.chars().collect();
    str_vec
}

fn main(){
    let a = get_char_vec(Path::new("../test/1.xt"));
    let tokens: Vec<Token> = Vec::new();
}
