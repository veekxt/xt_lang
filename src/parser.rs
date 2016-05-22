use lexer::*;
use err_status;

#[derive(Clone)]
pub enum AST {
    STMT(Vec<AST>),
    VAR{ iden:Box<AST>, exp: Box<AST> },
    IF{ exp: Box<AST>, stmt: Box<AST>, else_stmt:Box<AST>  },
    WHILE{ exp: Box<AST>, stmt: Box<AST> },
    BREAK,
    CONTINUE,
    RETURN( Box<AST> ),
    CALL{ exp: Box<AST>, arg_list: Box<AST> },
    ARGS(Vec<AST>),
    INDEX{exp:Box<AST>,index:Box<AST>},
    ASSIGN{ left_value:Box<AST>, exp: Box<AST> },
    DEVAL{ iden:Box<AST>, val:Box<AST> },
    DEF{ iden:Box<AST>,args:Box<AST>,stmt:Box<AST> },
    
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
    END,
    ERR(String,usize),
}

impl AST {
    pub fn print(&self,n:usize) {
        let mut q_left:&AST = &AST::NULL;
        let mut q_right:&AST = &AST::NULL;
        for _ in 0..n {
            print!("    ");
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
                print!("null");
            },
            AST::END => {
                print!("stmt-end");
            },
            AST::DEVAL{ref iden,ref val} => {
                print!("de-val");
                q_left = (*iden).as_ref();
                q_right = (*val).as_ref();
            },
            AST::ASSIGN{ref left_value,ref exp} => {
                print!("=");
                q_left = (*left_value).as_ref();
                q_right = (*exp).as_ref();
            },
            AST::VAR{ref iden,ref exp} => {
                print!("var");
                q_left = (*iden).as_ref();
                q_right = (*exp).as_ref();
            },
            AST::DEF{ref iden,ref args,ref stmt} => {
                println!("def");
                (*iden).as_ref().print(n+1);
                println!("");
                (*args).as_ref().print(n+1);
                println!("");
                (*stmt).as_ref().print(n+1);
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
            AST::DOT{ref left,ref right} => {
                print!(".");
                q_left = (*left).as_ref();
                q_right = (*right).as_ref();
            },
            AST::CALL{ref exp,ref arg_list} => {
                print!("call");
                q_left = (*exp).as_ref();
                q_right = (*arg_list).as_ref();
            },
            AST::ARGS(ref a) => {
                print!("args");
                print!("({})",a.len());
                for i in a {
                    println!("");
                    i.print(n+1);
                }
            },
            AST::INDEX{ref exp,ref index} => {
                print!("index");
                q_left = (*exp).as_ref();
                q_right = (*index).as_ref();
            },
            AST::STMT(ref ast_list) => {
                print!("stmt");
                for i in ast_list {
                    println!("");
                    i.print(n+1);
                }
            },
            AST::IF{ref exp,ref stmt,ref else_stmt} => {
                print!("if");
                println!("");
                (*exp).as_ref().print(n+1);
                println!("");
                (*stmt).as_ref().print(n+1);
                match *((*else_stmt).as_ref()) {
                    AST::NULL => {}
                    _ => {
                        println!("");
                        (*else_stmt).as_ref().print(n+1);
                    }
                }
            },
            AST::WHILE{ref exp,ref stmt} => {
                print!("while");
                q_left = (*exp).as_ref();
                q_right = (*stmt).as_ref();
            },
            AST::BREAK => {
                print!("break");
            },
            AST::CONTINUE => {
                print!("continue");
            },
            AST::RETURN(ref e) => {
                println!("return");
                (*e).as_ref().print(n+1);
            },
            AST::ERR(ref s,ref line) => {
                println!("line {}:parser error:{}",line,s);
            },
        }
        match *q_left {
            AST::NULL => {},
            _ => {
                match *q_right {
                    AST::NULL => {
                        print!("\n");
                        q_left.print(n+1);
                    },
                    _ => {
                        print!("\n");
                        q_left.print(n+1);
                        print!("\n");
                        q_right.print(n+1);
                    },
                }
            },
        }
    }
}

