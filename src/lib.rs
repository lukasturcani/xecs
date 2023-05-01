use pyo3::prelude::*;

mod app;
mod array_view_indices;
mod arrays;
mod component_id;
mod component_pool;
mod entity_id;
mod error_handlers;
mod getitem_key;
mod index;
mod map;
mod query;
mod query_id;
mod set;

/// A Python module implemented in Rust.
#[pymodule]
fn ecstasy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(arrays::imath::iadd_float32, m)?)?;
    m.add_class::<arrays::Float32>()?;
    m.add_class::<arrays::Float64>()?;
    m.add_class::<arrays::Int8>()?;
    m.add_class::<arrays::Int16>()?;
    m.add_class::<arrays::Int32>()?;
    m.add_class::<arrays::Int64>()?;
    m.add_class::<arrays::UInt8>()?;
    m.add_class::<arrays::UInt16>()?;
    m.add_class::<arrays::UInt32>()?;
    m.add_class::<arrays::UInt64>()?;
    m.add_class::<app::RustApp>()?;
    m.add_class::<array_view_indices::ArrayViewIndices>()?;
    m.add_class::<array_view_indices::MultipleArrayViewIndices>()?;
    Ok(())
}
