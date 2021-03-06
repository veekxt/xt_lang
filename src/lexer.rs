use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Clone)]
pub enum Token {
    // literal
    STR(String),
    INT(isize),
    FLOAT(f64),
    // operational character
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
    // other character
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
    // keyword
    VAR,
    DEF,
    CLASS,
    IF,
    ELSE,
    WHILE,
    BREAK,
    CONTINUE,
    RETURN,
    NOT,
    TRUE,
    FALSE,
    // FOR
    // end
    LAST,
    // mistake
    ERR(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::STR(ref s) => write!(f, "str:'{}'", s),
            Token::INT(ref s) => write!(f, "int:{}", s),
            Token::FLOAT(ref s) => write!(f, "float:{}", s),
            Token::PLUS => write!(f, "+"),
            Token::MINUS => write!(f, "-"),
            Token::STAR => write!(f, "*"),
            Token::SLASH => write!(f, "/"),
            Token::PERCENT => write!(f, "%"),
            Token::LT => write!(f, "<"),
            Token::LE => write!(f, "<="),
            Token::GT => write!(f, ">"),
            Token::GE => write!(f, ">="),
            Token::EQEQ => write!(f, "=="),
            Token::NE => write!(f, "!="),
            Token::ANDAND => write!(f, "&&"),
            Token::OROR => write!(f, "||"),
            Token::EQ => write!(f, "="),
            Token::COMMA => write!(f, ","),
            Token::DOT => write!(f, "."),
            Token::LPAR => write!(f, "("),
            Token::RPAR => write!(f, ")"),
            Token::LSQB => write!(f, "["),
            Token::RSQB => write!(f, "]"),
            Token::LBRACE => write!(f, "{{"),
            Token::RBRACE => write!(f, "}}"),
            Token::IDEN(ref s) => write!(f, "id: {}", s),
            Token::LF => write!(f, "\\n"),
            Token::VAR => write!(f, "var"),
            Token::DEF => write!(f, "def"),
            Token::CLASS => write!(f, "class"),
            Token::IF => write!(f, "if"),
            Token::ELSE => write!(f, "ELSE"),
            Token::WHILE => write!(f, "while"),
            Token::BREAK => write!(f, "break"),
            Token::CONTINUE => write!(f, "continue"),
            Token::RETURN => write!(f, "return"),
            Token::NOT => write!(f, "not"),
            Token::TRUE => write!(f, "true"),
            Token::FALSE => write!(f, "false"),
            Token::LAST => write!(f, "last"),
            Token::ERR(ref s) => write!(f, "err:{}", s),
        }
    }
}

pub struct StatusVec<T> {
    pub i: usize,
    pub vec_data: Vec<T>,
}

pub struct LineChars {
    pub i: usize,
    pub line: usize,
    pub vec_data: Vec<char>,
    pub keywords: HashMap<&'static str, Token>,
}

fn is_iden_start(c: char) -> bool {
    match c {
        'a' ... 'z' | 'A' ... 'Z' | '_' => true,
        _ => false,
    }
}

fn is_iden(c: char) -> bool {
    match c {
        '0' ... '9' | 'a' ... 'z' | 'A' ... 'Z' | '_' => true,
        _ => false,
    }
}

fn is_num_start(c: char) -> bool {
    match c {
        '0' ... '9' => true,
        _ => false,
    }
}

fn is_num(c: char) -> bool {
    match c {
        '0' ... '9' => true,
        _ => false,
    }
}

fn is_str_start(c: char) -> bool {
    match c {
        '\"' => true,
        _ => false,
    }
}

fn is_comment_start(c: char) -> bool {
    match c {
        '#' => true,
        _ => false,
    }
}

impl StatusVec<(Token, usize)> {
    pub fn get(&mut self, i: usize, m: usize) -> Token {
        if (self).i + i >= (self).vec_data.len() {
            Token::LAST
        } else {
            let (t, _) = self.vec_data[self.i + i].clone();
            self.i += m;
            t
        }
    }
    pub fn get_line(&mut self) -> usize {
        if (self).i >= (self).vec_data.len() {
            let (_, line) = self.vec_data[self.vec_data.len() - 1];
            line
        } else {
            let (_, line) = self.vec_data[self.i];
            line
        }
    }
}

