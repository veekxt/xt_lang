use parser::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct XtValueType {
    name: String,
    // maybe other
}

//struct ValueAttr {
//    name:String,
//    xtype:XtValue,
//}


pub enum XtFunction {
    SysFunction(fn(&XtValue) -> ()),
    DefFunction(Box<AST>),
}

pub struct XtValue {
    pub id: i64,
    pub xt_type: XtValueType,
    pub attribute: HashMap<String, XtValue>,
    pub method: HashMap<String,XtFunction>,
}

pub fn new_int(i: i64) -> Box<XtValue> {
    let mut val = XtValue {
        id: 0,
        xt_type: { XtValueType { name: String::from("__int__") } },
        attribute: { HashMap::new() },
        method: { HashMap::new() },
    };
    Box::new(val)
}