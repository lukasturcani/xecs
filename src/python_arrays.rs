use crate::array_view_indices::ArrayViewIndices;
use crate::error_handlers::{bad_index, cannot_read, cannot_write};
use crate::getitem_key::Key;
use crate::index::Index;
use itertools::izip;
use numpy::PyArray1;
use pyo3::prelude::*;
use std::sync::{Arc, RwLock};

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
                indices: ArrayViewIndices,
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

                pub fn p_new_view_with_indices(&self, indices: &ArrayViewIndices) -> Self {
                    Self {
                        array: Arc::clone(&self.array),
                        indices: Arc::clone(&indices.0),
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
                        indices: Arc::clone(&indices.0),
                    })
                }

                pub fn __getitem__(&self, key: Key) -> PyResult<Self> {
                    let indices = self.indices.read().map_err(cannot_read)?;
                    let new_indices = match key {
                        Key::Slice(slice) => {
                            let slice_indices = slice.indices(indices.len() as i64)?;
                            let mut new_indices = Vec::with_capacity(slice.len()?);
                            for index in (slice_indices.start..slice_indices.stop)
                                .step_by(slice_indices.step as usize)
                            {
                                new_indices.push(*unsafe { indices.get_unchecked(index as usize) })
                            }
                            new_indices
                        }
                        Key::ArrayIndices(array_indices_) => {
                            let array_indices = array_indices_.readonly();
                            let array_indices = array_indices.as_array();
                            let mut new_indices = Vec::with_capacity(array_indices.len());
                            for &index in array_indices {
                                new_indices
                                    .push(*indices.get(index as usize).ok_or_else(bad_index)?);
                            }
                            new_indices
                        }
                        Key::ArrayMask(mask) => {
                            // Ideally the capacity if new_indices would be the number of
                            // true values in mask. However, because that would mean we count
                            // them first, we allocate for the worst-case scenario instead -- we
                            // assume all values in the mask are true.
                            let mut new_indices = Vec::with_capacity(mask.len());
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

python_array! {
    pub mod float64 { struct Float64(f64) }
}
