use crate::index::Index;
use itertools::izip;
use numpy::PyArray1;
use pyo3::exceptions::{PyIndexError, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::types::PySlice;
use std::sync::{Arc, RwLock};

#[derive(FromPyObject)]
pub enum Key<'a> {
    Slice(&'a PySlice),
    ArrayIndices(&'a PyArray1<Index>),
    ArrayMask(&'a PyArray1<bool>),
}

macro_rules! python_array {
    (pub mod $mod_name:ident { struct $name:ident($type:ty) }) => {
        pub mod $mod_name {
            use super::*;

            #[derive(FromPyObject)]
            pub enum Value<'a> {
                One($type),
                Many(&'a PyArray1<$type>),
            }

            #[pyclass]
            pub struct $name {
                array: Arc<RwLock<Vec<$type>>>,
                indices: Arc<Vec<Index>>,
            }

            #[pymethods]
            impl $name {
                #[staticmethod]
                pub fn from_numpy(array: &PyArray1<$type>) -> PyResult<Self> {
                    Ok(Self {
                        array: Arc::new(RwLock::new(array.to_vec()?)),
                        indices: Arc::new(((0 as u32)..(array.len() as u32)).collect()),
                    })
                }

                pub fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<$type>>> {
                    let vec = self.array.read().map_err(cannot_read)?;
                    Ok(PyArray1::from_vec(py, vec.clone()).into_py(py))
                }

                pub fn p_spawn(&mut self, num: usize) {
                    self.indices.extend(
                        (self.indices.len() as Index)
                            ..(self.indices.len() as Index) + (num as Index),
                    )
                }

                #[staticmethod]
                pub fn p_create_pool(size: usize) -> Self {
                    Self {
                        array: Arc::new(RwLock::new(vec![0 as $type; size])),
                        indices: Arc::new(Vec::new()),
                    }
                }

                pub fn __getitem__(&self, key: Key) -> PyResult<Self> {
                    let indices = match key {
                        Key::Slice(slice) => {
                            let mut new_indices = Vec::with_capacity(self.indices.len());
                            let indices = slice.indices(self.indices.len() as i64)?;
                            for index in
                                (indices.start..indices.stop).step_by(indices.step as usize)
                            {
                                new_indices
                                    .push(*unsafe { self.indices.get_unchecked(index as usize) })
                            }
                            new_indices
                        }
                        Key::ArrayIndices(indices) => {
                            let mut new_indices = Vec::with_capacity(indices.len());
                            for &index in indices.readonly().as_array() {
                                new_indices
                                    .push(*self.indices.get(index as usize).ok_or_else(bad_index)?);
                            }
                            new_indices
                        }
                        Key::ArrayMask(mask) => {
                            let mut new_indices = Vec::with_capacity(self.indices.len());
                            for (&keep, &index) in
                                mask.readonly().as_array().iter().zip(self.indices.iter())
                            {
                                if keep {
                                    new_indices.push(index);
                                }
                            }
                            new_indices
                        }
                    };
                    Ok(Self {
                        array: Arc::clone(&self.array),
                        indices: Arc::clone(indices),
                    })
                }

                pub fn __setitem__(&mut self, key: Key, value: Value) -> PyResult<()> {
                    match (key, value) {
                        (Key::Slice(slice), Value::One(item)) => {
                            let indices = slice.indices(self.indices.len() as i64)?;
                            let mut array = self.array.write().map_err(cannot_write)?;
                            for index in
                                (indices.start..indices.stop).step_by(indices.step as usize)
                            {
                                unsafe {
                                    *array.get_unchecked_mut(
                                        *self.indices.get_unchecked(index as usize) as usize,
                                    ) = item;
                                };
                            }
                        }
                        (Key::ArrayIndices(indices), Value::One(item)) => {
                            let mut array = self.array.write().map_err(cannot_write)?;
                            for &index in indices.readonly().as_array() {
                                let array_index =
                                    *self.indices.get(index as usize).ok_or_else(bad_index)?;
                                unsafe {
                                    *array.get_unchecked_mut(array_index as usize) = item;
                                }
                            }
                        }
                        (Key::ArrayMask(mask), Value::One(item)) => {
                            let mut array = self.array.write().map_err(cannot_write)?;
                            for (&keep, &index) in
                                mask.readonly().as_array().iter().zip(self.indices.iter())
                            {
                                if keep {
                                    unsafe {
                                        *array.get_unchecked_mut(
                                            *self.indices.get_unchecked(index as usize) as usize,
                                        ) = item;
                                    }
                                }
                            }
                        }
                        (Key::Slice(slice), Value::Many(items)) => {
                            let indices = slice.indices(self.indices.len() as i64)?;
                            let mut array = self.array.write().map_err(cannot_write)?;
                            for (index, &item) in (indices.start..indices.stop)
                                .step_by(indices.step as usize)
                                .zip(items.readonly().as_array())
                            {
                                unsafe {
                                    *array.get_unchecked_mut(
                                        *self.indices.get_unchecked(index as usize) as usize,
                                    ) = item;
                                };
                            }
                        }
                        (Key::ArrayIndices(indices), Value::Many(items)) => {
                            let mut array = self.array.write().map_err(cannot_write)?;
                            for (&index, &item) in indices
                                .readonly()
                                .as_array()
                                .iter()
                                .zip(items.readonly().as_array())
                            {
                                let array_index =
                                    *self.indices.get(index as usize).ok_or_else(bad_index)?;
                                unsafe {
                                    *array.get_unchecked_mut(array_index as usize) = item;
                                }
                            }
                        }
                        (Key::ArrayMask(mask), Value::Many(items)) => {
                            let mut array = self.array.write().map_err(cannot_write)?;
                            for (&keep, &index, &item) in izip!(
                                mask.readonly().as_array().iter(),
                                self.indices.iter(),
                                items.readonly().as_array()
                            ) {
                                if keep {
                                    unsafe {
                                        *array.get_unchecked_mut(
                                            *self.indices.get_unchecked(index as usize) as usize,
                                        ) = item;
                                    }
                                }
                            }
                        }
                    }
                    Ok(())
                }
                pub fn __len__(&self) -> usize {
                    self.indices.len()
                }
            }
        }
    };
}
fn cannot_read<T>(_err: T) -> PyErr {
    PyRuntimeError::new_err("cannot read array")
}

fn bad_index() -> PyErr {
    PyIndexError::new_err("index out of range")
}

fn cannot_write<T>(_err: T) -> PyErr {
    PyRuntimeError::new_err("cannot mutate array")
}

python_array! {
    pub mod float64 { struct Float64(f64) }
}
