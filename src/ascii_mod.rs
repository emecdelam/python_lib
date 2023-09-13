use pyo3::prelude::*;

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
