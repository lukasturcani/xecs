use numpy::PyArray1;
use pyo3::prelude::*;

fn assign_value_at_indices<T: Copy>(array: &PyArray1<T>, indices: Vec<usize>, value: T) {
    let start = array.data();
    unsafe {
        for index in indices {
            let item_pointer = start.add(index);
            *item_pointer = value;
        }
    }
}

#[pyfunction]
fn assign_value_at_indices_bool(array: &PyArray1<bool>, indices: Vec<usize>, value: bool) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_i8(array: &PyArray1<i8>, indices: Vec<usize>, value: i8) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_i16(array: &PyArray1<i16>, indices: Vec<usize>, value: i16) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_i32(array: &PyArray1<i32>, indices: Vec<usize>, value: i32) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_i64(array: &PyArray1<i64>, indices: Vec<usize>, value: i64) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_u8(array: &PyArray1<u8>, indices: Vec<usize>, value: u8) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_u16(array: &PyArray1<u16>, indices: Vec<usize>, value: u16) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_u32(array: &PyArray1<u32>, indices: Vec<usize>, value: u32) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_u64(array: &PyArray1<u64>, indices: Vec<usize>, value: u64) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_f32(array: &PyArray1<f32>, indices: Vec<usize>, value: f32) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_f64(array: &PyArray1<f64>, indices: Vec<usize>, value: f64) {
    assign_value_at_indices(array, indices, value)
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
    m.add_function(wrap_pyfunction!(assign_value_at_indices_bool, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_i8, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_i16, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_i32, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_i64, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_u8, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_u16, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_u32, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_u64, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_f32, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_f64, m)?)?;
    Ok(())
}
