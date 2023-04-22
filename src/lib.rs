use pyo3::prelude::*;

mod app;
mod array_view_indices;
mod component_id;
mod component_pool;
mod entity_id;
mod error_handlers;
mod float_op_rhs_value;
mod getitem_key;
mod index;
mod map;
mod python_arrays;
mod query;
mod query_id;
mod readable_array;
mod set;

/// A Python module implemented in Rust.
#[pymodule]
fn ecstasy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<python_arrays::Float32>()?;
    m.add_class::<python_arrays::Float64>()?;
    m.add_class::<python_arrays::Int8>()?;
    m.add_class::<python_arrays::Int16>()?;
    m.add_class::<python_arrays::Int32>()?;
    m.add_class::<python_arrays::Int64>()?;
    m.add_class::<python_arrays::UInt8>()?;
    m.add_class::<python_arrays::UInt16>()?;
    m.add_class::<python_arrays::UInt32>()?;
    m.add_class::<python_arrays::UInt64>()?;
    m.add_class::<app::RustApp>()?;
    m.add_class::<array_view_indices::ArrayViewIndices>()?;
    m.add_class::<array_view_indices::MultipleArrayViewIndices>()?;
    Ok(())
}
