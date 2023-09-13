mod ascii_mod;

use pyo3::prelude::*;
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
/// Converts a list of numbers to an ASCII string.
#[pyfunction]
pub fn numbers_to_ascii(numbers: Vec<u8>) -> String {
    let mut ascii_string = String::new();

    for &num in numbers.iter() {
        let ascii_char = num as char;
        ascii_string.push(ascii_char);
    }
    return ascii_string
}
/// A Python module implemented in Rust.
#[pymodule]
fn emec_dev_tools(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(ascii_mod::numbers_to_ascii,m)?)?;
    Ok(())
}