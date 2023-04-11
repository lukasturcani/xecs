use pyo3::prelude::*;

mod array;
mod array_view;
mod component_pool;
mod python_arrays;
mod query;

/// A Python module implemented in Rust.
#[pymodule]
fn ecstasy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<python_arrays::Float64Array>()?;
    m.add_class::<python_arrays::Float64>()?;
    m.add_class::<query::Query>()?;
    Ok(())
}
