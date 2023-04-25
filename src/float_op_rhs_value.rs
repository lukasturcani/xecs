use crate::python_arrays::{
    float32::Float32, float64::Float64, Int16, Int32, Int64, Int8, ReadableArray, UInt16, UInt32,
    UInt64, UInt8,
};
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

#[derive(FromPyObject)]
pub enum FloatOpRhsValue<'a> {
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

impl<'a> FloatOpRhsValue<'a> {
    pub fn read(&self) -> PyResult<ReadableFloatOpRhsValue> {
        match self {
            FloatOpRhsValue::I64(value) => Ok(ReadableFloatOpRhsValue::I64(*value)),
            FloatOpRhsValue::F64(value) => Ok(ReadableFloatOpRhsValue::F64(*value)),
            FloatOpRhsValue::Float32(array) => array.read().map(ReadableFloatOpRhsValue::Float32),
            FloatOpRhsValue::Float64(array) => array.read().map(ReadableFloatOpRhsValue::Float64),
            FloatOpRhsValue::Int8(array) => array.read().map(ReadableFloatOpRhsValue::Int8),
            FloatOpRhsValue::Int16(array) => array.read().map(ReadableFloatOpRhsValue::Int16),
            FloatOpRhsValue::Int32(array) => array.read().map(ReadableFloatOpRhsValue::Int32),
            FloatOpRhsValue::Int64(array) => array.read().map(ReadableFloatOpRhsValue::Int64),
            FloatOpRhsValue::UInt8(array) => array.read().map(ReadableFloatOpRhsValue::UInt8),
            FloatOpRhsValue::UInt16(array) => array.read().map(ReadableFloatOpRhsValue::UInt16),
            FloatOpRhsValue::UInt32(array) => array.read().map(ReadableFloatOpRhsValue::UInt32),
            FloatOpRhsValue::UInt64(array) => array.read().map(ReadableFloatOpRhsValue::UInt64),
            FloatOpRhsValue::PyArrayF32(array) => {
                Ok(ReadableFloatOpRhsValue::PyArrayF32(array.readonly()))
            }
            FloatOpRhsValue::PyArrayF64(array) => {
                Ok(ReadableFloatOpRhsValue::PyArrayF64(array.readonly()))
            }
            FloatOpRhsValue::PyArrayI8(array) => {
                Ok(ReadableFloatOpRhsValue::PyArrayI8(array.readonly()))
            }
            FloatOpRhsValue::PyArrayI16(array) => {
                Ok(ReadableFloatOpRhsValue::PyArrayI16(array.readonly()))
            }
            FloatOpRhsValue::PyArrayI32(array) => {
                Ok(ReadableFloatOpRhsValue::PyArrayI32(array.readonly()))
            }
            FloatOpRhsValue::PyArrayI64(array) => {
                Ok(ReadableFloatOpRhsValue::PyArrayI64(array.readonly()))
            }
            FloatOpRhsValue::PyArrayU8(array) => {
                Ok(ReadableFloatOpRhsValue::PyArrayU8(array.readonly()))
            }
            FloatOpRhsValue::PyArrayU16(array) => {
                Ok(ReadableFloatOpRhsValue::PyArrayU16(array.readonly()))
            }
            FloatOpRhsValue::PyArrayU32(array) => {
                Ok(ReadableFloatOpRhsValue::PyArrayU32(array.readonly()))
            }
            FloatOpRhsValue::PyArrayU64(array) => {
                Ok(ReadableFloatOpRhsValue::PyArrayU64(array.readonly()))
            }
        }
    }
}

pub enum ReadableFloatOpRhsValue<'lock> {
    I64(i64),
    F64(f64),
    Float32(ReadableArray<'lock, f32>),
    Float64(ReadableArray<'lock, f64>),
    Int8(ReadableArray<'lock, i8>),
    Int16(ReadableArray<'lock, i16>),
    Int32(ReadableArray<'lock, i32>),
    Int64(ReadableArray<'lock, i64>),
    UInt8(ReadableArray<'lock, u8>),
    UInt16(ReadableArray<'lock, u16>),
    UInt32(ReadableArray<'lock, u32>),
    UInt64(ReadableArray<'lock, u64>),
    PyArrayF32(PyReadonlyArray1<'lock, f32>),
    PyArrayF64(PyReadonlyArray1<'lock, f64>),
    PyArrayI8(PyReadonlyArray1<'lock, i8>),
    PyArrayI16(PyReadonlyArray1<'lock, i16>),
    PyArrayI32(PyReadonlyArray1<'lock, i32>),
    PyArrayI64(PyReadonlyArray1<'lock, i64>),
    PyArrayU8(PyReadonlyArray1<'lock, u8>),
    PyArrayU16(PyReadonlyArray1<'lock, u16>),
    PyArrayU32(PyReadonlyArray1<'lock, u32>),
    PyArrayU64(PyReadonlyArray1<'lock, u64>),
}
