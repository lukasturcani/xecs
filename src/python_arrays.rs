use crate::array_view_indices::ArrayViewIndices;
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
                indices: Arc<RwLock<Vec<Index>>>,
            }

            #[pymethods]
            impl $name {
                #[staticmethod]
                pub fn from_numpy(array: &PyArray1<$type>) -> PyResult<Self> {
                    Ok(Self {
                        array: Arc::new(RwLock::new(array.to_vec()?)),
                        indices: Arc::new(RwLock::new(
                            ((0 as u32)..(array.len() as u32)).collect(),
                        )),
                    })
                }

                pub fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<$type>>> {
                    let vec = self.array.read().map_err(cannot_read)?;
                    Ok(PyArray1::from_vec(py, vec.clone()).into_py(py))
                }

                pub fn p_spawn(&mut self, num: Index) -> PyResult<()> {
                    let mut indices = self.indices.write().map_err(cannot_write)?;
                    let num_indices = indices.len() as Index;
                    if num_indices + num > (self.array.read().map_err(cannot_read)?.len() as Index)
                    {
                        Err(PyRuntimeError::new_err(
                            "cannot spawn more entities because pool is full",
                        ))
                    } else {
                        indices.extend(num_indices..num_indices + num);
                        Ok(())
                    }
                }

                pub fn p_new_view_with_indices(&self, indices: &ArrayViewIndices) -> Self {
                    Self {
                        array: Arc::clone(&self.array),
                        indices: Arc::clone(&indices.0),
                    }
                }

                #[staticmethod]
                pub fn p_with_capacity(capacity: usize, indices: &ArrayViewIndices) -> Self {
                    Self {
                        array: Arc::new(RwLock::new(vec![0 as $type; capacity])),
                        indices: Arc::clone(&indices.0),
                    }
                }

                pub fn __getitem__(&self, key: Key) -> PyResult<Self> {
                    let indices = self.indices.read().map_err(cannot_read)?;
                    let new_indices = match key {
                        Key::Slice(slice) => {
                            let mut new_indices = Vec::with_capacity(indices.len());
                            let slice_indices = slice.indices(indices.len() as i64)?;
                            for index in (slice_indices.start..slice_indices.stop)
                                .step_by(slice_indices.step as usize)
                            {
                                new_indices.push(*unsafe { indices.get_unchecked(index as usize) })
                            }
                            new_indices
                        }
                        Key::ArrayIndices(array_indices) => {
                            let mut new_indices = Vec::with_capacity(indices.len());
                            for &index in array_indices.readonly().as_array() {
                                new_indices
                                    .push(*indices.get(index as usize).ok_or_else(bad_index)?);
                            }
                            new_indices
                        }
                        Key::ArrayMask(mask) => {
                            let mut new_indices = Vec::with_capacity(indices.len());
                            for (&keep, &index) in
                                mask.readonly().as_array().iter().zip(indices.iter())
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
                        indices: Arc::new(RwLock::new(new_indices)),
                    })
                }

                pub fn __setitem__(&mut self, key: Key, value: Value) -> PyResult<()> {
                    let mut array = self.array.write().map_err(cannot_write)?;
                    let indices = self.indices.read().map_err(cannot_read)?;
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
                                .zip(items.readonly().as_array())
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
                                .zip(items.readonly().as_array())
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
                    Ok(self.indices.read().map_err(cannot_read)?.len())
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
