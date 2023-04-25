use crate::array_view_indices::ArrayViewIndices;
use crate::error_handlers::{bad_index, cannot_read, cannot_write};
use crate::float_op_rhs_value::{FloatOpRhsValue, ReadableFloatOpRhsValue};
use crate::getitem_key::GetItemKey;
use crate::index::Index;
use numpy::PyArray1;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct Array<T> {
    array: Arc<RwLock<Vec<T>>>,
    indices: ArrayViewIndices,
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
    pub fn p_new_view_with_indices(&self, indices: &ArrayViewIndices) -> Self {
        Self {
            array: Arc::clone(&self.array),
            indices: ArrayViewIndices(Arc::clone(&indices.0)),
        }
    }
    pub fn write(&mut self) -> PyResult<WriteableArray<T>> {
        Ok(WriteableArray {
            vec: self.array.write().map_err(cannot_write)?,
            indices: self.indices.0.read().map_err(cannot_read)?,
        })
    }
    pub fn read(&self) -> PyResult<ReadableArray<T>> {
        Ok(ReadableArray {
            vec: self.array.read().map_err(cannot_read)?,
            indices: self.indices.0.read().map_err(cannot_read)?,
        })
    }
    pub fn __getitem__(&self, key: GetItemKey) -> PyResult<Self> {
        Ok(Self {
            array: Arc::clone(&self.array),
            indices: self.indices.__getitem__(key)?,
        })
    }
    pub fn __len__(&self) -> PyResult<usize> {
        self.indices.__len__()
    }
}

pub struct WriteableArray<'lock, T> {
    pub vec: RwLockWriteGuard<'lock, Vec<T>>,
    pub indices: RwLockReadGuard<'lock, Vec<Index>>,
}

pub struct ReadableArray<'lock, T> {
    vec: RwLockReadGuard<'lock, Vec<T>>,
    indices: RwLockReadGuard<'lock, Vec<Index>>,
}

macro_rules! value_zip_mut {
    ($self_array:expr, $self_indices:expr, $other_value:expr, $($fn:tt)*) => {
        let f = $($fn)*;
        for self_index in $self_indices.iter() {
            let self_value = unsafe { $self_array.get_unchecked_mut(*self_index as usize) };
            f(self_index, self_value, &$other_value)
        }
    }
}

macro_rules! array_zip_mut {
    ($self_array:expr, $self_indices:expr, $other:expr, $type:ty, $($fn:tt)*) => {
        let f = $($fn)*;
        let other_array = $other.0.array.read().map_err(cannot_write)?;
        let other_indices = $other.0.indices.0.read().map_err(cannot_read)?;
        for (self_index, &other_index) in $self_indices.iter().zip(other_indices.iter()) {
            let self_value = unsafe { $self_array.get_unchecked_mut(*self_index as usize) };
            let other_value = unsafe { other_array.get_unchecked(other_index as usize) };
            f(self_index, self_value, &(*other_value as $type))
        }
    };
}

macro_rules! py_array_zip_mut {
    ($self_array:expr, $self_indices:expr, $other:expr, $type:ty, $($fn:tt)*) => {
        let f = $($fn)*;
        for (self_index, &other_value) in $self_indices
            .iter()
            .zip($other.readonly().as_array().iter())
        {
            let self_value = unsafe { $self_array.get_unchecked_mut(*self_index as usize) };
            f(self_index, self_value, &(other_value as $type))
        }
    };
}

