use lexer::*;

pub enum AST {
    STMT(Vec<AST>),
    VAR{ iden:Box<AST>, exp: Box<AST> },
    IF{ exp: Box<AST>, stmt: Box<AST>, else_stmt:Option<Box<AST>>  },
    WHILE{ exp: Box<AST>, stmt: Box<AST> },
    FN_CALL{ exp: Box<AST>, arg_list: Box<AST> },
    ARGS(Vec<AST>),
    ASSIGN{ left_value:Box<AST>, exp: Box<AST> },
    INT(isize),
    FLOAT(f64),
    STR(String),
    IDEN(String),
    ERR(String),
}

pub fn expe(tokens:&mut StatusVec<Token>) -> AST {
    a_int(tokens)
}

pub fn exp7(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::INT(s) => { a_int(tokens) },
        _ => { AST::ERR("expect min exp !".to_string()) },
    }
}

pub fn a_int(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::INT(s) => { AST::INT(s) },
        _ => { AST::ERR("expect int !".to_string()) },
    }
}

pub fn a_float(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::FLOAT(s) => { AST::FLOAT(s) },
        _ => { AST::ERR("expect float !".to_string()) },
    }
}

pub fn a_str(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::STR(s) => { AST::STR(s) },
        _ => { AST::ERR("expect str !".to_string()) },
    }
}

pub fn a_iden(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::IDEN(s) => {  AST::IDEN(s) },
        _ => { AST::ERR("expect str !".to_string()) },
    }
}
