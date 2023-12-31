mod ascii_mod;
mod terminal;

use pyo3::prelude::*;
use ascii_mod::*;
use terminal::*;
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
/// A Python module implemented in Rust.
#[pymodule]
fn emec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(numbers_ascii,m)?)?;
    m.add_function(wrap_pyfunction!(terminal_from_file,m)?)?;
    Ok(())
}