macro_rules! float_rhs_zip_mut {
    ($self:expr, $other:expr, $type: ty, $($fn:tt)*) => {
        let mut self_array = $self.array.write().map_err(cannot_write)?;
        let self_indices = $self.indices.0.read().map_err(cannot_read)?;
        match $other {
            FloatOpRhsValue::I64(other_value) => {
                value_zip_mut!(self_array, self_indices, other_value as $type, $($fn)*);
            }
            FloatOpRhsValue::F64(other_value) => {
                value_zip_mut!(self_array, self_indices, other_value as $type, $($fn)*);
            }
            FloatOpRhsValue::Float32(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            FloatOpRhsValue::Float64(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            FloatOpRhsValue::Int8(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            FloatOpRhsValue::Int16(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            FloatOpRhsValue::Int32(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            FloatOpRhsValue::Int64(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            FloatOpRhsValue::UInt8(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            FloatOpRhsValue::UInt16(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            FloatOpRhsValue::UInt32(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            FloatOpRhsValue::UInt64(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayF32(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayF64(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayI8(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayI16(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayI32(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayI64(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayU8(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayU16(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayU32(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayU64(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
        }
    };
}

macro_rules! int_rhs_zip_mut {
    ($self:expr, $other:expr, $type: ty, $($fn:tt)*) => {
        let mut self_array = $self.array.write().map_err(cannot_write)?;
        let self_indices = $self.indices.0.read().map_err(cannot_read)?;
        match $other {
            IntOpRhsValue::I64(other_value) => {
                value_zip_mut!(self_array, self_indices, other_value as $type, $($fn)*);
            }
            IntOpRhsValue::Int8(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            IntOpRhsValue::Int16(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            IntOpRhsValue::Int32(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            IntOpRhsValue::Int64(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            IntOpRhsValue::UInt8(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            IntOpRhsValue::UInt16(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            IntOpRhsValue::UInt32(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            IntOpRhsValue::UInt64(other_array) => {
                array_zip_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            IntOpRhsValue::PyArrayI8(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            IntOpRhsValue::PyArrayI16(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            IntOpRhsValue::PyArrayI32(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            IntOpRhsValue::PyArrayI64(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            IntOpRhsValue::PyArrayU8(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            IntOpRhsValue::PyArrayU16(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            IntOpRhsValue::PyArrayU32(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            IntOpRhsValue::PyArrayU64(py_array) => {
                py_array_zip_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
        }
    };
}

macro_rules! value_zip {
    ($self_array:expr, $self_indices:expr, $other_value:expr, $self_type:ty, $($fn:tt)*) => {
        let mut f = $($fn)*;
        for self_index in $self_indices.iter() {
            let self_value = unsafe { $self_array.get_unchecked(*self_index as usize) };
            f(self_index, &(*self_value as $self_type), &$other_value)
        }
    }
}

macro_rules! array_zip {
    ($self_array:expr, $self_indices:expr, $other:expr, $self_type:ty, $other_type:ty, $($fn:tt)*) => {
        let mut f = $($fn)*;
        let other_array = $other.0.array.read().map_err(cannot_write)?;
        let other_indices = $other.0.indices.0.read().map_err(cannot_read)?;
        for (self_index, &other_index) in $self_indices.iter().zip(other_indices.iter()) {
            let self_value = unsafe { $self_array.get_unchecked(*self_index as usize) };
            let other_value = unsafe { other_array.get_unchecked(other_index as usize) };
            f(self_index, &(*self_value as $self_type), &(*other_value as $other_type))
        }
    };
}

macro_rules! py_array_zip {
    ($self_array:expr, $self_indices:expr, $other:expr, $self_type:ty, $other_type:ty, $($fn:tt)*) => {
        let mut f = $($fn)*;
        for (self_index, &other_value) in $self_indices
            .iter()
            .zip($other.readonly().as_array().iter())
        {
            let self_value = unsafe { $self_array.get_unchecked(*self_index as usize) };
            f(self_index, &(*self_value as $self_type), &(other_value as $other_type))
        }
    };
}

macro_rules! zip_match {
    ($self_array:expr, $self_indices:expr, $other:expr, $type:ty, $($fn:tt)*) => {
        match $other {
            FloatOpRhsValue::I64(other_value) => {
                value_zip!($self_array, $self_indices, other_value as $type, $type, $($fn)*);
            }
            FloatOpRhsValue::F64(other_value) => {
                value_zip!($self_array, $self_indices, other_value as $type, $type, $($fn)*);
            }
            FloatOpRhsValue::Float32(other_array) => {
                array_zip!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::Float64(other_array) => {
                array_zip!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::Int8(other_array) => {
                array_zip!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::Int16(other_array) => {
                array_zip!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::Int32(other_array) => {
                array_zip!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::Int64(other_array) => {
                array_zip!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::UInt8(other_array) => {
                array_zip!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::UInt16(other_array) => {
                array_zip!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::UInt32(other_array) => {
                array_zip!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::UInt64(other_array) => {
                array_zip!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayF32(py_array) => {
                py_array_zip!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayF64(py_array) => {
                py_array_zip!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayI8(py_array) => {
                py_array_zip!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayI16(py_array) => {
                py_array_zip!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayI32(py_array) => {
                py_array_zip!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayI64(py_array) => {
                py_array_zip!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayU8(py_array) => {
                py_array_zip!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayU16(py_array) => {
                py_array_zip!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayU32(py_array) => {
                py_array_zip!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatOpRhsValue::PyArrayU64(py_array) => {
                py_array_zip!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
        }
    }
}

macro_rules! int_cmp {
    ($self:expr, $other:expr, $type:ty, $op:tt) => {
        {
            let self_array = $self.array.read().map_err(cannot_read)?;
            let self_indices = $self.indices.0.read().map_err(cannot_read)?;
            let indices = ArrayViewIndices::with_capacity(self_indices.len());
            {
                let mut out_indices = indices.0.write().map_err(cannot_write)?;
                match $other {
                    FloatOpRhsValue::I64(other_value) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        value_zip!(self_array, self_indices, other_value as $type, $type, func);
                    }
                    FloatOpRhsValue::F64(other_value) => {
                        let func_f64 = |index: &u32, a: &f64, b: &f64| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        value_zip!(self_array, self_indices, other_value as f64, f64, func_f64);
                    }
                    FloatOpRhsValue::Float32(other_array) => {
                        let func_f32 = |index: &u32, a: &f32, b: &f32| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        array_zip!(self_array, self_indices, other_array, f32, f32, func_f32);
                    }
                    FloatOpRhsValue::Float64(other_array) => {
                        let func_f64 = |index: &u32, a: &f64, b: &f64| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        array_zip!(self_array, self_indices, other_array, f64, f64, func_f64);
                    }
                    FloatOpRhsValue::Int8(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        array_zip!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatOpRhsValue::Int16(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        array_zip!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatOpRhsValue::Int32(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        array_zip!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatOpRhsValue::Int64(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        array_zip!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatOpRhsValue::UInt8(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        array_zip!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatOpRhsValue::UInt16(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        array_zip!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatOpRhsValue::UInt32(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        array_zip!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatOpRhsValue::UInt64(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        array_zip!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatOpRhsValue::PyArrayF32(py_array) => {
                        let func_f32 = |index: &u32, a: &f32, b: &f32| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        py_array_zip!(self_array, self_indices, py_array, f32, f32, func_f32);
                    }
                    FloatOpRhsValue::PyArrayF64(py_array) => {
                        let func_f64 = |index: &u32, a: &f64, b: &f64| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        py_array_zip!(self_array, self_indices, py_array, f64, f64, func_f64);
                    }
                    FloatOpRhsValue::PyArrayI8(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        py_array_zip!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatOpRhsValue::PyArrayI16(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        py_array_zip!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatOpRhsValue::PyArrayI32(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        py_array_zip!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatOpRhsValue::PyArrayI64(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        py_array_zip!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatOpRhsValue::PyArrayU8(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        py_array_zip!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatOpRhsValue::PyArrayU16(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        py_array_zip!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatOpRhsValue::PyArrayU32(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        py_array_zip!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatOpRhsValue::PyArrayU64(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        py_array_zip!(self_array, self_indices, py_array, $type, $type, func);
                    }
                };
            }
            Ok(indices)
        }
    }
}

macro_rules! float_cmp {
    ($self:expr, $other:expr, $type:ty, $op:tt) => {
        {
            let self_array = $self.array.read().map_err(cannot_read)?;
            let self_indices = $self.indices.0.read().map_err(cannot_read)?;
            let indices = ArrayViewIndices::with_capacity(self_indices.len());
            {
                let mut out_indices = indices.0.write().map_err(cannot_write)?;
                zip_match!(
                    self_array,
                    self_indices,
                    $other,
                    $type,
                    |index: &u32, a: &$type, b: &$type| {
                        if a $op b {
                            out_indices.push(*index);
                        }
                    }
                );
            }
            Ok(indices)
        }

    }

}

macro_rules! float_iop {
    ($self:expr, $other:expr, $type:ty, $op:tt) => {
        {
            float_rhs_zip_mut!($self, $other, $type, |_, a: &mut $type, b: &$type| {
                *a $op b
            });
            Ok(())
        }
    }
}

macro_rules! float_array {
    (impl Array<$type:ty>) => {
        impl Array<$type> {
            // pub fn __setitem__(&mut self, key: GetItemKey, value: FloatOpRhsValue) -> PyResult<()> {
            //     let array = self.array.write().map_err(cannot_write)?;
            //     let indices = self.indices.0.read().map_err(cannot_read)?;
            //     match key {
            //         GetItemKey::Slice(slice) => {
            //             let slice_indices = slice.indices(indices.len() as i64)?;
            //             for (index, new_value) in
            //                 (slice_indices.start..slice_indices.stop).step_by(slice_indices.step as usize).zip(value.iter())
            //             {
            //                 let array_index = unsafe { indices.get_unchecked(index as usize) };
            //                 let value = array.get_unchecked_mut(*array_index as usize);
            //                 *value = new_value;
            //             }
            //         }
            //         GetItemKey::PyArrayIndices(array_indices_) => {
            //             let array_indices = array_indices_.readonly();
            //             let array_indices = array_indices.as_array();
            //             for &index in array_indices {
            //                 new_indices.push(*indices.get(index as usize).ok_or_else(bad_index)?);
            //             }
            //         }
            //         GetItemKey::PyArrayMask(mask) => {
            //             for (&keep, &index) in mask.readonly().as_array().iter().zip(indices.iter()) {
            //                 if keep {
            //                     new_indices.push(index);
            //                 }
            //             }
            //         }
            //         GetItemKey::VectorIndices(vector_indices) => {
            //             for index in vector_indices {
            //                 new_indices.push(*indices.get(index).ok_or_else(bad_index)?);
            //             }
            //         }
            //         GetItemKey::VectorMask(mask) => {
            //             for (keep, &index) in mask.into_iter().zip(indices.iter()) {
            //                 if keep {
            //                     new_indices.push(index);
            //                 }
            //             }
            //         }
            //     };
            //     Ok(())
            // }

            pub fn __iadd__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                self.write()?.zip_with(other.read()?, |_, a, b| { *a += b });
                Ok(())
            }

            pub fn __isub__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                float_iop!(self, other, $type, -=)
            }

            pub fn __imul__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                float_iop!(self, other, $type, *=)
            }
            pub fn __itruediv__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                float_iop!(self, other, $type, /=)
            }
            pub fn __ifloordiv__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                float_rhs_zip_mut!(self, other, $type, |_, a: &mut $type, b: &$type| {
                    *a = a.div_euclid(*b)
                });
                Ok(())
            }
            pub fn __imod__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                float_iop!(self, other, $type, %=)
            }
            pub fn __ipow__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                let mut self_array = self.array.write().map_err(cannot_write)?;
                let self_indices = self.indices.0.read().map_err(cannot_read)?;
                let powf = |_, a: &mut $type, b: &$type| {
                    *a = a.powf(*b)
                };
                let powi = |_, a: &mut $type, b: &i32| {
                    *a = a.powi(*b)
                };
                match other {
                    FloatOpRhsValue::I64(other_value) => {
                        value_zip_mut!(self_array, self_indices, other_value as i32, powi);
                    }
                    FloatOpRhsValue::F64(other_value) => {
                        value_zip_mut!(self_array, self_indices, other_value as $type, powf);
                    }
                    FloatOpRhsValue::Float32(other_array) => {
                        array_zip_mut!(self_array, self_indices, other_array, $type, powf);
                    }
                    FloatOpRhsValue::Float64(other_array) => {
                        array_zip_mut!(self_array, self_indices, other_array, $type, powf);
                    }
                    FloatOpRhsValue::Int8(other_array) => {
                        array_zip_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatOpRhsValue::Int16(other_array) => {
                        array_zip_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatOpRhsValue::Int32(other_array) => {
                        array_zip_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatOpRhsValue::Int64(other_array) => {
                        array_zip_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatOpRhsValue::UInt8(other_array) => {
                        array_zip_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatOpRhsValue::UInt16(other_array) => {
                        array_zip_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatOpRhsValue::UInt32(other_array) => {
                        array_zip_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatOpRhsValue::UInt64(other_array) => {
                        array_zip_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatOpRhsValue::PyArrayF32(py_array) => {
                        py_array_zip_mut!(self_array, self_indices, py_array, $type, powf);
                    }
                    FloatOpRhsValue::PyArrayF64(py_array) => {
                        py_array_zip_mut!(self_array, self_indices, py_array, $type, powf);
                    }
                    FloatOpRhsValue::PyArrayI8(py_array) => {
                        py_array_zip_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatOpRhsValue::PyArrayI16(py_array) => {
                        py_array_zip_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatOpRhsValue::PyArrayI32(py_array) => {
                        py_array_zip_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatOpRhsValue::PyArrayI64(py_array) => {
                        py_array_zip_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatOpRhsValue::PyArrayU8(py_array) => {
                        py_array_zip_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatOpRhsValue::PyArrayU16(py_array) => {
                        py_array_zip_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatOpRhsValue::PyArrayU32(py_array) => {
                        py_array_zip_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatOpRhsValue::PyArrayU64(py_array) => {
                        py_array_zip_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                }
                Ok(())
            }
            pub fn __lt__(&self, other: FloatOpRhsValue) -> PyResult<ArrayViewIndices> {
                float_cmp!(self, other, $type, <)
            }
            pub fn __le__(&self, other: FloatOpRhsValue) -> PyResult<ArrayViewIndices> {
                float_cmp!(self, other, $type, <=)
            }
            pub fn __gt__(&self, other: FloatOpRhsValue) -> PyResult<ArrayViewIndices> {
                float_cmp!(self, other, $type, >)
            }
            pub fn __ge__(&self, other: FloatOpRhsValue) -> PyResult<ArrayViewIndices> {
                float_cmp!(self, other, $type, >=)
            }
            pub fn __eq__(&self, other: FloatOpRhsValue) -> PyResult<ArrayViewIndices> {
                float_cmp!(self, other, $type, ==)
            }
            pub fn __ne__(&self, other: FloatOpRhsValue) -> PyResult<ArrayViewIndices> {
                float_cmp!(self, other, $type, !=)
            }
        }
    };
}

macro_rules! int_iop {
    ($self:expr, $other:expr, $type:ty, $op:tt) => {
        {
            int_rhs_zip_mut!($self, $other, $type, |_, a: &mut $type, b: &$type| {
                *a $op b
            });
            Ok(())
        }
    }
}

macro_rules! int_array {
    (impl Array<$type:ty>) => {
        impl Array<$type> {
            // pub fn __setitem__(&mut self, key: GetItemKey, value: IntOpRhsValue) -> PyResult<()> {
            //     int_binary_op!(self.array, key, value, $type, |_, b| b);
            //     Ok(())
            // }
            pub fn __iadd__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                int_iop!(self, other, $type, +=)
            }
            pub fn __isub__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                int_iop!(self, other, $type, -=)
            }
            pub fn __imul__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                int_iop!(self, other, $type, *=)
            }
            pub fn __itruediv__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                int_iop!(self, other, $type, /=)
            }
            pub fn __ifloordiv__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                int_rhs_zip_mut!(self, other, $type, |_, a: &mut $type, b: &$type| {
                    *a = a.div_euclid(*b)
                });
                Ok(())
            }
            pub fn __imod__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                int_iop!(self, other, $type, %=)
            }
            pub fn __ipow__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                int_rhs_zip_mut!(self, other, $type, |_, a: &mut $type, b: &$type| {
                    *a = a.pow(*b as u32)
                });
                Ok(())
            }
            pub fn __lt__(&self, other: FloatOpRhsValue) -> PyResult<ArrayViewIndices> {
                int_cmp!(self, other, $type, <)
            }
            pub fn __le__(&self, other: FloatOpRhsValue) -> PyResult<ArrayViewIndices> {
                int_cmp!(self, other, $type, <=)
            }
            pub fn __gt__(&self, other: FloatOpRhsValue) -> PyResult<ArrayViewIndices> {
                int_cmp!(self, other, $type, >)
            }
            pub fn __ge__(&self, other: FloatOpRhsValue) -> PyResult<ArrayViewIndices> {
                int_cmp!(self, other, $type, >=)
            }
            pub fn __eq__(&self, other: FloatOpRhsValue) -> PyResult<ArrayViewIndices> {
                int_cmp!(self, other, $type, ==)
            }
            pub fn __ne__(&self, other: FloatOpRhsValue) -> PyResult<ArrayViewIndices> {
                int_cmp!(self, other, $type, !=)
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
    (pub mod $mod_name:ident { pub struct $name:ident($type:ty) }) => {
        pub mod $mod_name {
            use super::*;
            #[pyclass]
            pub struct $name(pub Array<$type>);
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
                pub fn p_new_view_with_indices(&self, indices: &ArrayViewIndices) -> Self {
                    Self(self.0.p_new_view_with_indices(indices))
                }
                pub fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<$type>>> {
                    self.0.numpy(py)
                }
                pub fn __len__(&self) -> PyResult<usize> {
                    self.0.__len__()
                }
                pub fn __getitem__(&self, key: GetItemKey) -> PyResult<Self> {
                    self.0.__getitem__(key).map(Self)
                }
                // pub fn __setitem__(&mut self, key: GetItemKey, value: FloatOpRhsValue) -> PyResult<()> {
                //     self.0.__setitem__(key, value)
                // }
                pub fn __iadd__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                    self.0.__iadd__(other)
                }
                pub fn __isub__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                    self.0.__isub__(other)
                }
                pub fn __imul__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                    self.0.__imul__(other)
                }
                pub fn __itruediv__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                    self.0.__itruediv__(other)
                }
                pub fn __ifloordiv__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                    self.0.__ifloordiv__(other)
                }
                pub fn __imod__(&mut self, other: FloatOpRhsValue) -> PyResult<()> {
                    self.0.__imod__(other)
                }
                #[args(modulo = "None")]
                pub fn __ipow__(
                    &mut self,
                    other: FloatOpRhsValue,
                    _modulo: &PyAny,
                ) -> PyResult<()> {
                    self.0.__ipow__(other)
                }
                pub fn __richcmp__(
                    &mut self,
                    other: FloatOpRhsValue,
                    op: CompareOp,
                ) -> PyResult<ArrayViewIndices> {
                    match op {
                        CompareOp::Lt => self.0.__lt__(other),
                        CompareOp::Le => self.0.__le__(other),
                        CompareOp::Gt => self.0.__gt__(other),
                        CompareOp::Ge => self.0.__ge__(other),
                        CompareOp::Eq => self.0.__eq__(other),
                        CompareOp::Ne => self.0.__ne__(other),
                    }
                }
            }
            impl $name {
                pub fn read(&self) -> PyResult<ReadableArray<$type>> {
                    self.0.read()
                }
                pub fn write(&mut self) -> PyResult<WriteableArray<$type>> {
                    self.0.write()
                }
            }
            impl<'lock> WriteableArray<'lock, $type> {
                pub fn zip_with<F>(&mut self, other: ReadableFloatOpRhsValue, f: F)
                where
                    F: Fn(&u32, &mut $type, &$type),
                {
                    match other {
                        ReadableFloatOpRhsValue::Float32(other) => {
                            for (self_index, &other_index) in
                                self.indices.iter().zip(other.indices.iter())
                            {
                                let self_value =
                                    unsafe { self.vec.get_unchecked_mut(*self_index as usize) };
                                let other_value =
                                    unsafe { other.vec.get_unchecked(other_index as usize) };
                                f(self_index, self_value, &(*other_value as $type))
                            }
                        }
                        _ => panic!(""),
                    }
                }
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
            pub fn p_new_view_with_indices(&self, indices: &ArrayViewIndices) -> Self {
                Self(self.0.p_new_view_with_indices(indices))
            }
            pub fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<$type>>> {
                self.0.numpy(py)
            }
            pub fn __getitem__(&self, key: GetItemKey) -> PyResult<Self> {
                self.0.__getitem__(key).map(Self)
            }
            // pub fn __setitem__(&mut self, key: GetItemKey, value: IntOpRhsValue) -> PyResult<()> {
            //     self.0.__setitem__(key, value)
            // }
            pub fn __len__(&self) -> PyResult<usize> {
                self.0.__len__()
            }
            pub fn __iadd__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                self.0.__iadd__(other)
            }
            pub fn __isub__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                self.0.__isub__(other)
            }
            pub fn __imul__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                self.0.__imul__(other)
            }
            pub fn __itruediv__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                self.0.__itruediv__(other)
            }
            pub fn __ifloordiv__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                self.0.__ifloordiv__(other)
            }
            pub fn __imod__(&mut self, other: IntOpRhsValue) -> PyResult<()> {
                self.0.__imod__(other)
            }
            #[args(modulo = "None")]
            pub fn __ipow__(&mut self, other: IntOpRhsValue, _modulo: &PyAny) -> PyResult<()> {
                self.0.__ipow__(other)
            }
            pub fn __richcmp__(
                &mut self,
                other: FloatOpRhsValue,
                op: CompareOp,
            ) -> PyResult<ArrayViewIndices> {
                match op {
                    CompareOp::Lt => self.0.__lt__(other),
                    CompareOp::Le => self.0.__le__(other),
                    CompareOp::Gt => self.0.__gt__(other),
                    CompareOp::Ge => self.0.__ge__(other),
                    CompareOp::Eq => self.0.__eq__(other),
                    CompareOp::Ne => self.0.__ne__(other),
                }
            }
        }
        impl $name {
            pub fn read(&self) -> PyResult<ReadableArray<$type>> {
                self.0.read()
            }
            pub fn write(&mut self) -> PyResult<WriteableArray<$type>> {
                self.0.write()
            }
        }
    };
}

python_float_array! {
    pub mod float32 { pub struct Float32(f32) }
}

python_float_array! {
    pub mod float64 { pub struct Float64(f64) }
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
