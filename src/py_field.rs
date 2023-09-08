use std::sync::{Arc, RwLock};

use pyo3::{exceptions::PyRuntimeError, prelude::*};

use crate::{
    array_view_indices::ArrayViewIndices,
    error_handlers::{cannot_read, cannot_write},
};

#[pyclass]
pub struct PyField {
    array: Arc<RwLock<Vec<Option<PyObject>>>>,
    indices: ArrayViewIndices,
}

#[pymethods]
impl PyField {
    #[staticmethod]
    fn p_with_indices(indices: &ArrayViewIndices) -> PyResult<Self> {
        Ok(Self {
            array: Arc::new(RwLock::new(vec![
                None;
                indices
                    .0
                    .read()
                    .map_err(cannot_read)?
                    .capacity()
            ])),
            indices: ArrayViewIndices(Arc::clone(&indices.0)),
        })
    }
    fn p_new_view_with_indices(&self, indices: &ArrayViewIndices) -> Self {
        Self {
            array: Arc::clone(&self.array),
            indices: ArrayViewIndices(Arc::clone(&indices.0)),
        }
    }
    fn fill(&mut self, py: Python, value: PyObject) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        for &index in indices.iter() {
            unsafe {
                *array.get_unchecked_mut(index as usize) = Some(Py::clone_ref(&value, py));
            }
        }

        Ok(())
    }
    fn get(&self, index: usize) -> PyResult<PyObject> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let object = unsafe { array.get_unchecked(*indices.get_unchecked(index) as usize) }.clone();
        object.map_or(Err(PyRuntimeError::new_err("invalid object")), |value| {
            Ok(value)
        })
    }
}
