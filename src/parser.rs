use lexer::*;

pub enum AST {
    STMT(Vec<AST>),
    VAR{ iden:Box<AST>, exp: Box<AST> },
    IF{ exp: Box<AST>, stmt: Box<AST>, else_stmt:Option<Box<AST>>  },
    WHILE{ exp: Box<AST>, stmt: Box<AST> },
    FN_CALL{ exp: Box<AST>, arg_list: Box<AST> },
    ARGS(Vec<AST>),
    ASSIGN{ left_value:Box<AST>, exp: Box<AST> },
    
    COMMA,
    PLUS{ left:Box<AST>, right: Box<AST> },
    MINUS{ left:Box<AST>, right: Box<AST> },
    NEG(Box<AST>),
    MUL{ left:Box<AST>, right: Box<AST> },
    DIV{ left:Box<AST>, right: Box<AST> },
    MOD{ left:Box<AST>, right: Box<AST> },
    LT{ left:Box<AST>, right: Box<AST> },
    LE{ left:Box<AST>, right: Box<AST> },
    GT{ left:Box<AST>, right: Box<AST> },
    GE{ left:Box<AST>, right: Box<AST> },
    EQ{ left:Box<AST>, right: Box<AST> },
    NE{ left:Box<AST>, right: Box<AST> },
    ANDAND{ left:Box<AST>, right: Box<AST> },
    OROR{ left:Box<AST>, right: Box<AST> },
    DOT{ left:Box<AST>, right: Box<AST> },
    NOT(Box<AST>),
    FALSE,
    TRUE,
    
    INT(isize),
    FLOAT(f64),
    STR(String),
    IDEN(String),
    NULL,
    ERR(String),
}