macro_rules! err_return {
    ($fn_exp:expr) => (
        match $fn_exp {
            AST::ERR(s,line) => { return AST::ERR(s,line); },
            ast => { ast },
        }
    )
}

macro_rules! parser_expect {
    ($tokens:ident,$token:pat,$mess:expr) => (
        match $tokens.get(0,0) {
            $token => { $tokens.i+=1; },
            _ => { return AST::ERR($mess.to_string(),$tokens.get_line()); },
        }
    )
}

macro_rules! option {
    ($tokens:ident,$token:pat) => (
        loop {
            match $tokens.get(0,0) {
                $token => { $tokens.i+=1; },
                _ =>{break;},
            }
        }
    )
}

pub fn exp(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let mut eroot=err_return!(exp1(tokens));
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

pub fn exp1(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let mut eroot=err_return!(exp2(tokens));
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

pub fn exp2(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let mut eroot=err_return!(exp3(tokens));
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

pub fn exp3(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let mut eroot=err_return!(exp4(tokens));
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

pub fn exp4(tokens:&mut StatusVec<(Token,usize)>) -> AST {
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

pub fn expn(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    match tokens.get(0,0) {
        Token::NOT => {
            tokens.i+=1;
            AST::NOT(Box::new(err_return!(exp1(tokens))))
        },
        _ => {
            err_return!(exp5(tokens))
        },
    }
}

pub fn exp5(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let mut eroot=err_return!(exp7(tokens));
    loop {
        match tokens.get(0,0) {
            Token::LPAR => {
                tokens.i+=1;
                let left = AST::CALL{ exp:Box::new(eroot), arg_list:Box::new(err_return!(exp_args(tokens))) };
                parser_expect!(tokens,Token::RPAR,"expect \")\",");
                eroot = left;
            },
            Token::LSQB => {
                tokens.i+=1;
                let left = AST::INDEX{ exp:Box::new(eroot), index:Box::new(err_return!(exp_index(tokens))) };
                parser_expect!(tokens,Token::RSQB,"expect \"]\"");
                eroot = left;
            },
            Token::DOT => {
                tokens.i+=1;
                let left = AST::DOT{ left:Box::new(eroot), right:Box::new(err_return!(a_iden(tokens))) };
                eroot = left;
            },
            _ => {
                break;
            },
        }
    }
    eroot
}

#[derive(Clone,Copy)]
pub struct Status {
    in_if:isize,
    in_loop:isize,
    in_stmt:isize,
    in_fn:isize,
}

impl Status {
    pub fn new(in_if:isize,in_loop:isize,in_stmt:isize,in_fn:isize) -> Status{
        Status{in_if:in_if,in_loop:in_loop,in_stmt:in_stmt,in_fn:in_fn}
    }
}

fn goto_next_line(tokens:&mut StatusVec<(Token,usize)>) {
    loop {
        match  tokens.get(0,0) {
            Token::LF | Token::LAST => {break;}
            _ => {tokens.i+=1;}
        }
    }
}

pub fn single_stmt(tokens:&mut StatusVec<(Token,usize)>,sta:&mut Status) -> AST {
    option!(tokens,Token::LF);
    match  tokens.get(0,0) {
        Token::LBRACE => {
            sta.in_stmt += 1;
            tokens.i+=1;
            option!(tokens,Token::LF);
            let tmp = err_return!(stmt(tokens,sta));
            option!(tokens,Token::LF);
            parser_expect!(tokens,Token::RBRACE,"stmt-block lost a \"}\"");
            sta.in_stmt -= 1;
            tmp
        }
        Token::RBRACE => {
            AST::END
        }
        Token::DEF => {
            fn_def(tokens,sta)
        }
        Token::IF => {
            stmt_if(tokens,sta)
        }
        Token::WHILE => {
            sta.in_loop += 1;
            let tmp = err_return!(stmt_while(tokens,sta));
            sta.in_loop -= 1;
            tmp
        }
        Token::BREAK => {
            if sta.in_loop > 0 {
                a_brk(tokens)
            }else{
                AST::ERR("\"break\" must in a loop-struct !".to_string(),tokens.get_line())
            }
        }
        Token::RETURN => {
            if sta.in_fn > 0 {
                a_rtn(tokens)
            }else{
                AST::ERR("\"return\" must in a function define !".to_string(),tokens.get_line())
            }
        }
        Token::CONTINUE => {
            if sta.in_loop > 0 {
                a_ctn(tokens)
            }else{
                AST::ERR("\"continue\" must in a loop-struct !".to_string(),tokens.get_line())
            }
        }
          Token::IDEN(_)
        | Token::INT(_) 
        | Token::STR(_)
        | Token::MINUS
        | Token::NOT
        | Token::FLOAT(_)
        | Token::LPAR
        => {
            let first_exp = err_return!(exp(tokens));
            match tokens.get(0,0) {
                Token::EQ => {
                    err_return!(stmt_assign(tokens,true,first_exp))
                }
                _ => first_exp
            }
        }
        Token::VAR => {
            err_return!(stmt_var(tokens))
        }
        Token::LAST => {
            AST::END
        }
        _ => {
            AST::ERR("expect stmt or stmt-block !".to_string(),tokens.get_line())
        }
    }
}

macro_rules! add_stmt {
    ($tokens:ident,$vec:ident,$ast:expr) => (
        {
            let a = $ast;
            match a {
                AST::ERR(_,_) => {
                    unsafe { err_status::parser_err = true; }
                    a.print(0);
                    goto_next_line($tokens);
                }
                _ => {$vec.push(a);}
            }
        }
    )
}



pub fn stmt(tokens:&mut StatusVec<(Token,usize)>,sta:&mut Status) -> AST {
    let mut stmt_vec:Vec<AST> = Vec::new();
    option!(tokens,Token::LF);
    add_stmt!(tokens,stmt_vec,single_stmt(tokens,sta));
    loop {
        match tokens.get(0,0) {
            Token::LF => {
                tokens.i+=1;
                match tokens.get(0,0) {
                    Token::RBRACE | Token::LAST => {}
                    _ => {add_stmt!(tokens,stmt_vec,single_stmt(tokens,sta));}
                }
            }
            Token::LBRACE => {
                add_stmt!(tokens,stmt_vec,single_stmt(tokens,sta));
            }
            Token::LAST => {
                stmt_vec.push(AST::END);
                break;
            }
            Token::RBRACE => {
                if sta.in_stmt>0 {break;}
                else {
                    add_stmt!(tokens,stmt_vec,AST::ERR("find \"}\" but \"{\" miss".to_string(),tokens.get_line()));
                    //return AST::ERR("find \"}\" but \"{\" miss".to_string(),tokens.get_line());
                }
            }
            _ => {
                add_stmt!(tokens,stmt_vec,AST::ERR("expect a new line!".to_string(),tokens.get_line()));
                //return AST::ERR("expect a new line!".to_string(),tokens.get_line());
            }
        }
    }
    AST::STMT(stmt_vec)
}

pub fn stmt_if(tokens:&mut StatusVec<(Token,usize)>,sta:&mut Status) -> AST {
    parser_expect!(tokens,Token::IF,"expect \"if\"");
    let condition = err_return!(exp(tokens));
    option!(tokens,Token::LF);
    let stmt = err_return!(single_stmt(tokens,sta));
    let mut else_stmt = AST::NULL;
    match tokens.get(0,0) {
        Token::ELSE => {
            tokens.i+=1;
            option!(tokens,Token::LF);
            else_stmt = err_return!(single_stmt(tokens,sta));
        }
        _ => {}
    }
    AST::IF{ exp:Box::new(condition), stmt:Box::new(stmt), else_stmt:Box::new(else_stmt) }
}

pub fn stmt_assign(tokens:&mut StatusVec<(Token,usize)>,has_left:bool,left:AST) -> AST {
    //标记：
    //直接使用exp来parser左值，不合理
    //比如 3+4 = foo()  这种错误要延迟到解释阶段处理
    let left_value = if !has_left { err_return!(exp(tokens)) } else { left };
    parser_expect!(tokens,Token::EQ,"expect a \"=\" !");
    let right_value = err_return!(exp(tokens));
    AST::ASSIGN{ left_value:Box::new(left_value), exp:Box::new(right_value) }
}

pub fn stmt_var(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    parser_expect!(tokens,Token::VAR,"expect a \"var\" !");
    let iden = err_return!(a_iden(tokens));
    match tokens.get(0,0) {
        Token::LF => {
            AST::VAR{ iden:Box::new(iden),exp:Box::new(AST::NULL) }
        }
        _ => { 
            parser_expect!(tokens,Token::EQ,"expect a \"=\" !");
            AST::VAR{ iden:Box::new(iden), exp:Box::new(err_return!(exp(tokens))) } 
        }
    }
}

pub fn stmt_while(tokens:&mut StatusVec<(Token,usize)>,sta:&mut Status) -> AST {
    parser_expect!(tokens,Token::WHILE,"expect \"while\"");
    let condition = err_return!(exp(tokens));
    option!(tokens,Token::LF);
    let stmt = err_return!(single_stmt(tokens,sta));
    AST::WHILE{ exp:Box::new(condition), stmt:Box::new(stmt) }
}

pub fn exp_index(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    err_return!(exp(tokens))
}

pub fn fn_def(tokens:&mut StatusVec<(Token,usize)>,sta:&mut Status) -> AST {
    parser_expect!(tokens,Token::DEF,"function define must be use a \"def\"");
    let fun_name = err_return!(a_iden(tokens));
    parser_expect!(tokens,Token::LPAR,"function define have no arguments,expect a \"(...)\"");
    let fun_args = err_return!(def_args(tokens));
    parser_expect!(tokens,Token::RPAR,"function arguments must be closed with \")\"");
    sta.in_fn+=1;
    let fun_stmt = err_return!(single_stmt(tokens,sta));
    sta.in_fn-=1;
    AST::DEF { iden:Box::new(fun_name),args:Box::new(fun_args),stmt:Box::new(fun_stmt) }
}

pub fn def_args(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    enum ArgType {
        Must,
        Default,
    }
    let mut arg_type = ArgType::Must;
    let mut args:Vec<AST> = Vec::new();
    macro_rules! match_first_de_val {
        ($tmp:expr) => {
            match tokens.get(0,0) {
                Token::EQ => {
                    tokens.i+=1;
                    let de_val = AST::DEVAL{ iden:Box::new($tmp),val:Box::new(err_return!(exp(tokens))) };
                    args.push(de_val);
                    arg_type = ArgType::Default;
                }
                _ => {
                    args.push($tmp);
                }
            }
        }
    }
    match tokens.get(0,0) {
        Token::RPAR => {
            
        },
        _ => {
            let tmp = err_return!(a_iden(tokens));
            match_first_de_val!(tmp);
            loop{
                match tokens.get(0,0) {
                    Token::COMMA => {
                        tokens.i+=1;
                        match arg_type {
                            ArgType::Default => {
                                args.push(err_return!(exp_default_value(tokens)));
                            }
                            ArgType::Must => {
                                let tmp = err_return!(a_iden(tokens));
                                match_first_de_val!(tmp);
                            }
                        }
                    }
                    _ => {break;}
                }
            }
        },
    }
    AST::ARGS(args)
}

pub fn exp_args(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    enum ArgType {
        Must,
        Default,
    }
    let mut arg_type = ArgType::Must;
    let mut args:Vec<AST> = Vec::new();
    macro_rules! match_first_de_val {
        ($tmp:expr) => {
            match $tmp {
                AST::IDEN(_) => {
                    match tokens.get(0,0) {
                        Token::EQ => {
                            tokens.i+=1;
                            let de_val = AST::DEVAL{ iden:Box::new($tmp),val:Box::new(err_return!(exp(tokens))) };
                            args.push(de_val);
                            arg_type = ArgType::Default;
                        }
                        _ => {
                            args.push($tmp);
                        }
                    }
                }
                _ => {
                    args.push($tmp);
                }
            }
        }
    }
    match tokens.get(0,0) {
        Token::RPAR => {
            
        },
        _ => {
            let tmp = err_return!(exp(tokens));
            match_first_de_val!(tmp);
            loop{
                match tokens.get(0,0) {
                    Token::COMMA => {
                        tokens.i+=1;
                        match arg_type {
                            ArgType::Default => {
                                args.push(err_return!(exp_default_value(tokens)));
                            }
                            ArgType::Must => {
                                let tmp = err_return!(exp(tokens));
                                match_first_de_val!(tmp);
                            }
                        }
                    }
                    _ => {break;}
                }
            }
        },
    }
    AST::ARGS(args)
}

pub fn exp_default_value(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let tmp = err_return!(exp(tokens));
    match tmp {
        AST::IDEN(_) => {
            parser_expect!(tokens,Token::EQ,"in default value expect \"=\"");
            AST::DEVAL{ iden:Box::new(tmp),val:Box::new(err_return!(exp(tokens))) }
        }
        _ => { AST::ERR("expect default value , but left-value not a iden !".to_string(),tokens.get_line()) },
    }
}

pub fn exp7(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::TRUE     =>    { a_bool(tokens) },
        Token::FALSE    =>    { a_bool(tokens) },
        Token::INT(_)   =>    { a_int(tokens) },
        Token::FLOAT(_) =>    { a_float(tokens) },
        Token::STR(_)   =>    { a_str(tokens) },
        Token::IDEN(_)  =>    { a_iden(tokens) },
        Token::LPAR     => {
            tokens.i+=1;
            let tmp = err_return!(exp(tokens));
            parser_expect!(tokens,Token::RPAR,"expect \")\"");
            tmp
        }
        _ => { AST::ERR("expect min-exp !".to_string(),tokens.get_line()) },
    }
}

pub fn a_bool(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::TRUE => { tokens.i+=1; AST::TRUE },
        Token::FALSE => { tokens.i+=1; AST::FALSE },
        _ => { AST::ERR("expect \"true\" or \"false\"".to_string(),tokens.get_line()) },
    }
}

pub fn a_int(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::INT(s) => { tokens.i+=1; AST::INT(s) },
        _ => { AST::ERR("expect int !".to_string(),tokens.get_line()) },
    }
}

pub fn a_float(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::FLOAT(s) => { tokens.i+=1; AST::FLOAT(s) },
        _ => { AST::ERR("expect float !".to_string(),tokens.get_line()) },
    }
}

pub fn a_str(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::STR(s) => { tokens.i+=1; AST::STR(s) },
        _ => { AST::ERR("expect str !".to_string(),tokens.get_line()) },
    }
}

pub fn a_iden(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::IDEN(s) => { tokens.i+=1; AST::IDEN(s) },
        _ => { AST::ERR("expect identifier !".to_string(),tokens.get_line()) },
    }
}

pub fn a_brk(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::BREAK => { tokens.i+=1; AST::BREAK },
        _ => { AST::ERR("expect \"break\"".to_string(),tokens.get_line()) },
    }
}

pub fn a_ctn(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::CONTINUE => { tokens.i+=1; AST::CONTINUE },
        _ => { AST::ERR("expect \"continue\" !".to_string(),tokens.get_line()) },
    }
}

pub fn a_rtn(tokens:&mut StatusVec<(Token,usize)>) -> AST {
    let t = tokens.get(0,0);
    match t {
        Token::RETURN => { 
            tokens.i+=1;
            match tokens.get(0,0) {
                Token::LF | Token::RBRACE => {
                    AST::RETURN(Box::new(AST::NULL))
                }
                _ => AST::RETURN(Box::new(err_return!(exp(tokens))))
            }
        },
        _ => { AST::ERR("expect \"continue\" !".to_string(),tokens.get_line()) },
    }
}