use crate::arrays::python_float_array::{Float32, Float64};
use crate::arrays::python_int_array::{Int16, Int32, Int64, Int8, UInt16, UInt32, UInt64, UInt8};
use numpy::PyArray1;
use pyo3::prelude::*;

#[derive(FromPyObject, Debug)]
pub enum FloatRhs<'a> {
    I64(i64),
    F64(f64),
    Float32(PyRef<'a, Float32>),
    Float64(PyRef<'a, Float64>),
    Int8(PyRef<'a, Int8>),
    Int16(PyRef<'a, Int16>),
    Int32(PyRef<'a, Int32>),
    Int64(PyRef<'a, Int64>),
    UInt8(PyRef<'a, UInt8>),
    UInt16(PyRef<'a, UInt16>),
    UInt32(PyRef<'a, UInt32>),
    UInt64(PyRef<'a, UInt64>),
    PyArrayF32(&'a PyArray1<f32>),
    PyArrayF64(&'a PyArray1<f64>),
    PyArrayI8(&'a PyArray1<i8>),
    PyArrayI16(&'a PyArray1<i16>),
    PyArrayI32(&'a PyArray1<i32>),
    PyArrayI64(&'a PyArray1<i64>),
    PyArrayU8(&'a PyArray1<u8>),
    PyArrayU16(&'a PyArray1<u16>),
    PyArrayU32(&'a PyArray1<u32>),
    PyArrayU64(&'a PyArray1<u64>),
}
