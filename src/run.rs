use xt_type::*;
use parser::*;
use std::collections::HashMap;

type SymbolBlock = HashMap<String, XtValue>;
type SymbolTable = Vec<SymbolBlock>;

struct Context {
    symbol_table: SymbolTable,
}

fn run_exp(ast: &Box<AST>) -> Box<XtValue> {
    match **ast {
        AST::STMT(_) => {},
        AST::VAR { .. } => {},
        AST::IF { .. } => {},
        AST::WHILE { .. } => {},
        AST::BREAK => {},
        AST::CONTINUE => {},
        AST::RETURN(_) => {},
        AST::CALL { .. } => {},
        AST::ARGS(_) => {},
        AST::ATTRS(_) => {},
        AST::FUNCTIONS(_) => {},
        AST::CLASS { .. } => {},
        AST::INDEX { .. } => {},
        AST::ASSIGN { .. } => {},
        AST::DEVAL { .. } => {},
        AST::DEF { .. } => {},
        AST::PLUS { .. } => {},
        AST::MINUS { .. } => {},
        AST::NEG(_) => {},
        AST::MUL { .. } => {},
        AST::DIV { .. } => {},
        AST::MOD { .. } => {},
        AST::LT { .. } => {},
        AST::LE { .. } => {},
        AST::GT { .. } => {},
        AST::GE { .. } => {},
        AST::EQ { .. } => {},
        AST::NE { .. } => {},
        AST::ANDAND { .. } => {},
        AST::OROR { .. } => {},
        AST::DOT { .. } => {},
        AST::NOT(_) => {},
        AST::FALSE => {},
        AST::TRUE => {},
        AST::INT(_) => {},
        AST::FLOAT(_) => {},
        AST::STR(_) => {},
        AST::IDEN(_) => {},
        AST::NULL => {},
        AST::END => {},
        AST::ERR(_, _) => {},
    }
    new_int(42)
}

enum StmtInfo {
    RETURN(XtValue),
    BREAK,
    CONTINUE,
    OTHER,
}

fn run_var(context: &mut Context, iden: &Box<AST>, exp: &Box<AST>) -> Result<StmtInfo, String> {
    let iden_name = match **iden {
        AST::IDEN(ref nstr) => {
            Some(nstr)
        }
        _ => {
            None
        }
    };
    if let Some(last) = context.symbol_table.last_mut() {
        if let Some(nstr) = iden_name {
            last.insert(nstr.to_string(), *run_exp(exp));
            println!("Add new symbol: {}", nstr);
        } else {
            // TODO left node is not string
        }
    } else {

    }
    Result::Ok(StmtInfo::OTHER)
}

fn run_stmt(context: &mut Context, ast_list: &Vec<AST>) -> Result<StmtInfo, String> {
    for ast in ast_list.iter() {
        match ast {
            &AST::VAR { ref iden, ref exp } => {
                run_var(context, iden, exp);
            }
            &AST::STMT(ref stmt) => {
                run_stmt(context,stmt);
            }
            _ => {}
        }
    }
    Result::Ok(StmtInfo::OTHER)
}

pub fn run(ast: AST) -> Result<isize, isize> {
    println!("=========interpreter start========");

    let mut symbol_table: SymbolTable = Vec::new();
    symbol_table.push(SymbolBlock::new());
    let mut context = Context { symbol_table };

    if let AST::STMT(ast_vec) = ast {
        run_stmt(&mut context, &ast_vec);
    }

    println!("==========interpreter end=========");
    Result::Ok(0)
}