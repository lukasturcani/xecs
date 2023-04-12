use pyo3::prelude::*;

mod app;
mod component_id;
mod component_pool;
mod entity_id;
mod index;
mod map;
mod python_arrays;
mod query;
mod query_id;
mod set;

/// A Python module implemented in Rust.
#[pymodule]
fn ecstasy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<python_arrays::float64::Float64>()?;
    m.add_class::<app::RustApp>()?;
    Ok(())
}
