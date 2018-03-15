use xt_type::*;
use parser::*;
use std::collections::HashMap;

type SymbolBlock = HashMap<String, XtValue>;
type SymbolTable = Vec<SymbolBlock>;

struct Context {
    symbol_table: SymbolTable,
}

fn run_exp(ast: AST) -> Box<XtValue> {
    new_int(42)
}

enum StmtInfo {
    RETURN(XtValue),
    BREAK,
    CONTINUE,
    OTHER,
}

fn run_var(context: &mut Context, iden: Box<AST>, exp: Box<AST>) -> Result<StmtInfo, String> {
    let iden_name = match *iden {
        AST::STR(nstr) => {
            Some(nstr)
        }
        _ => { None }
    };
    if let Some(last) = context.symbol_table.last_mut() {
        if let Some(nstr) = iden_name {
            last.insert(nstr, *run_exp(*exp));
        } else {
            // TODO left node is not string
        }
    } else {
        // TODO table len is 0
        // No, symbol table always more than 0
    }
    Result::Ok(StmtInfo::OTHER)
}

pub fn run(ast: AST) -> Result<isize, isize> {
    println!("=========interpreter start========");

    let mut symbol_table: SymbolTable = Vec::new();
    symbol_table.push(SymbolBlock::new());
    let mut context = Context { symbol_table: symbol_table };
    match ast {
        AST::VAR { iden, exp } => {
            run_var(&mut context, iden, exp);
        }
        _ => {}
    }
    println!("==========interpreter end=========");
    Result::Ok(0)
}