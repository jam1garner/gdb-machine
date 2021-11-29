//! Types representing the parsed output from the GDB machine interface
//!
//! Output can be parsed from a string using [`Output::parse`]
use std::collections::HashMap;

mod parser;

#[cfg(test)]
mod parser_tests;

#[derive(Debug, Clone)]
pub enum StreamRecord {
    Console(String),
    Target(String),
    Log(String),
}

#[derive(Debug, Clone)]
pub enum Value {
    Const(String),
    Tuple(HashMap<String, Value>),
    List(Vec<Value>),
    ListMap(HashMap<String, Value>),
}

#[derive(Debug, Clone)]
pub enum ResultClass {
    Done,
    Running,
    Connected,
    Error,
    Exit,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum AsyncClass {
    Stopped,
}

#[derive(Debug, Clone)]
pub enum OutOfBandRecord {
    Async(AsyncRecord),
    Stream(StreamRecord),
}

#[derive(Debug, Clone)]
pub struct AsyncRecord {
    pub kind: AsyncRecordKind,
    pub token: Option<usize>,
    pub output: AsyncOutput,
}

#[derive(Debug, Clone)]
pub enum AsyncRecordKind {
    Exec,
    Status,
    Notify,
}

#[derive(Debug, Clone)]
pub struct AsyncOutput {
    pub class: AsyncClass,
    pub results: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct Output {
    pub out_of_band: Vec<OutOfBandRecord>,
    pub result: Option<ResultRecord>,
}

#[derive(Debug, Clone)]
pub struct ResultRecord {
    pub result_class: ResultClass,
    pub results: HashMap<String, Value>,
}