impl AST {
    pub fn print(&self,n:usize) {
        let mut q_left:&AST = &AST::NULL;
        let mut q_right:&AST = &AST::NULL;
        for i in 0..n {
            print!("  ");
        }
        match *self {
            AST::INT(ref s) => {
                print!("{}",s);
            },
            AST::STR(ref s) => {
                print!("'{}'",s);
            },
            AST::FLOAT(ref s) => {
                print!("{}",s);
            },
            AST::IDEN(ref s) => {
                print!("{}",s);
            },
            AST::NULL => {
                print!("NULL");
            },
            AST::PLUS{ref left,ref right} => {
                print!("+");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::MINUS{ref left,ref right} => {
                print!("-");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::MUL{ref left,ref right} => {
                print!("*");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::DIV{ref left,ref right} => {
                print!("/");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::MOD{ref left,ref right} => {
                print!("%");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::LT{ref left,ref right} => {
                print!("<");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::LE{ref left,ref right} => {
                print!("<=");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::GT{ref left,ref right} => {
                print!(">");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::GE{ref left,ref right} => {
                print!(">=");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::EQ{ref left,ref right} => {
                print!("==");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::NE{ref left,ref right} => {
                print!("!=");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::ANDAND{ref left,ref right} => {
                print!("and");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::OROR{ref left,ref right} => {
                print!("or");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::NEG(ref left) => {
                print!("neg:-");
                q_left = (*left).as_ref();
            },
            AST::NOT(ref left) => {
                print!("not");
                q_left = (*left).as_ref();
            },
            AST::TRUE => {
                print!("true");
            },
            AST::FALSE => {
                print!("false");
            },
            _ => {print!("todo:unknown");},
        }
        match *q_left {
            AST::NULL => {},
            _ => {
                //print!("( ");
                print!("\n");
                q_left.print(n+1);
                match *q_right {
                    AST::NULL => {},
                    _ => {
                        //print!(",");
                        print!("\n");
                        q_right.print(n+1);
                    },
                }
                //print!(" )");
            },
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

pub fn exp(tokens:&mut StatusVec<Token>) -> AST {
    let mut eroot=err_return!(exp1(tokens));
    let mut eright:Box<AST>;
    loop {
        match tokens.get(0,0) {
            Token::ANDAND => {
                tokens.i+=1;
                let left = AST::ANDAND{left:Box::new(eroot),right:Box::new(err_return!(exp1(tokens)))};
                eroot=left;
            },
            Token::OROR => {
                tokens.i+=1;
                let left = AST::OROR{left:Box::new(eroot),right:Box::new(err_return!(exp1(tokens)))};
                eroot=left;
            },
            _ =>{break;},
        }
    }
    eroot
}

pub fn exp1(tokens:&mut StatusVec<Token>) -> AST {
    let mut eroot=err_return!(exp2(tokens));
    let mut eright:Box<AST>;
    loop {
        match tokens.get(0,0) {
            Token::LT=> {
                tokens.i+=1;
                let left = AST::LT{left:Box::new(eroot),right:Box::new(err_return!(exp2(tokens)))};
                eroot=left;
            },
            Token::LE => {
                tokens.i+=1;
                let left = AST::LE{left:Box::new(eroot),right:Box::new(err_return!(exp2(tokens)))};
                eroot=left;
            },
            Token::GT=> {
                tokens.i+=1;
                let left = AST::GT{left:Box::new(eroot),right:Box::new(err_return!(exp2(tokens)))};
                eroot=left;
            },
            Token::GE => {
                tokens.i+=1;
                let left = AST::GE{left:Box::new(eroot),right:Box::new(err_return!(exp2(tokens)))};
                eroot=left;
            },
            Token::EQEQ => {
                tokens.i+=1;
                let left = AST::EQ{left:Box::new(eroot),right:Box::new(err_return!(exp2(tokens)))};
                eroot=left;
            },
            Token::NE => {
                tokens.i+=1;
                let left = AST::NE{left:Box::new(eroot),right:Box::new(err_return!(exp2(tokens)))};
                eroot=left;
            },
            _ =>{break;},
        }
    }
    eroot
}

pub fn exp2(tokens:&mut StatusVec<Token>) -> AST {
    let mut eroot=err_return!(exp3(tokens));
    let mut eright:Box<AST>;
    loop {
        match tokens.get(0,0) {
            Token::PLUS => {
                tokens.i+=1;
                let left = AST::PLUS{left:Box::new(eroot),right:Box::new(err_return!(exp3(tokens)))};
                eroot=left;
            },
            Token::MINUS => {
                tokens.i+=1;
                let left = AST::MINUS{left:Box::new(eroot),right:Box::new(err_return!(exp3(tokens)))};
                eroot=left;
            },
            _ =>{break;},
        }
    }
    eroot
}

pub fn exp3(tokens:&mut StatusVec<Token>) -> AST {
    let mut eroot=err_return!(exp4(tokens));
    let mut eright:Box<AST>;
    loop {
        match tokens.get(0,0) {
            Token::STAR => {
                tokens.i+=1;
                let left = AST::MUL{left:Box::new(eroot),right:Box::new(err_return!(exp4(tokens)))};
                eroot=left;
            },
            Token::SLASH => {
                tokens.i+=1;
                let left = AST::DIV{left:Box::new(eroot),right:Box::new(err_return!(exp4(tokens)))};
                eroot=left;
            },
            Token::PERCENT => {
                tokens.i+=1;
                let left = AST::MOD{left:Box::new(eroot),right:Box::new(err_return!(exp4(tokens)))};
                eroot=left;
            },
            _ =>{break;},
        }
    }
    eroot
}

pub fn exp4(tokens:&mut StatusVec<Token>) -> AST {
    match tokens.get(0,0) {
        Token::MINUS => {
            tokens.i+=1;
            AST::NEG(Box::new(err_return!(exp4(tokens))))
        },
        _ => {
            err_return!(expn(tokens))
        },
    }
}

pub fn expn(tokens:&mut StatusVec<Token>) -> AST {
    match tokens.get(0,0) {
        Token::NOT => {
            tokens.i+=1;
            AST::NOT(Box::new(err_return!(exp1(tokens))))
        },
        _ => {
            err_return!(exp7(tokens))
        },
    }
}

pub fn exp5(tokens:&mut StatusVec<Token>) -> AST {
    err_return!(exp6(tokens))
}

pub fn exp6(tokens:&mut StatusVec<Token>) -> AST {
    err_return!(exp7(tokens))
}

pub fn exp7(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::TRUE     =>    { a_bool(tokens) },
        Token::FALSE    =>    { a_bool(tokens) },
        Token::INT(s)   =>    { a_int(tokens) },
        Token::FLOAT(s) =>    { a_float(tokens) },
        Token::STR(s)   =>    { a_str(tokens) },
        Token::IDEN(s)  =>    { a_iden(tokens) },
        Token::LPAR     => {
            tokens.i+=1;
            let tmp = err_return!(exp(tokens));
            tokens.i+=1;
            tmp
        }
        _ => { AST::ERR("expect min exp !".to_string()) },
    }
}

pub fn call(tokens:&mut StatusVec<Token>) -> AST {
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

pub fn args(tokens:&mut StatusVec<Token>) -> AST {
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

pub fn a_bool(tokens:&mut StatusVec<Token>) -> AST {
    let t = tokens.get(0,1);
    match t {
        Token::TRUE => { AST::TRUE },
        Token::FALSE => { AST::FALSE },
        _ => { AST::ERR("expect \"true\" or \"false\"".to_string()) },
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
