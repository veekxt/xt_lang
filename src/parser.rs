use lexer::*;

pub enum AST {
    STMT(Vec<AST>),
    VAR{ iden:Box<AST>, exp: Box<AST> },
    IF{ exp: Box<AST>, stmt: Box<AST>, else_stmt:Option<Box<AST>>  },
    WHILE{ exp: Box<AST>, stmt: Box<AST> },
    FN_CALL{ exp: Box<AST>, arg_list: Box<AST> },
    ARGS(Vec<AST>),
    ASSIGN{ left_value:Box<AST>, exp: Box<AST> },
    PLUS{ left:Box<AST>, right: Box<AST> },
    INT(isize),
    FLOAT(f64),
    STR(String),
    IDEN(String),
    NULL,
    ERR(String),
}

impl AST {
    pub fn print(&self) {
        match *self {
            AST::INT(ref s) => {
                print!("{}",s);
            },
            AST::PLUS{ref left,ref right} => {
                print!("+");
                print!("(");
                left.print();
                print!(",");
                right.print();
                print!(")");
            }
            _ => {print!("todo:unknown");},
        }
    }
}

macro_rules! err_return {
    ($fn_exp:expr) => (
        match $fn_exp {
            AST::ERR(s) => { return AST::ERR(s); },
            ast => { ast },
        }
    )
}

pub fn expe(tokens:&mut StatusVec<Token>) -> AST {
    let mut eroot=err_return!(exp7(tokens));
    let mut eright:Box<AST>;
    loop {
        match tokens.get(0,0) {
            Token::PLUS => {
                tokens.i+=1;
                let left = AST::PLUS{left:Box::new(eroot),right:Box::new(err_return!(exp7(tokens)))};
                eroot=left;
            },
            _ =>{break;},
        }
    }
    eroot
}

pub fn exp7(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::INT(s) => { a_int(tokens) },
        _ => { AST::ERR("expect min exp !".to_string()) },
    }
}

pub fn a_int(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,1);
    match t {
        Token::INT(s) => { AST::INT(s) },
        _ => { AST::ERR("expect int !".to_string()) },
    }
}

pub fn a_float(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,1);
    match t {
        Token::FLOAT(s) => { AST::FLOAT(s) },
        _ => { AST::ERR("expect float !".to_string()) },
    }
}

pub fn a_str(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,1);
    match t {
        Token::STR(s) => { AST::STR(s) },
        _ => { AST::ERR("expect str !".to_string()) },
    }
}

pub fn a_iden(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,1);
    match t {
        Token::IDEN(s) => {  AST::IDEN(s) },
        _ => { AST::ERR("expect str !".to_string()) },
    }
}
