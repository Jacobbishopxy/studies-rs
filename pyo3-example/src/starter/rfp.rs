//! Calling a Rust function from Python

use std::collections::HashMap;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyfunction]
fn multiply(a: isize, b: isize) -> PyResult<isize> {
    Ok(a * b)
}

#[pyfunction]
fn get_fibonacci(number: isize) -> PyResult<u128> {
    if number == 1 {
        return Ok(1);
    } else if number == 2 {
        return Ok(2);
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _ in 1..number {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    Ok(sum)
}

#[pyfunction]
fn list_sum(a: Vec<isize>) -> PyResult<isize> {
    Ok(a.iter().sum())
}

#[pyfunction]
fn dict_printer(hm: HashMap<String, String>) {
    for (key, value) in hm {
        println!("{} -> {}", key, value);
    }
}

#[pyfunction]
fn word_printer(mut word: String, n: isize, reverse: bool, uppercase: bool) {
    if reverse {
        let mut reversed_word = String::new();
        for c in word.chars().rev() {
            reversed_word.push(c);
        }
        word = reversed_word;
    }
    if uppercase {
        word = word.to_uppercase();
    }
    for _ in 0..n {
        print!("{}", word);
    }
}

#[pyclass]
pub struct RustStruct {
    #[pyo3(get, set)]
    pub data: String,
    #[pyo3(get, set)]
    pub vector: Vec<u8>,
}

#[pymethods]
impl RustStruct {
    #[new]
    fn new(data: String, vector: Vec<u8>) -> Self {
        RustStruct { data, vector }
    }

    pub fn printer(&self) {
        println!("{}", self.data);
        for i in &self.vector {
            println!("{}", i);
        }
    }

    pub fn extend_vector(&mut self, vector: Vec<u8>) {
        println!("{:?}", self.data);
        self.vector.extend(vector);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Human {
    name: String,
    age: u8,
}

#[pyfunction]
fn human_says_hi(human: String) {
    println!("{:?}", human);
    let human: Human = serde_json::from_str(&human).unwrap();

    println!(
        "Now we can work with the struct: \n {:#?}.\n {} is {} year old.",
        human, human.name, human.age
    );
}

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(get_fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(list_sum, m)?)?;
    m.add_function(wrap_pyfunction!(dict_printer, m)?)?;
    m.add_function(wrap_pyfunction!(word_printer, m)?)?;
    m.add_class::<RustStruct>()?;

    m.add_function(wrap_pyfunction!(human_says_hi, m)?)?;
    Ok(())
}
