use crate::error_handlers::{bad_index, cannot_read, cannot_write};
use crate::getitem_key::GetItemKey;
use crate::index::Index;
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

#[pyclass]
pub struct ArrayViewIndices(pub Arc<RwLock<Vec<Index>>>);

#[pymethods]
impl ArrayViewIndices {
    #[staticmethod]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(RwLock::new(Vec::with_capacity(capacity))))
    }
    pub fn spawn(&mut self, num: Index) -> PyResult<()> {
        let mut indices = self.0.write().map_err(cannot_write)?;
        let num_indices = indices.len() as Index;
        if num_indices + num > (indices.capacity() as Index) {
            Err(PyRuntimeError::new_err(
                "cannot spawn more entities because pool is full",
            ))
        } else {
            indices.extend(num_indices..num_indices + num);
            Ok(())
        }
    }
    pub fn __len__(&self) -> PyResult<usize> {
        Ok(self.0.read().map_err(cannot_read)?.len())
    }
    pub fn __getitem__(&self, key: GetItemKey) -> PyResult<Self> {
        let indices = self.0.read().map_err(cannot_read)?;
        let new_indices = match key {
            GetItemKey::Slice(slice) => {
                let slice_indices = slice.indices(indices.len() as i64)?;
                let mut new_indices = Vec::with_capacity(slice_indices.slicelength as usize);
                for index in
                    (slice_indices.start..slice_indices.stop).step_by(slice_indices.step as usize)
                {
                    new_indices.push(*unsafe { indices.get_unchecked(index as usize) })
                }
                new_indices
            }
            GetItemKey::PyArrayIndices(array_indices_) => {
                let array_indices = array_indices_.readonly();
                let array_indices = array_indices.as_array();
                let mut new_indices = Vec::with_capacity(array_indices.len());
                for &index in array_indices {
                    new_indices.push(*indices.get(index as usize).ok_or_else(bad_index)?);
                }
                new_indices
            }
            GetItemKey::PyArrayMask(mask) => {
                // Ideally the capacity if new_indices would be the number of
                // true values in mask. However, because that would mean we count
                // them first, we allocate for the worst-case scenario instead -- we
                // assume all values in the mask are true.
                let mut new_indices = Vec::with_capacity(mask.len());
                for (&keep, &index) in mask.readonly().as_array().iter().zip(indices.iter()) {
                    if keep {
                        new_indices.push(index);
                    }
                }
                new_indices
            }
            GetItemKey::VectorIndices(vector_indices) => {
                let mut new_indices = Vec::with_capacity(vector_indices.len());
                for index in vector_indices {
                    new_indices.push(*indices.get(index).ok_or_else(bad_index)?);
                }
                new_indices
            }
            GetItemKey::VectorMask(mask) => {
                // Ideally the capacity if new_indices would be the number of
                // true values in mask. However, because that would mean we count
                // them first, we allocate for the worst-case scenario instead -- we
                // assume all values in the mask are true.
                let mut new_indices = Vec::with_capacity(mask.len());
                for (keep, &index) in mask.into_iter().zip(indices.iter()) {
                    if keep {
                        new_indices.push(index);
                    }
                }
                new_indices
            }
        };
        Ok(Self(Arc::new(RwLock::new(new_indices))))
    }
}
