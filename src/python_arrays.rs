use crate::array_view_indices::ArrayViewIndices;
use crate::error_handlers::{cannot_read, cannot_write};
use numpy::PyArray1;
use pyo3::prelude::*;
use std::sync::{Arc, RwLock};

struct Array<T> {
    array: Arc<RwLock<Vec<T>>>,
    indices: ArrayViewIndices,
}

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

#[derive(FromPyObject)]
pub enum IntOpRhsValue<'a> {
    I64(i64),
    Int8(PyRef<'a, Int8>),
    Int16(PyRef<'a, Int16>),
    Int32(PyRef<'a, Int32>),
    Int64(PyRef<'a, Int64>),
    UInt8(PyRef<'a, UInt8>),
    UInt16(PyRef<'a, UInt16>),
    UInt32(PyRef<'a, UInt32>),
    UInt64(PyRef<'a, UInt64>),
    PyArrayI8(&'a PyArray1<i8>),
    PyArrayI16(&'a PyArray1<i16>),
    PyArrayI32(&'a PyArray1<i32>),
    PyArrayI64(&'a PyArray1<i64>),
    PyArrayU8(&'a PyArray1<u8>),
    PyArrayU16(&'a PyArray1<u16>),
    PyArrayU32(&'a PyArray1<u32>),
    PyArrayU64(&'a PyArray1<u64>),
}

impl<T> Array<T>
where
    T: numpy::Element,
{
    pub fn p_from_numpy(array: &PyArray1<T>) -> PyResult<Self> {
        Ok(Self {
            array: Arc::new(RwLock::new(array.to_vec()?)),
            indices: ArrayViewIndices(Arc::new(RwLock::new(
                ((0 as u32)..(array.len() as u32)).collect(),
            ))),
        })
    }

    pub fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<T>>> {
        let vec = self.array.read().map_err(cannot_read)?;
        Ok(PyArray1::from_vec(py, vec.clone()).into_py(py))
    }

    pub fn p_with_indices(indices: &ArrayViewIndices, default: T) -> PyResult<Self> {
        Ok(Self {
            array: Arc::new(RwLock::new(vec![
                default;
                indices
                    .0
                    .read()
                    .map_err(cannot_read)?
                    .capacity()
            ])),
            indices: ArrayViewIndices(Arc::clone(&indices.0)),
        })
    }

    pub fn __getitem__(&self, key: &ArrayViewIndices) -> Self {
        Self {
            array: Arc::clone(&self.array),
            indices: ArrayViewIndices(Arc::clone(&key.0)),
        }
    }
    pub fn __len__(&self) -> PyResult<usize> {
        self.indices.__len__()
    }
}

macro_rules! value_op {
    ($self_array:ident, $self_indices:ident, $other_value:ident, $type:ty, $op:tt) => {
        for &self_index in $self_indices.iter() {
            let self_value = unsafe { $self_array.get_unchecked_mut(self_index as usize) };
            *self_value $op $other_value as $type;
        }
    };
}

macro_rules! array_op {
    ($self_array:ident, $self_indices:ident, $other:ident, $type:ty, $op:tt) => {
        let other_array = $other.0.array.read().map_err(cannot_write)?;
        let other_indices = $other.0.indices.0.read().map_err(cannot_read)?;
        for (&self_index, &other_index) in $self_indices.iter().zip(other_indices.iter()) {
            let self_value = unsafe { $self_array.get_unchecked_mut(self_index as usize) };
            let other_value = unsafe { other_array.get_unchecked(other_index as usize) };
            *self_value $op *other_value as $type;
        }
    };
}

macro_rules! py_array_op {
    ($self_array:ident, $self_indices:ident, $other:ident, $type:ty, $op:tt) => {
        for (&self_index, &other_value) in $self_indices.iter().zip($other.readonly().as_array().iter()) {
            let self_value = unsafe { $self_array.get_unchecked_mut(self_index as usize) };
            *self_value $op other_value as $type;
        }
    };
}

macro_rules! float_binary_op {
    ($self_array:expr, $self_indices:expr, $other:ident, $type:ty, $op:tt) => {
        let mut self_array = $self_array.write().map_err(cannot_write)?;
        let self_indices = $self_indices.0.read().map_err(cannot_read)?;
        match $other {
            FloatOpRhsValue::I64(other_value) => {
                value_op!(self_array, self_indices, other_value, $type, $op);
            }
            FloatOpRhsValue::F64(other_value) => {
                value_op!(self_array, self_indices, other_value, $type, $op);
            }
            FloatOpRhsValue::Float32(other_array) => {
                array_op!(self_array, self_indices, other_array, $type, $op);
            }
            FloatOpRhsValue::Float64(other_array) => {
                array_op!(self_array, self_indices, other_array, $type, $op);
            }
            FloatOpRhsValue::Int8(other_array) => {
                array_op!(self_array, self_indices, other_array, $type, $op);
            }
            FloatOpRhsValue::Int16(other_array) => {
                array_op!(self_array, self_indices, other_array, $type, $op);
            }
            FloatOpRhsValue::Int32(other_array) => {
                array_op!(self_array, self_indices, other_array, $type, $op);
            }
            FloatOpRhsValue::Int64(other_array) => {
                array_op!(self_array, self_indices, other_array, $type, $op);
            }
            FloatOpRhsValue::UInt8(other_array) => {
                array_op!(self_array, self_indices, other_array, $type, $op);
            }
            FloatOpRhsValue::UInt16(other_array) => {
                array_op!(self_array, self_indices, other_array, $type, $op);
            }
            FloatOpRhsValue::UInt32(other_array) => {
                array_op!(self_array, self_indices, other_array, $type, $op);
            }
            FloatOpRhsValue::UInt64(other_array) => {
                array_op!(self_array, self_indices, other_array, $type, $op);
            }
            FloatOpRhsValue::PyArrayF32(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            FloatOpRhsValue::PyArrayF64(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            FloatOpRhsValue::PyArrayI8(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            FloatOpRhsValue::PyArrayI16(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            FloatOpRhsValue::PyArrayI32(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            FloatOpRhsValue::PyArrayI64(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            FloatOpRhsValue::PyArrayU8(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            FloatOpRhsValue::PyArrayU16(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            FloatOpRhsValue::PyArrayU32(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            FloatOpRhsValue::PyArrayU64(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
        }
    };
}

macro_rules! float_array {
    (impl Array<$type:ty>) => {
        impl Array<$type> {
            pub fn __setitem__(&mut self, key: &ArrayViewIndices, value: FloatOpRhsValue) -> PyResult<()> {
                float_binary_op!(self.array, key, value, $type, =);
                Ok(())
            }

            pub fn __iadd__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                float_binary_op!(self.array, self.indices, other, $type, +=);
                Ok(())
            }

            pub fn __isub__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                float_binary_op!(self.array, self.indices, other, $type, -=);
                Ok(())
            }

            pub fn __imul__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                float_binary_op!(self.array, self.indices, other, $type, *=);
                Ok(())
            }
            pub fn __itruediv__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                float_binary_op!(self.array, self.indices, other, $type, /=);
                Ok(())
            }
            pub fn __imod__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                float_binary_op!(self.array, self.indices, other, $type, %=);
                Ok(())
            }

        }
    };
}

macro_rules! int_binary_op {
    ($self_array:expr, $self_indices:expr, $other:ident, $type:ty, $op:tt) => {
        let mut self_array = $self_array.write().map_err(cannot_write)?;
        let self_indices = $self_indices.0.read().map_err(cannot_read)?;
        match $other {
            IntOpRhsValue::I64(other_value) => {
                value_op!(self_array, self_indices, other_value, $type, $op);
            }
            IntOpRhsValue::Int8(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntOpRhsValue::Int16(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntOpRhsValue::Int32(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntOpRhsValue::Int64(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntOpRhsValue::UInt8(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntOpRhsValue::UInt16(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntOpRhsValue::UInt32(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntOpRhsValue::UInt64(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntOpRhsValue::PyArrayI8(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            IntOpRhsValue::PyArrayI16(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            IntOpRhsValue::PyArrayI32(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            IntOpRhsValue::PyArrayI64(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            IntOpRhsValue::PyArrayU8(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            IntOpRhsValue::PyArrayU16(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            IntOpRhsValue::PyArrayU32(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
            IntOpRhsValue::PyArrayU64(py_array) => {
                py_array_op!(self_array, self_indices, py_array, $type, $op);
            }
        }
    };
}

macro_rules! int_array {
    (impl Array<$type:ty>) => {
        impl Array<$type> {
            pub fn __setitem__(&mut self, key: &ArrayViewIndices, value: IntOpRhsValue) -> PyResult<()> {
                int_binary_op!(self.array, key, value, $type, =);
                Ok(())
            }
            pub fn __iadd__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                int_binary_op!(self.array, self.indices, other, $type, +=);
                Ok(())
            }
            pub fn __isub__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                int_binary_op!(self.array, self.indices, other, $type, -=);
                Ok(())
            }
            pub fn __imul__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                int_binary_op!(self.array, self.indices, other, $type, *=);
                Ok(())
            }
            pub fn __itrudediv__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                int_binary_op!(self.array, self.indices, other, $type, /=);
                Ok(())
            }
            pub fn __imod__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                int_binary_op!(self.array, self.indices, other, $type, %=);
                Ok(())
            }
        }
    };
}

float_array! { impl Array<f32> }
float_array! { impl Array<f64> }
int_array! { impl Array<i8> }
int_array! { impl Array<i16> }
int_array! { impl Array<i32> }
int_array! { impl Array<i64> }
int_array! { impl Array<u8> }
int_array! { impl Array<u16> }
int_array! { impl Array<u32> }
int_array! { impl Array<u64> }

macro_rules! python_float_array {
    (pub struct $name:ident($type:ty)) => {
        #[pyclass]
        pub struct $name(Array<$type>);
        #[pymethods]
        impl $name {
            #[staticmethod]
            pub fn p_with_indices(indices: &ArrayViewIndices) -> PyResult<Self> {
                Array::p_with_indices(indices, 0.0).map(Self)
            }
            #[staticmethod]
            pub fn p_from_numpy(array: &PyArray1<$type>) -> PyResult<Self> {
                Array::p_from_numpy(array).map(Self)
            }
            pub fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<$type>>> {
                self.0.numpy(py)
            }
            pub fn __len__(&self) -> PyResult<usize> {
                self.0.__len__()
            }
            pub fn __getitem__(&self, key: &ArrayViewIndices) -> Self {
                Self(self.0.__getitem__(key))
            }
            pub fn __setitem__(
                &mut self,
                key: &ArrayViewIndices,
                value: FloatOpRhsValue,
            ) -> PyResult<()> {
                self.0.__setitem__(key, value)
            }
            pub fn __iadd__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                self.0.__iadd__(other)
            }
        }
    };
}

macro_rules! python_int_array {
    (pub struct $name:ident($type:ty)) => {
        #[pyclass]
        pub struct $name(Array<$type>);
        #[pymethods]
        impl $name {
            #[staticmethod]
            pub fn p_with_indices(indices: &ArrayViewIndices) -> PyResult<Self> {
                Array::p_with_indices(indices, 0).map(Self)
            }
            #[staticmethod]
            pub fn p_from_numpy(array: &PyArray1<$type>) -> PyResult<Self> {
                Array::p_from_numpy(array).map(Self)
            }
            pub fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<$type>>> {
                self.0.numpy(py)
            }
            pub fn __getitem__(&self, key: &ArrayViewIndices) -> Self {
                Self(self.0.__getitem__(key))
            }
            pub fn __len__(&self) -> PyResult<usize> {
                self.0.__len__()
            }
            pub fn __iadd__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                self.0.__iadd__(other)
            }
        }
    };
}

python_float_array! {
    pub struct Float32(f32)
}

python_float_array! {
    pub struct Float64(f64)
}

python_int_array! {
    pub struct Int8(i8)
}

python_int_array! {
    pub struct Int16(i16)
}

python_int_array! {
    pub struct Int32(i32)
}

python_int_array! {
    pub struct Int64(i64)
}

python_int_array! {
    pub struct UInt8(u8)
}

python_int_array! {
    pub struct UInt16(u16)
}

python_int_array! {
    pub struct UInt32(u32)
}

python_int_array! {
    pub struct UInt64(u64)
}
