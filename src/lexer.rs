use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Clone)]
pub enum Token{
    //字面量
    STR(String),
    INT(isize),
    FLOAT(f64),
    //运算符号
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERCENT,
    LT,
    LE,
    GT,
    GE,
    EQEQ,
    NE,
    ANDAND,
    OROR,
    //其他符号
    EQ,
    COMMA,
    DOT,
    LPAR,
    RPAR,
    LSQB,
    RSQB,
    LBRACE,
    RBRACE,
    IDEN(String),
    LF,
    //关键字
    VAR,
    IF,
    WHILE,
    NOT,
    //FOR,
    //结束
    LAST,
    //错误
    ERR(String),
}

impl fmt::Display for Token{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::STR(ref s)   => write!(f, "str:'{}'", s),
            Token::INT(ref s)   => write!(f, "int:{}", s),
            Token::FLOAT(ref s) => write!(f, "float:{}", s),
            Token::PLUS         => write!(f, "+"),
            Token::MINUS        => write!(f, "-"),
            Token::STAR         => write!(f, "*"),
            Token::SLASH        => write!(f, "/"),
            Token::PERCENT      => write!(f, "%"),
            Token::LT           => write!(f, "<"),
            Token::LE           => write!(f, "<="),
            Token::GT           => write!(f, ">"),
            Token::GE           => write!(f, ">="),
            Token::EQEQ         => write!(f, "=="),
            Token::NE           => write!(f, "!="),
            Token::ANDAND       => write!(f, "&&"),
            Token::OROR         => write!(f, "||"),
            Token::EQ           => write!(f, "="),
            Token::COMMA        => write!(f, ","),
            Token::DOT          => write!(f, "."),
            Token::LPAR         => write!(f, "("),
            Token::RPAR         => write!(f, ")"),
            Token::LSQB         => write!(f, "["),
            Token::RSQB         => write!(f, "]"),
            Token::LBRACE       => write!(f, "{{"),
            Token::RBRACE       => write!(f, "}}"),
            Token::IDEN(ref s)  => write!(f, "id: {}", s),
            Token::LF           => write!(f, "\n"),
            Token::VAR          => write!(f, "var"),
            Token::IF           => write!(f, "if"),
            Token::WHILE        => write!(f, "while"),
            Token::NOT          => write!(f, "not"),
            Token::LAST         => write!(f, "last"),
            Token::ERR(ref s)   => write!(f, "err:{}", s),
        }
    }
}

pub struct StatusVec<T>{
    pub i:usize,
    pub vec_data:Vec<T>,
}

fn is_iden_start(c:char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' | '_' => true,
        _ => false,
    }
}

fn is_iden(c:char) -> bool {
    match c {
        '0'...'9' | 'a'...'z' | 'A'...'Z' | '_' => true,
        _ => false,
    }
}

fn is_num_start(c:char) -> bool {
    match c {
        '0'...'9' => true,
        _ => false,
    }
}

fn is_num(c:char) -> bool {
    match c {
        '0'...'9' => true,
        _ => false,
    }
}

fn is_str_start(c:char) -> bool {
    match c {
        '\"' => true,
        _ => false,
    }
}

impl StatusVec<Token> {
    pub fn get(&mut self,i:usize,m:usize) -> Token{
        if (self).i + i >= (self).vec_data.len() { 
            Token::LAST
        }
        else {
            let t = self.vec_data[self.i + i].clone();
            self.i += m;
            t
        }
    }
}

impl StatusVec<char> {

    fn now_char(&self) -> Option<char> {
        if self.i < self.vec_data.len() {Some(self.vec_data[self.i])}
        else { None }
    }
    
    fn goto_useful(&mut self) {
        loop {
            if let Some(now) = self.now_char() {
                if now == '\r'
                || now == ' '
                || now == '\t' {
                    self.i += 1;
                }else{
                    break;
                }
            }
            else{
                break;
            }
        }
    }
    
