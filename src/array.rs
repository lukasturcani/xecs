use crate::array_view::ArrayView;
use numpy::PyArray1;
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use std::sync::{Arc, RwLock};

fn cannot_read<T>(_err: T) -> PyErr {
    PyRuntimeError::new_err("cannot read array")
}

pub struct Array<T>(Arc<RwLock<Vec<T>>>);

impl<T> Array<T>
where
    T: numpy::Element,
{
    pub fn from_numpy(array: &PyArray1<T>) -> PyResult<Self> {
        Ok(Self(Arc::new(RwLock::new(array.to_vec()?))))
    }

    pub fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<T>>> {
        let vec = self.0.read().map_err(cannot_read)?;
        Ok(PyArray1::from_vec(py, vec.clone()).into_py(py))
    }

    pub fn view(&self) -> PyResult<ArrayView<T>> {
        Ok(ArrayView {
            array: Arc::clone(&self.0),
            indices: (0..self.0.read().map_err(cannot_read)?.len() as u32).collect(),
        })
    }
}
