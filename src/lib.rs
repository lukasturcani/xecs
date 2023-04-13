use pyo3::prelude::*;

mod app;
mod array_view_indices;
mod component_id;
mod component_pool;
mod entity_id;
mod error_handlers;
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
    m.add_class::<array_view_indices::ArrayViewIndices>()?;
    m.add_class::<array_view_indices::MultipleArrayViewIndices>()?;
    Ok(())
}
