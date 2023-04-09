use numpy::PyArray1;
use pyo3::prelude::*;

#[pyfunction]
fn assign_value_at_indices(array: &PyArray1<f64>, indices: Vec<usize>, value: f64) {
    let start = array.data();
    unsafe {
        for index in indices {
            let item_pointer = start.add(index);
            *item_pointer = value;
        }
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn necs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices, m)?)?;
    Ok(())
}
