use std::ops::AddAssign;

use num::cast::AsPrimitive;
use numpy::PyArray1;
use pyo3::prelude::*;

use crate::{array_view_indices::ArrayViewIndices, error_handlers::cannot_read};

#[derive(FromPyObject, Debug)]
pub enum FloatRhs2<'a> {
    F32(&'a PyArray1<f32>),
    F64(&'a PyArray1<f64>),
    I8(&'a PyArray1<i8>),
    I16(&'a PyArray1<i16>),
    I32(&'a PyArray1<i32>),
    I64(&'a PyArray1<i64>),
    U8(&'a PyArray1<u8>),
    U16(&'a PyArray1<u16>),
    U32(&'a PyArray1<u32>),
    U64(&'a PyArray1<u64>),
}

fn iadd_impl<T, U>(
    array: &PyArray1<T>,
    indices: &ArrayViewIndices,
    other_array: &PyArray1<U>,
    other_indices: &ArrayViewIndices,
) -> PyResult<()>
where
    T: numpy::Element + AddAssign<T> + Copy + 'static,
    U: numpy::Element + AsPrimitive<T>,
{
    let mut array = array.readwrite();
    let mut array = array.as_array_mut();
    let start = array.as_mut_ptr();
    let indices = indices.0.read().map_err(cannot_read)?;
    let other_array = other_array.readonly();
    let other_array = other_array.as_array();
    let other_start = other_array.as_ptr();
    let other_indices = other_indices.0.read().map_err(cannot_read)?;
    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
        let value = unsafe { start.add(index as usize) };
        let other_value = unsafe { *other_start.add(other_index as usize) }.as_();
        unsafe { *value += other_value };
    }
    Ok(())
}

#[pyfunction]
pub fn iadd_float32(
    array: &PyArray1<f32>,
    indices: &ArrayViewIndices,
    other_array: FloatRhs2,
    other_indices: &ArrayViewIndices,
) -> PyResult<()> {
    match other_array {
        FloatRhs2::F32(rhs) => {
            iadd_impl(array, indices, rhs, other_indices)?;
        }
        FloatRhs2::F64(rhs) => {
            iadd_impl(array, indices, rhs, other_indices)?;
        }
        FloatRhs2::I8(rhs) => {
            iadd_impl(array, indices, rhs, other_indices)?;
        }
        FloatRhs2::I16(rhs) => {
            iadd_impl(array, indices, rhs, other_indices)?;
        }
        FloatRhs2::I32(rhs) => {
            iadd_impl(array, indices, rhs, other_indices)?;
        }
        FloatRhs2::I64(rhs) => {
            iadd_impl(array, indices, rhs, other_indices)?;
        }
        FloatRhs2::U8(rhs) => {
            iadd_impl(array, indices, rhs, other_indices)?;
        }
        FloatRhs2::U16(rhs) => {
            iadd_impl(array, indices, rhs, other_indices)?;
        }
        FloatRhs2::U32(rhs) => {
            iadd_impl(array, indices, rhs, other_indices)?;
        }
        FloatRhs2::U64(rhs) => {
            iadd_impl(array, indices, rhs, other_indices)?;
        }
    }
    Ok(())
}
