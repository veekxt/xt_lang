use std::fmt;

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