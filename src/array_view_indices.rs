use crate::error_handlers::{cannot_read, cannot_write};
use crate::index::Index;
use numpy::PyArray1;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::sync::{Arc, RwLock};

#[pyclass]
pub struct MultipleArrayViewIndices {
    indices: Vec<Arc<RwLock<Vec<Index>>>>,
    next: u8,
}

impl MultipleArrayViewIndices {
    pub fn new(indices: Vec<Arc<RwLock<Vec<Index>>>>) -> Self {
        Self { indices, next: 0 }
    }
}

#[pymethods]
impl MultipleArrayViewIndices {
    fn next(&mut self) -> Option<ArrayViewIndices> {
        if self.next < (self.indices.len() as u8) {
            self.next += 1;
            Some(ArrayViewIndices(Arc::clone(unsafe {
                self.indices.get_unchecked((self.next - 1) as usize)
            })))
        } else {
            None
        }
    }
}

/// Indices into the component pool which form the array view.
#[pyclass(module = "xecs")]
pub struct ArrayViewIndices(pub Arc<RwLock<Vec<Index>>>);

#[pymethods]
impl ArrayViewIndices {
    /// Construct a new, emtpy set of indices with a given capacity.
    ///
    /// Parameters:
    ///     capacity (int):
    ///         The amount of indices which can be held without
    ///         reallocating.
    /// Returns:
    ///     ArrayViewIndices: The new indices.
    #[staticmethod]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(RwLock::new(Vec::with_capacity(capacity))))
    }
    /// Add new indices to self.
    ///
    /// Parameters:
    ///     num (int): The number of new indices to add.
    /// Returns:
    ///     ArrayViewIndices: The newly added indices are returned.
    pub fn spawn(&mut self, num: Index) -> PyResult<Self> {
        let mut indices = self.0.write().map_err(cannot_write)?;
        let num_indices = indices.len() as Index;
        if num_indices + num > (indices.capacity() as Index) {
            Err(PyRuntimeError::new_err(
                "cannot spawn more entities because pool is full",
            ))
        } else {
            indices.extend(num_indices..num_indices + num);
            Ok(Self(Arc::new(RwLock::new(Vec::from_iter(
                num_indices..num_indices + num,
            )))))
        }
    }
    pub fn __len__(&self) -> PyResult<usize> {
        Ok(self.0.read().map_err(cannot_read)?.len())
    }

    pub fn __getitem__(&self, key: &PyArray1<bool>) -> PyResult<Self> {
        let indices = self.0.read().map_err(cannot_read)?;
        // Ideally the capacity of new_indices would be the number of
        // true values in key. However, because that would mean we count
        // them first, we allocate for the worst-case scenario instead -- we
        // assume all values in the key are true.
        let mut new_indices = Vec::with_capacity(key.len());
        for (&keep, &index) in key.readonly().as_array().iter().zip(indices.iter()) {
            if keep {
                new_indices.push(index);
            }
        }
        Ok(Self(Arc::new(RwLock::new(new_indices))))
    }
}
