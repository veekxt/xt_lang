mod lexer;
mod parser;
use lexer::*;
//use parser::*;
use std::path::Path;

pub enum AST {
    STMT(Vec<AST>),
    VAR{ iden:Box<AST>, exp: Box<AST> },
    IF{ exp: Box<AST>, stmt: Box<AST>, else_stmt:Option<Box<AST>>  },
    WHILE{ exp: Box<AST>, stmt: Box<AST> },
    FN_CALL{ exp: Box<AST>, arg_list: Box<AST> },
    ARGS(Vec<AST>),
    ASSIGN{ left_value:Box<AST>, exp: Box<AST> },
    INT(isize),
    STR(String),
    IDEN(String),
}

fn build_exp() -> AST {
    return AST::STR("asd".to_string());
}

fn main(){
    let tokens = get_tokens_from(Path::new("test/1.xt"));
    println!("Tokens :");
    for t in tokens.vec_data {
        print!("  {}",t);
        print!("\n");
    }
    println!("Tokens END");
}
