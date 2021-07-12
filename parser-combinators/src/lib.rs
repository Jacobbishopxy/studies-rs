//！ Rust parser combinators
//!
//! Blog address: https://jacobbishopxy.github.io/2021-7-11-rust-parser-combinators/

pub mod step1;
pub mod step2;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}
