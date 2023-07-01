use crate::array_view_indices::ArrayViewIndices;
use crate::error_handlers::{cannot_read, cannot_write};
use crate::getitem_key::GetItemKey;
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

                pub fn __getitem__(&self, key: GetItemKey) -> PyResult<Self> {
                    Ok(Self {
                        array: Arc::clone(&self.array),
                        indices: self.indices.__getitem__(key)?,
                    })
                }

                pub fn __setitem__(&mut self, key: GetItemKey, value: Value) -> PyResult<()> {
                    let mut array = self.array.write().map_err(cannot_write)?;
                    let indices = self.indices.0.read().map_err(cannot_read)?;
                    match (key, value) {
                        (GetItemKey::Slice(slice), Value::One(item)) => {
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
                        (GetItemKey::ArrayMask(mask), Value::One(item)) => {
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
                        (GetItemKey::Slice(slice), Value::Many(items)) => {
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
                        (GetItemKey::ArrayMask(mask), Value::Many(items)) => {
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
                        (GetItemKey::Slice(slice), Value::ManyArray(items)) => {
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
                        (GetItemKey::ArrayMask(mask), Value::ManyArray(items)) => {
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

python_array! {
    pub mod float32 { struct Float32(f32) }
}

python_array! {
    pub mod float64 { struct Float64(f64) }
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
