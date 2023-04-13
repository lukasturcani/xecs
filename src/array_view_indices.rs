use crate::error_handlers::{cannot_read, cannot_write};
use crate::index::Index;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::sync::{Arc, RwLock};

#[pyclass]
pub struct MultipleArrayViewIndices(pub Vec<Arc<RwLock<Vec<Index>>>>);

#[pyclass]
pub struct ArrayViewIndices(pub Arc<RwLock<Vec<Index>>>);

#[pymethods]
impl ArrayViewIndices {
    #[staticmethod]
    fn with_capacity(capacity: usize) -> Self {
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
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.0.read().map_err(cannot_read)?.len())
    }
}
