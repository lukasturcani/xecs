use crate::array_view_indices::ArrayViewIndices;
use crate::error_handlers::{bad_index, cannot_read, cannot_write};
use crate::getitem_key::Key;
use crate::mask::Mask;
use itertools::izip;
use numpy::PyArray1;
use pyo3::prelude::*;
use std::sync::{Arc, RwLock};

struct Array<T> {
    array: Arc<RwLock<Vec<T>>>,
    indices: ArrayViewIndices,
}

#[derive(FromPyObject)]
pub enum FloatMathRhsValue<'a> {
    I64(i64),
    F64(f64),
    Float32(PyRef<'a, Float32>),
    Float64(PyRef<'a, Float64>),
    Int8(PyRef<'a, Int8>),
    Int16(PyRef<'a, Int16>),
    Int32(PyRef<'a, Int32>),
    Int64(PyRef<'a, Int64>),
    UInt8(PyRef<'a, Int8>),
    UInt16(PyRef<'a, Int16>),
    UInt32(PyRef<'a, Int32>),
    UInt64(PyRef<'a, Int64>),
}

#[derive(FromPyObject)]
pub enum IntMathRhsValue<'a> {
    I64(i64),
    Int8(PyRef<'a, Int8>),
    Int16(PyRef<'a, Int16>),
    Int32(PyRef<'a, Int32>),
    Int64(PyRef<'a, Int64>),
    UInt8(PyRef<'a, Int8>),
    UInt16(PyRef<'a, Int16>),
    UInt32(PyRef<'a, Int32>),
    UInt64(PyRef<'a, Int64>),
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

    pub fn p_new_view_with_indices(&self, indices: &ArrayViewIndices) -> Self {
        Self {
            array: Arc::clone(&self.array),
            indices: ArrayViewIndices(Arc::clone(&indices.0)),
        }
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

    pub fn __getitem__(&self, mask: &Mask) -> PyResult<Self> {
        Ok(Self {
            array: Arc::clone(&self.array),
            indices: self.indices.__getitem__(mask)?,
        })
    }
    pub fn __len__(&self) -> PyResult<usize> {
        self.indices.__len__()
    }
}

macro_rules! value_op {
    ($self_array:ident, $self_indices:ident, $other_value:ident, $type:ty, $op:tt) => {
        for &self_index in $self_indices.iter() {
            let self_value = unsafe { $self_array.get_unchecked_mut(self_index as usize) };
            *self_value $op *$other_value as $type;
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

macro_rules! float_math_op {
    ($self:ident, $other:ident, $type:ty, $op:tt) => {
        let mut self_array = $self.array.write().map_err(cannot_write)?;
        let self_indices = $self.indices.0.read().map_err(cannot_read)?;
        match $other {
            FloatMathRhsValue::I64(other_value) => {
                value_op!(self_array, self_indices, other_value, $type, $op);
            }
            FloatMathRhsValue::F64(other_value) => {
                value_op!(self_array, self_indices, other_value, $type, $op);
            }
            FloatMathRhsValue::Float32(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            FloatMathRhsValue::Float64(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            FloatMathRhsValue::Int8(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            FloatMathRhsValue::Int16(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            FloatMathRhsValue::Int32(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            FloatMathRhsValue::Int64(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            FloatMathRhsValue::UInt8(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            FloatMathRhsValue::UInt16(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            FloatMathRhsValue::UInt32(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            FloatMathRhsValue::UInt64(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
        }
    };
}

macro_rules! float_array {
    (impl Array<$type:ty>) => {
        impl Array<$type> {
            pub fn __setitem__(&mut self, key: Key, value: &FloatMathRhsValue) -> PyResult<()> {
                Ok(())
            }

            pub fn __iadd__(&mut self, other: &FloatMathRhsValue) -> PyResult<()> {
                float_math_op!(self, other, $type, +=);
                Ok(())
            }

            pub fn __isub__(&mut self, other: &FloatMathRhsValue) -> PyResult<()> {
                float_math_op!(self, other, $type, -=);
                Ok(())
            }
        }
    };
}

macro_rules! int_math_op {
    ($self:ident, $other:ident, $type:ty, $op:tt) => {
        let mut self_array = $self.array.write().map_err(cannot_write)?;
        let self_indices = $self.indices.0.read().map_err(cannot_read)?;
        match $other {
            IntMathRhsValue::I64(other_value) => {
                value_op!(self_array, self_indices, other_value, $type, $op);
            }
            IntMathRhsValue::Int8(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntMathRhsValue::Int16(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntMathRhsValue::Int32(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntMathRhsValue::Int64(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntMathRhsValue::UInt8(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntMathRhsValue::UInt16(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntMathRhsValue::UInt32(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
            IntMathRhsValue::UInt64(other) => {
                array_op!(self_array, self_indices, other, $type, $op);
            }
        }
    };
}

macro_rules! int_array {
    (impl Array<$type:ty>) => {
        impl Array<$type> {
            pub fn __iadd__(&mut self, other: &IntMathRhsValue) -> PyResult<()> {
                int_math_op!(self, other, $type, +=);
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

macro_rules! python_array {
    (pub mod $mod_name:ident { struct $name:ident($type:ty) }) => {
        pub mod $mod_name {
            use super::*;

            #[derive(FromPyObject)]
            pub enum Value<'a> {
                One($type),
                Many(PyRef<'a, $name>),
                ManyArray(&'a PyArray1<$type>),
            }

            #[pyclass]
            pub struct $name {
                array: Arc<RwLock<Vec<$type>>>,
                indices: ArrayViewIndices,
            }

            #[pymethods]
            impl $name {
                #[staticmethod]
                pub fn from_numpy(array: &PyArray1<$type>) -> PyResult<Self> {
                    Ok(Self {
                        array: Arc::new(RwLock::new(array.to_vec()?)),
                        indices: ArrayViewIndices(Arc::new(RwLock::new(
                            ((0 as u32)..(array.len() as u32)).collect(),
                        ))),
                    })
                }

                pub fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<$type>>> {
                    let vec = self.array.read().map_err(cannot_read)?;
                    Ok(PyArray1::from_vec(py, vec.clone()).into_py(py))
                }

                pub fn p_new_view_with_indices(&self, indices: &ArrayViewIndices) -> Self {
                    Self {
                        array: Arc::clone(&self.array),
                        indices: ArrayViewIndices(Arc::clone(&indices.0)),
                    }
                }

                #[staticmethod]
                pub fn p_with_indices(indices: &ArrayViewIndices) -> PyResult<Self> {
                    Ok(Self {
                        array: Arc::new(RwLock::new(vec![
                            0 as $type;
                            indices
                                .0
                                .read()
                                .map_err(cannot_read)?
                                .capacity()
                        ])),
                        indices: ArrayViewIndices(Arc::clone(&indices.0)),
                    })
                }

                pub fn __getitem__(&self, key: Key) -> PyResult<Self> {
                    Ok(Self {
                        array: Arc::clone(&self.array),
                        indices: self.indices.__getitem__(key)?,
                    })
                }

                pub fn __setitem__(&mut self, key: Key, value: Value) -> PyResult<()> {
                    let mut array = self.array.write().map_err(cannot_write)?;
                    let indices = self.indices.0.read().map_err(cannot_read)?;
                    match (key, value) {
                        (Key::Slice(slice), Value::One(item)) => {
                            let slice_indices = slice.indices(indices.len() as i64)?;
                            for index in (slice_indices.start..slice_indices.stop)
                                .step_by(slice_indices.step as usize)
                            {
                                unsafe {
                                    *array.get_unchecked_mut(
                                        *indices.get_unchecked(index as usize) as usize,
                                    ) = item;
                                };
                            }
                        }
                        (Key::ArrayIndices(array_indices), Value::One(item)) => {
                            for &index in array_indices.readonly().as_array() {
                                let array_index =
                                    *indices.get(index as usize).ok_or_else(bad_index)?;
                                unsafe {
                                    *array.get_unchecked_mut(array_index as usize) = item;
                                }
                            }
                        }
                        (Key::ArrayMask(mask), Value::One(item)) => {
                            for (&keep, &index) in
                                mask.readonly().as_array().iter().zip(indices.iter())
                            {
                                if keep {
                                    unsafe {
                                        *array.get_unchecked_mut(
                                            *indices.get_unchecked(index as usize) as usize,
                                        ) = item;
                                    }
                                }
                            }
                        }
                        (Key::Slice(slice), Value::Many(items)) => {
                            let slice_indices = slice.indices(indices.len() as i64)?;
                            for (index, &item) in (slice_indices.start..slice_indices.stop)
                                .step_by(slice_indices.step as usize)
                                .zip(items.array.read().map_err(cannot_read)?.iter())
                            {
                                unsafe {
                                    *array.get_unchecked_mut(
                                        *indices.get_unchecked(index as usize) as usize,
                                    ) = item;
                                };
                            }
                        }
                        (Key::ArrayIndices(array_indices), Value::Many(items)) => {
                            for (&index, &item) in array_indices
                                .readonly()
                                .as_array()
                                .iter()
                                .zip(items.array.read().map_err(cannot_read)?.iter())
                            {
                                let array_index =
                                    *indices.get(index as usize).ok_or_else(bad_index)?;
                                unsafe {
                                    *array.get_unchecked_mut(array_index as usize) = item;
                                }
                            }
                        }
                        (Key::ArrayMask(mask), Value::Many(items)) => {
                            for (&keep, &index, &item) in izip!(
                                mask.readonly().as_array().iter(),
                                indices.iter(),
                                items.array.read().map_err(cannot_read)?.iter()
                            ) {
                                if keep {
                                    unsafe {
                                        *array.get_unchecked_mut(
                                            *indices.get_unchecked(index as usize) as usize,
                                        ) = item;
                                    }
                                }
                            }
                        }
                        (Key::Slice(slice), Value::ManyArray(items)) => {
                            let slice_indices = slice.indices(indices.len() as i64)?;
                            for (index, &item) in (slice_indices.start..slice_indices.stop)
                                .step_by(slice_indices.step as usize)
                                .zip(items.readonly().as_array())
                            {
                                unsafe {
                                    *array.get_unchecked_mut(
                                        *indices.get_unchecked(index as usize) as usize,
                                    ) = item;
                                };
                            }
                        }
                        (Key::ArrayIndices(array_indices), Value::ManyArray(items)) => {
                            for (&index, &item) in array_indices
                                .readonly()
                                .as_array()
                                .iter()
                                .zip(items.readonly().as_array())
                            {
                                let array_index =
                                    *indices.get(index as usize).ok_or_else(bad_index)?;
                                unsafe {
                                    *array.get_unchecked_mut(array_index as usize) = item;
                                }
                            }
                        }
                        (Key::ArrayMask(mask), Value::ManyArray(items)) => {
                            for (&keep, &index, &item) in izip!(
                                mask.readonly().as_array().iter(),
                                indices.iter(),
                                items.readonly().as_array()
                            ) {
                                if keep {
                                    unsafe {
                                        *array.get_unchecked_mut(
                                            *indices.get_unchecked(index as usize) as usize,
                                        ) = item;
                                    }
                                }
                            }
                        }
                    }
                    Ok(())
                }
                pub fn __len__(&self) -> PyResult<usize> {
                    self.indices.__len__()
                }

                pub fn __iadd__(&mut self, other: Value) -> PyResult<()> {
                    let mut self_array = self.array.write().map_err(cannot_write)?;
                    let self_indices = self.indices.0.read().map_err(cannot_read)?;

                    match other {
                        Value::One(item) => {
                            self_indices.iter().for_each(|&index| unsafe {
                                *self_array.get_unchecked_mut(index as usize) += item;
                            });
                        }
                        Value::Many(other) => {
                            let other_array = other.array.read().map_err(cannot_read)?;
                            let other_indices = other.indices.0.read().map_err(cannot_read)?;
                            self_indices.iter().zip(other_indices.iter()).for_each(
                                |(&self_index, &other_index)| unsafe {
                                    *self_array.get_unchecked_mut(self_index as usize) +=
                                        other_array.get_unchecked(other_index as usize);
                                },
                            );
                        }
                        Value::ManyArray(other) => {
                            self_indices
                                .iter()
                                .zip(other.readonly().as_array())
                                .for_each(|(&self_index, &item)| unsafe {
                                    *self_array.get_unchecked_mut(self_index as usize) += item;
                                });
                        }
                    }
                    Ok(())
                }
                pub fn __isub__(&mut self, other: Value) -> PyResult<()> {
                    let mut self_array = self.array.write().map_err(cannot_write)?;
                    let self_indices = self.indices.0.read().map_err(cannot_read)?;

                    match other {
                        Value::One(item) => {
                            self_indices.iter().for_each(|&index| unsafe {
                                *self_array.get_unchecked_mut(index as usize) -= item;
                            });
                        }
                        Value::Many(other) => {
                            let other_array = other.array.read().map_err(cannot_read)?;
                            let other_indices = other.indices.0.read().map_err(cannot_read)?;
                            self_indices.iter().zip(other_indices.iter()).for_each(
                                |(&self_index, &other_index)| unsafe {
                                    *self_array.get_unchecked_mut(self_index as usize) -=
                                        other_array.get_unchecked(other_index as usize);
                                },
                            );
                        }
                        Value::ManyArray(other) => {
                            self_indices
                                .iter()
                                .zip(other.readonly().as_array())
                                .for_each(|(&self_index, &item)| unsafe {
                                    *self_array.get_unchecked_mut(self_index as usize) -= item;
                                });
                        }
                    }
                    Ok(())
                }

                pub fn __imul__(&mut self, other: Value) -> PyResult<()> {
                    let mut self_array = self.array.write().map_err(cannot_write)?;
                    let self_indices = self.indices.0.read().map_err(cannot_read)?;

                    match other {
                        Value::One(item) => {
                            self_indices.iter().for_each(|&index| unsafe {
                                *self_array.get_unchecked_mut(index as usize) *= item;
                            });
                        }
                        Value::Many(other) => {
                            let other_array = other.array.read().map_err(cannot_read)?;
                            let other_indices = other.indices.0.read().map_err(cannot_read)?;
                            self_indices.iter().zip(other_indices.iter()).for_each(
                                |(&self_index, &other_index)| unsafe {
                                    *self_array.get_unchecked_mut(self_index as usize) *=
                                        other_array.get_unchecked(other_index as usize);
                                },
                            );
                        }
                        Value::ManyArray(other) => {
                            self_indices
                                .iter()
                                .zip(other.readonly().as_array())
                                .for_each(|(&self_index, &item)| unsafe {
                                    *self_array.get_unchecked_mut(self_index as usize) *= item;
                                });
                        }
                    }
                    Ok(())
                }

                pub fn __itruediv__(&mut self, other: Value) -> PyResult<()> {
                    let mut self_array = self.array.write().map_err(cannot_write)?;
                    let self_indices = self.indices.0.read().map_err(cannot_read)?;

                    match other {
                        Value::One(item) => {
                            self_indices.iter().for_each(|&index| unsafe {
                                *self_array.get_unchecked_mut(index as usize) /= item;
                            });
                        }
                        Value::Many(other) => {
                            let other_array = other.array.read().map_err(cannot_read)?;
                            let other_indices = other.indices.0.read().map_err(cannot_read)?;
                            self_indices.iter().zip(other_indices.iter()).for_each(
                                |(&self_index, &other_index)| unsafe {
                                    *self_array.get_unchecked_mut(self_index as usize) /=
                                        other_array.get_unchecked(other_index as usize);
                                },
                            );
                        }
                        Value::ManyArray(other) => {
                            self_indices
                                .iter()
                                .zip(other.readonly().as_array())
                                .for_each(|(&self_index, &item)| unsafe {
                                    *self_array.get_unchecked_mut(self_index as usize) /= item;
                                });
                        }
                    }
                    Ok(())
                }

                pub fn __ifloordiv__(&mut self, other: Value) -> PyResult<()> {
                    let mut self_array = self.array.write().map_err(cannot_write)?;
                    let self_indices = self.indices.0.read().map_err(cannot_read)?;

                    match other {
                        Value::One(item) => {
                            self_indices.iter().for_each(|&index| unsafe {
                                *self_array.get_unchecked_mut(index as usize) = self_array
                                    .get_unchecked_mut(index as usize)
                                    .div_euclid(item);
                            });
                        }
                        Value::Many(other) => {
                            let other_array = other.array.read().map_err(cannot_read)?;
                            let other_indices = other.indices.0.read().map_err(cannot_read)?;
                            self_indices.iter().zip(other_indices.iter()).for_each(
                                |(&self_index, &other_index)| unsafe {
                                    *self_array.get_unchecked_mut(self_index as usize) = self_array
                                        .get_unchecked_mut(self_index as usize)
                                        .div_euclid(
                                            *other_array.get_unchecked(other_index as usize),
                                        );
                                },
                            );
                        }
                        Value::ManyArray(other) => {
                            self_indices
                                .iter()
                                .zip(other.readonly().as_array())
                                .for_each(|(&self_index, &item)| unsafe {
                                    *self_array.get_unchecked_mut(self_index as usize) = self_array
                                        .get_unchecked_mut(self_index as usize)
                                        .div_euclid(item);
                                });
                        }
                    }
                    Ok(())
                }

                pub fn __imod__(&mut self, other: Value) -> PyResult<()> {
                    let mut self_array = self.array.write().map_err(cannot_write)?;
                    let self_indices = self.indices.0.read().map_err(cannot_read)?;

                    match other {
                        Value::One(item) => {
                            self_indices.iter().for_each(|&index| unsafe {
                                *self_array.get_unchecked_mut(index as usize) %= item;
                            });
                        }
                        Value::Many(other) => {
                            let other_array = other.array.read().map_err(cannot_read)?;
                            let other_indices = other.indices.0.read().map_err(cannot_read)?;
                            self_indices.iter().zip(other_indices.iter()).for_each(
                                |(&self_index, &other_index)| unsafe {
                                    *self_array.get_unchecked_mut(self_index as usize) %=
                                        other_array.get_unchecked(other_index as usize);
                                },
                            );
                        }
                        Value::ManyArray(other) => {
                            self_indices
                                .iter()
                                .zip(other.readonly().as_array())
                                .for_each(|(&self_index, &item)| unsafe {
                                    *self_array.get_unchecked_mut(self_index as usize) %= item;
                                });
                        }
                    }
                    Ok(())
                }

                #[args(_modulo = "None")]
                pub fn __ipow__(&mut self, other: Value, _modulo: &PyAny) -> PyResult<()> {
                    let mut self_array = self.array.write().map_err(cannot_write)?;
                    let self_indices = self.indices.0.read().map_err(cannot_read)?;

                    match other {
                        Value::One(item) => {
                            self_indices.iter().for_each(|&index| unsafe {
                                self_array.get_unchecked_mut(index as usize).power(item);
                            });
                        }
                        Value::Many(other) => {
                            let other_array = other.array.read().map_err(cannot_read)?;
                            let other_indices = other.indices.0.read().map_err(cannot_read)?;
                            self_indices.iter().zip(other_indices.iter()).for_each(
                                |(&self_index, &other_index)| unsafe {
                                    self_array
                                        .get_unchecked_mut(self_index as usize)
                                        .power(*other_array.get_unchecked(other_index as usize));
                                },
                            );
                        }
                        Value::ManyArray(other) => {
                            self_indices
                                .iter()
                                .zip(other.readonly().as_array())
                                .for_each(|(&self_index, &item)| unsafe {
                                    self_array
                                        .get_unchecked_mut(self_index as usize)
                                        .power(item);
                                });
                        }
                    }
                    Ok(())
                }
            }
        }
    };
}

macro_rules! python_float_array {
    (pub struct $name:ident($type:ty)) => {
        #[pyclass]
        pub struct $name(Array<$type>);
        #[pymethods]
        impl $name {
            pub fn p_new_view_with_indices(&self, indices: &ArrayViewIndices) -> Self {
                Self(self.0.p_new_view_with_indices(indices))
            }
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
            pub fn __iadd__(&mut self, other: FloatMathRhsValue) -> PyResult<()> {
                self.0.__iadd__(&other)
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
            pub fn p_new_view_with_indices(&self, indices: &ArrayViewIndices) -> Self {
                Self(self.0.p_new_view_with_indices(indices))
            }
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
            pub fn __len__(&self) -> PyResult<usize> {
                self.0.__len__()
            }
            pub fn __iadd__(&mut self, other: IntMathRhsValue) -> PyResult<()> {
                self.0.__iadd__(&other)
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

trait Power {
    type Other;
    fn power(self, other: Self::Other);
}

impl Power for &mut f64 {
    type Other = f64;
    fn power(self, other: Self::Other) {
        // TODO: Check to if special casing integers to use powi improves
        // performance.
        *self = self.powf(other);
    }
}

impl Power for &mut f32 {
    type Other = f32;
    fn power(self, other: Self::Other) {
        // TODO: Check to if special casing integers to use powi improves
        // performance.
        *self = self.powf(other);
    }
}