impl LineChars {
    fn now_char(&self) -> Option<char> {
        if self.i < self.vec_data.len() { Some(self.vec_data[self.i]) } else { None }
    }

    fn goto_useful(&mut self) {
        loop {
            if let Some(now) = self.now_char() {
                if now == '\r'
                    || now == ' '
                    || now == '\t' {
                    self.i += 1;
                } else if is_comment_start(now) {
                    self.i += 1;
                    while let Some(now) = self.now_char() {
                        if now == '\n' {
                            self.i += 1;
                            self.line += 1;
                            break;
                        } else {
                            self.i += 1;
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn next(&mut self) -> (Option<Token>, usize) {
        let rs: Option<Token>;
        self.goto_useful();
        let mut tmp_str = String::new();
        match self.now_char() {
            Some(c) => {
                // keyword or identifier
                if is_iden_start(c) {
                    self.i += 1;
                    tmp_str.push(c);
                    loop {
                        match self.now_char() {
                            Some(c2) => {
                                if is_iden(c2) {
                                    self.i += 1;
                                    tmp_str.push(c2);
                                } else { break; }
                            }
                            _ => { break; }
                        }
                    }
                    match self.keywords.get(&*tmp_str) {
                        Some(k) => {
                            rs = Some(k.clone());
                        }
                        None => {
                            let t = Token::IDEN(tmp_str);
                            rs = Some(t);
                        }
                    }
                } else if is_num_start(c) {
                    let mut is_float: bool = false;
                    self.i += 1;
                    tmp_str.push(c);
                    loop {
                        match self.now_char() {
                            Some(c2) => {
                                if is_num(c2) {
                                    self.i += 1;
                                    tmp_str.push(c2);
                                } else if c2 == '.' && is_float == false {
                                    self.i += 1;
                                    if let Some(c3) = self.now_char() {
                                        if !is_num(c3) {
                                            self.i -= 1;
                                            break;
                                        } else {
                                            tmp_str.push(c2);
                                        }
                                    } else {
                                        self.i -= 1;
                                        break;
                                    }
                                    is_float = true;
                                } else { break; }
                            }
                            None => { break; }
                        }
                    }
                    rs = if is_float == true { Some(Token::FLOAT(f64::from_str(tmp_str.as_ref()).unwrap())) } else { Some(Token::INT(isize::from_str(tmp_str.as_ref()).unwrap())) }
                } else if is_str_start(c) {
                    self.i += 1;
                    loop {
                        match self.now_char() {
                            Some(c2) => {
                                self.i += 1;
                                if c2 == '\\' {
                                    if let Some(next) = self.now_char() {
                                        self.i += 1;
                                        match next {
                                            'n' => { tmp_str.push('\n') }
                                            'r' => { tmp_str.push('\r') }
                                            't' => { tmp_str.push('\t') }
                                            '\\' => { tmp_str.push('\\') }
                                            '0' => { tmp_str.push('\0') }
                                            '\"' => {}
                                            _ => {
                                                self.i -= 1;
                                                tmp_str.push('\\');
                                            }
                                        }
                                    }
                                } else if c2 == '\"' { break; } else {
                                    tmp_str.push(c2);
                                }
                            }
                            None => {
                                return (Some(Token::ERR("has no end of str \"".to_string())), self.line);
                                //break;
                            }
                        }
                    }
                    rs = Some(Token::STR(tmp_str));
                } else {
                    self.i += 1;
                    rs = match c {
                        // operational character
                        '+' => { Some(Token::PLUS) }
                        '-' => { Some(Token::MINUS) }
                        '*' => { Some(Token::STAR) }
                        '/' => { Some(Token::SLASH) }
                        '%' => { Some(Token::PERCENT) }
                        '<' => {
                            if let Some(next) = self.now_char() {
                                if next == '=' {
                                    self.i += 1;
                                    Some(Token::LE)
                                } else { Some(Token::LT) }
                            } else { Some(Token::LT) }
                        }
                        '>' => {
                            if let Some(next) = self.now_char() {
                                if next == '=' {
                                    self.i += 1;
                                    Some(Token::GE)
                                } else { Some(Token::GT) }
                            } else { Some(Token::GT) }
                        }
                        '=' => {
                            if let Some(next) = self.now_char() {
                                if next == '=' {
                                    self.i += 1;
                                    Some(Token::EQEQ)
                                } else { Some(Token::EQ) }
                            } else { Some(Token::EQ) }
                        }
                        '!' => {
                            if let Some(next) = self.now_char() {
                                if next == '=' {
                                    self.i += 1;
                                    Some(Token::NE)
                                } else { Some(Token::ERR("unknown token \"!\"".to_string())) }
                            } else { Some(Token::ERR("unknown token \"!\"".to_string())) }
                        }
                        '&' => {
                            if let Some(next) = self.now_char() {
                                if next == '&' {
                                    self.i += 1;
                                    Some(Token::ANDAND)
                                } else { Some(Token::ERR("unknown token \"&\"".to_string())) }
                            } else { Some(Token::ERR("unknown token \"&\"".to_string())) }
                        }
                        '|' => {
                            if let Some(next) = self.now_char() {
                                if next == '|' {
                                    self.i += 1;
                                    Some(Token::OROR)
                                } else { Some(Token::ERR("unknown token \"|\"".to_string())) }
                            } else { Some(Token::ERR("unknown token \"|\"".to_string())) }
                        }
                        '(' => { Some(Token::LPAR) }
                        ')' => { Some(Token::RPAR) }
                        '[' => { Some(Token::LSQB) }
                        ']' => { Some(Token::RSQB) }
                        '{' => { Some(Token::LBRACE) }
                        '}' => { Some(Token::RBRACE) }
                        ',' => { Some(Token::COMMA) }
                        '.' => { Some(Token::DOT) }
                        '\n' => {
                            let mut lines = 0;
                            lines += 1;
                            while let Some(next) = self.now_char() {
                                if next == '\n' {
                                    self.i += 1;
                                    lines += 1;
                                } else {
                                    break;
                                }
                            }
                            let tmp = (Some(Token::LF), self.line);
                            self.line += lines;
                            return tmp;
                        }
                        _ => { Some(Token::ERR("unknown token!".to_string())) }
                    }
                }
            }
            None => { rs = None; }
        }
        (rs, self.line)
    }
}

fn get_keywords() -> HashMap<&'static str, Token> {
    let mut keywords = HashMap::new();
    keywords.insert("var", Token::VAR);
    keywords.insert("def", Token::DEF);
    keywords.insert("class", Token::CLASS);
    keywords.insert("if", Token::IF);
    keywords.insert("else", Token::ELSE);
    keywords.insert("while", Token::WHILE);
    keywords.insert("break", Token::BREAK);
    keywords.insert("continue", Token::CONTINUE);
    keywords.insert("return", Token::RETURN);
    keywords.insert("not", Token::NOT);
    keywords.insert("true", Token::TRUE);
    keywords.insert("false", Token::FALSE);
    keywords
}

pub fn init_lexer_context() -> (HashMap<&'static str, Token>, ) {
    (get_keywords(), )
}

pub fn get_char_vec(path: &Path) -> Vec<char> {
    let mut f = match File::open(path) {
        Ok(r) => r,
        _ => panic!("can't open file !"),
    };
    let mut buffer = String::new();
    match f.read_to_string(&mut buffer) {
        Ok(r) => r,
        _ => panic!("can't convent file to string !"),
    };
    let str_vec: Vec<char> = buffer.chars().collect();
    str_vec
}

pub fn get_tokens_from(path: &Path) -> (StatusVec<(Token, usize)>, bool) {
    let mut err = false;
    let char_vec = get_char_vec(path);
    let mut tokens: Vec<(Token, usize)> = Vec::new();
    let (keywords, ) = init_lexer_context();
    let mut read_token = LineChars { i: 0, line: 1, vec_data: char_vec, keywords };
    loop {
        let (t, line) = read_token.next();
        match t {
            Some(to) => {
                tokens.push((to.clone(), line));
                match to {
                    Token::ERR(ref s) => {
                        err = true;
                        println!("line {}:lexer error:{}", line, s);
                    }
                    _ => {}
                }
            }
            None => { break; }
        }
    }
    (StatusVec::<(Token, usize)> { i: 0, vec_data: tokens }, err)
}