//！ Rust parser combinators
//!
//! Blog address: https://jacobbishopxy.github.io/2021-7-11-rust-parser-combinators/

pub mod step1;
pub mod step2;
pub mod step3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Element {
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Element>,
}
