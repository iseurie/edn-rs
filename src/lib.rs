extern crate ordered_float;

use std::collections::{LinkedList, BTreeMap, BTreeSet};

use ordered_float::OrderedFloat;

pub mod parser;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Nil,
    Boolean(bool),
    String(String),
    Char(char),
    Symbol(String),
    Keyword(String),
    Integer(i64),
    Float(OrderedFloat<f64>),
    List(LinkedList<Value>),
    Vector(Vec<Value>),
    Map(BTreeMap<Value, Value>),
    Set(BTreeSet<Value>),
    Tagged(String, Box<Value>),
}