    fn next(&mut self) -> Option<Token>{
        self.goto_useful();
        let mut tmp_str = String::new();
        match self.now_char() {
            Some(c) => {
                //关键字和标识符
                if is_iden_start(c) {
                    self.i += 1;
                    tmp_str.push(c);
                    loop {
                        match self.now_char() {
                            Some(c2) => {
                                if is_iden(c2) {
                                    self.i += 1;
                                    tmp_str.push(c2);
                                }
                                else { break; }
                            },
                            _ => {break;},
                        }
                    }
                    let keywords = get_keywords();
                    match keywords.get(&*tmp_str) {
                        Some(k) => {
                            return Some(k.clone());
                        },
                        None => {
                            let t = Token::IDEN(tmp_str);
                            return Some(t);
                        },
                    }
                }
                else if is_num_start(c) {
                    let mut is_float:bool = false;
                    self.i+=1;
                    tmp_str.push(c);
                    loop {
                        match self.now_char() {
                            Some(c2) => {
                                if is_num(c2) {
                                    self.i += 1;
                                    tmp_str.push(c2);
                                }
                                else if c2=='.' && is_float == false {
                                    self.i += 1;
                                    tmp_str.push(c2);
                                    is_float = true;
                                }
                                else {break;}
                            },
                            None => {break;},
                        }
                    }
                    return if is_float==true { Some(Token::FLOAT(f64::from_str(tmp_str.as_ref()).unwrap())) } else { Some(Token::INT(isize::from_str(tmp_str.as_ref()).unwrap())) }
                }
                else if is_str_start(c) {
                    let mut is_end = false;
                    self.i+=1;
                    loop {
                        match self.now_char() {
                            Some(c2) => {
                                self.i+=1;
                                if c2=='\\' {
                                    if let Some(next) = self.now_char() {
                                        self.i+=1;
                                        match next {
                                        'n' => { tmp_str.push('\n') },
                                        'r' => { tmp_str.push('\r') },
                                        't' => { tmp_str.push('\t') },
                                        '\\'=> { tmp_str.push('\\') },
                                        '0' => { tmp_str.push('\0') },
                                        '\"'=> {  },
                                        _   => { self.i-=1;tmp_str.push('\\'); }
                                        }
                                    }
                                }
                                else if c2=='\"' { is_end=true;break; }
                                else {
                                    tmp_str.push(c2);
                                }
                            },
                            None => { 
                                return Some(Token::ERR("has no end of str \"".to_string()));
                            },
                        }
                    }
                    return Some(Token::STR(tmp_str));
                }
                else {
                    self.i += 1;
                    return match c {
                        //运算符
                        '+' => { Some(Token::PLUS) },
                        '-' => { Some(Token::MINUS) },
                        '*' => { Some(Token::STAR) },
                        '/' => { Some(Token::SLASH) },
                        '%' => { Some(Token::PERCENT) },
                        '<' => {
                                    if let Some(next) = self.now_char() {
                                        if next == '=' { self.i += 1; Some(Token::LE) }
                                        else { Some(Token::LT) }
                                    }
                                    else { Some(Token::LT) }
                               }
                        '>' => {
                                    if let Some(next) = self.now_char() {
                                        if next == '=' { self.i += 1; Some(Token::GE) }
                                        else { Some(Token::GT) }
                                    }
                                    else { Some(Token::GT) }
                               }
                        '=' => {
                                    if let Some(next) = self.now_char() {
                                        if next == '=' { self.i += 1; Some(Token::EQEQ) }
                                        else { Some(Token::EQ) }
                                    }
                                    else { Some(Token::EQ) }
                               },
                        '!' => {
                                    if let Some(next) = self.now_char() {
                                        if next == '=' { self.i += 1; Some(Token::NE) }
                                        else { Some(Token::ERR("unknown token \"!\"".to_string())) }
                                    }
                                    else { Some(Token::ERR("unknown token \"!\"".to_string())) }
                               }
                        '&' => {
                                    if let Some(next) = self.now_char() {
                                        if next == '&' { self.i += 1; Some(Token::ANDAND) }
                                        else { Some(Token::ERR("unknown token \"&\"".to_string())) }
                                    }
                                    else { Some(Token::ERR("unknown token \"&\"".to_string())) }
                               }
                        '|' => {
                                    if let Some(next) = self.now_char() {
                                        if next == '|' { self.i += 1; Some(Token::OROR) }
                                        else { Some(Token::ERR("unknown token \"|\"".to_string())) }
                                    }
                                    else { Some(Token::ERR("unknown token \"|\"".to_string())) }
                               }
                        '(' => { Some(Token::LPAR) },
                        ')' => { Some(Token::RPAR) },
                        '[' => { Some(Token::LSQB) },
                        ']' => { Some(Token::RSQB) },
                        '{' => { Some(Token::LBRACE) },
                        '}' => { Some(Token::RBRACE) },
                        ',' => { Some(Token::COMMA) },
                        '.' => { Some(Token::DOT) },
                        '\n'=> { Some(Token::LF) },
                         _  => { Some(Token::ERR("".to_string())) },
                    }
                }
            },
            None => { return None; },
        }
    }
    
}

pub fn get_keywords() -> HashMap<&'static str,Token> {
    let mut keywords = HashMap::new();
    keywords.insert("var",Token::VAR);
    keywords.insert("if",Token::IF);
    keywords.insert("while",Token::WHILE);
    keywords.insert("not",Token::NOT);
    keywords
}

pub fn get_char_vec(path:&Path) -> Vec<char> {
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

pub fn get_tokens_from(path:&Path) -> StatusVec<Token>{
    let char_vec = get_char_vec(path);
    let mut tokens: Vec<Token> = Vec::new();
    let mut read_token = StatusVec::<char>{i:0,vec_data:char_vec};
    loop {
        let t = read_token.next();
        match t {
            Some(t) => {
                tokens.push(t);
            },
            None => {break;},
        }
    }
    StatusVec::<Token> { i: 0, vec_data: tokens }
}