use crate::entity_index::EntityIndex;
use itertools::izip;
use numpy::PyArray1;
use pyo3::{
    exceptions::{PyIndexError, PyRuntimeError},
    prelude::*,
    types::PySlice,
};
use std::sync::{Arc, RwLock};

pub struct ArrayView<T> {
    pub array: Arc<RwLock<Vec<T>>>,
    pub indices: Vec<EntityIndex>,
}

impl<T> ArrayView<T>
where
    T: numpy::Element + Copy,
{
    pub fn __getitem__(&self, key: Key) -> PyResult<Self> {
        let indices = match key {
            Key::Slice(slice) => {
                let mut new_indices = Vec::with_capacity(self.indices.len());
                let indices = slice.indices(self.indices.len() as i64)?;
                for index in (indices.start..indices.stop).step_by(indices.step as usize) {
                    new_indices.push(*unsafe { self.indices.get_unchecked(index as usize) })
                }
                new_indices
            }
            Key::ArrayIndices(indices) => {
                let mut new_indices = Vec::with_capacity(indices.len());
                for &index in indices.readonly().as_array() {
                    new_indices.push(*self.indices.get(index as usize).ok_or_else(bad_index)?);
                }
                new_indices
            }
            Key::ArrayMask(mask) => {
                let mut new_indices = Vec::with_capacity(self.indices.len());
                for (&keep, &index) in mask.readonly().as_array().iter().zip(self.indices.iter()) {
                    if keep {
                        new_indices.push(index);
                    }
                }
                new_indices
            }
        };
        Ok(Self {
            array: Arc::clone(&self.array),
            indices,
        })
    }

    pub fn __setitem__(&mut self, key: Key, value: Value<T>) -> PyResult<()> {
        match (key, value) {
            (Key::Slice(slice), Value::One(item)) => {
                let indices = slice.indices(self.indices.len() as i64)?;
                let mut array = self.array.write().map_err(cannot_write)?;
                for index in (indices.start..indices.stop).step_by(indices.step as usize) {
                    unsafe {
                        *array.get_unchecked_mut(
                            *self.indices.get_unchecked(index as usize) as usize
                        ) = item;
                    };
                }
            }
            (Key::ArrayIndices(indices), Value::One(item)) => {
                let mut array = self.array.write().map_err(cannot_write)?;
                for &index in indices.readonly().as_array() {
                    let array_index = *self.indices.get(index as usize).ok_or_else(bad_index)?;
                    unsafe {
                        *array.get_unchecked_mut(array_index as usize) = item;
                    }
                }
            }
            (Key::ArrayMask(mask), Value::One(item)) => {
                let mut array = self.array.write().map_err(cannot_write)?;
                for (&keep, &index) in mask.readonly().as_array().iter().zip(self.indices.iter()) {
                    if keep {
                        unsafe {
                            *array.get_unchecked_mut(
                                *self.indices.get_unchecked(index as usize) as usize
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
                            *self.indices.get_unchecked(index as usize) as usize
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
                    let array_index = *self.indices.get(index as usize).ok_or_else(bad_index)?;
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
                                *self.indices.get_unchecked(index as usize) as usize
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

#[derive(FromPyObject)]
pub enum Key<'a> {
    Slice(&'a PySlice),
    ArrayIndices(&'a PyArray1<EntityIndex>),
    ArrayMask(&'a PyArray1<bool>),
}

pub enum Value<'a, T> {
    One(T),
    Many(&'a PyArray1<T>),
}

fn bad_index() -> PyErr {
    PyIndexError::new_err("index out of range")
}

fn cannot_write<T>(_err: T) -> PyErr {
    PyRuntimeError::new_err("cannot mutate array")
}
