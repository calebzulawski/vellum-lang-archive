mod ast;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(grammar);

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<Value>),
    Table(HashMap<String, Value>),
}
