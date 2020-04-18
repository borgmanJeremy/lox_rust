#![allow(dead_code)]
pub mod scanner;
pub use crate::scanner::{Token,TokenType,Literal};

pub mod parser;