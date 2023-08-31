use pyo3::prelude::*;

mod app;
mod array_view_indices;
mod combinations;
mod component_id;
mod component_pool;
mod entity_id;
mod error_handlers;
mod float32;
mod getitem_key;
mod index;
mod map;
mod query;
mod query_id;
mod set;
mod time;

/// A Python module implemented in Rust.
#[pymodule]
fn xecs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<float32::Float32>()?;
    m.add_class::<app::RustApp>()?;
    m.add_class::<array_view_indices::ArrayViewIndices>()?;
    m.add_class::<array_view_indices::MultipleArrayViewIndices>()?;
    m.add_class::<time::Duration>()?;
    m.add_class::<time::Instant>()?;
    m.add_class::<time::Time>()?;
    m.add_function(wrap_pyfunction!(combinations::combinations_2, m)?)?;
    m.add_function(wrap_pyfunction!(combinations::product_2, m)?)?;
    Ok(())
}
