use crate::python_arrays::{
    Float32, Float64, Int16, Int32, Int64, Int8, UInt16, UInt32, UInt64, UInt8,
};
use crate::readable_array;
use crate::readable_array::ReadableArray;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

#[derive(FromPyObject)]
pub enum FloatOpRhsValue<'a> {
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
    fn read(&self) -> PyResult<ReadFloatOpRhsValue<'_>> {
        match self {
            FloatOpRhsValue::F64(value) => Ok(ReadFloatOpRhsValue::F64(*value)),
            FloatOpRhsValue::Float32(array) => array.read().map(ReadFloatOpRhsValue::Float32),
            FloatOpRhsValue::Float64(array) => array.read().map(ReadFloatOpRhsValue::Float64),
            FloatOpRhsValue::Int8(array) => array.read().map(ReadFloatOpRhsValue::Int8),
            FloatOpRhsValue::Int16(array) => array.read().map(ReadFloatOpRhsValue::Int16),
            FloatOpRhsValue::Int32(array) => array.read().map(ReadFloatOpRhsValue::Int32),
            FloatOpRhsValue::Int64(array) => array.read().map(ReadFloatOpRhsValue::Int64),
            FloatOpRhsValue::UInt8(array) => array.read().map(ReadFloatOpRhsValue::UInt8),
            FloatOpRhsValue::UInt16(array) => array.read().map(ReadFloatOpRhsValue::UInt16),
            FloatOpRhsValue::UInt32(array) => array.read().map(ReadFloatOpRhsValue::UInt32),
            FloatOpRhsValue::UInt64(array) => array.read().map(ReadFloatOpRhsValue::UInt64),
            FloatOpRhsValue::PyArrayF32(array) => {
                Ok(ReadFloatOpRhsValue::PyArrayF32(array.readonly()))
            }
            FloatOpRhsValue::PyArrayF64(array) => {
                Ok(ReadFloatOpRhsValue::PyArrayF64(array.readonly()))
            }
            FloatOpRhsValue::PyArrayI8(array) => {
                Ok(ReadFloatOpRhsValue::PyArrayI8(array.readonly()))
            }
            FloatOpRhsValue::PyArrayI16(array) => {
                Ok(ReadFloatOpRhsValue::PyArrayI16(array.readonly()))
            }
            FloatOpRhsValue::PyArrayI32(array) => {
                Ok(ReadFloatOpRhsValue::PyArrayI32(array.readonly()))
            }
            FloatOpRhsValue::PyArrayI64(array) => {
                Ok(ReadFloatOpRhsValue::PyArrayI64(array.readonly()))
            }
            FloatOpRhsValue::PyArrayU8(array) => {
                Ok(ReadFloatOpRhsValue::PyArrayU8(array.readonly()))
            }
            FloatOpRhsValue::PyArrayU16(array) => {
                Ok(ReadFloatOpRhsValue::PyArrayU16(array.readonly()))
            }
            FloatOpRhsValue::PyArrayU32(array) => {
                Ok(ReadFloatOpRhsValue::PyArrayU32(array.readonly()))
            }
            FloatOpRhsValue::PyArrayU64(array) => {
                Ok(ReadFloatOpRhsValue::PyArrayU64(array.readonly()))
            }
        }
    }
}

enum ReadFloatOpRhsValue<'lock> {
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

impl<'lock> ReadFloatOpRhsValue<'lock> {
    fn iter_f64(&self) -> IterF64<'_> {
        match self {
            ReadFloatOpRhsValue::F64(value) => IterF64::Value(value),
            ReadFloatOpRhsValue::Float32(array) => IterF64::Array(array.iter()),
            _ => panic!(""),
        }
    }
}

enum IterF64<'a> {
    Value(&'a f64),
    Array(readable_array::Iter<'a, f64>),
}

impl<'a> Iterator for IterF64<'a> {
    type Item = &'a f64;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterF64::Value(value) => Some(value),
            IterF64::Array(iter) => iter.next(),
        }
    }
}
