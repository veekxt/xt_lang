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

pub fn exp_int(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::INT(s) => { return AST::INT(s) },
        _ => {return AST::ERR("expect int !".to_string());},
    }
}

pub fn exp_float(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::FLOAT(s) => { return AST::FLOAT(s) },
        _ => {return AST::ERR("expect float !".to_string());},
    }
}

pub fn exp_str(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::STR(s) => { return AST::STR(s) },
        _ => {return AST::ERR("expect str !".to_string());},
    }
}
