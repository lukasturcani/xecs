use crate::array_view_indices::ArrayViewIndices;
use crate::error_handlers::cannot_read;
use crate::getitem_key::GetItemKey;
use numpy::PyArray1;
use pyo3::prelude::*;
use std::sync::{Arc, RwLock};

pub struct Array<T> {
    pub array: Arc<RwLock<Vec<T>>>,
    pub indices: ArrayViewIndices,
}

impl<T> Array<T>
where
    T: numpy::Element + Copy,
{
    pub fn p_from_numpy(array: &PyArray1<T>) -> PyResult<Self> {
        Ok(Self {
            array: Arc::new(RwLock::new(array.to_vec()?)),
            indices: ArrayViewIndices(Arc::new(RwLock::new(
                ((0 as u32)..(array.len() as u32)).collect(),
            ))),
        })
    }

    pub fn to_vec(&self) -> PyResult<Vec<T>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        Ok(indices
            .iter()
            .map(|&index| unsafe { *array.get_unchecked(index as usize) })
            .collect())
    }

    pub fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<T>>> {
        Ok(PyArray1::from_vec(py, self.to_vec()?).into_py(py))
    }

    pub fn p_with_indices(indices: &ArrayViewIndices, default: T) -> PyResult<Self> {
        Ok(Self {
            array: Arc::new(RwLock::new(vec![
                default;
                indices
                    .0
                    .read()
                    .map_err(cannot_read)?
                    .capacity()
            ])),
            indices: ArrayViewIndices(Arc::clone(&indices.0)),
        })
    }
    pub fn p_new_view_with_indices(&self, indices: &ArrayViewIndices) -> Self {
        Self {
            array: Arc::clone(&self.array),
            indices: ArrayViewIndices(Arc::clone(&indices.0)),
        }
    }
    pub fn __getitem__(&self, key: GetItemKey) -> PyResult<Self> {
        Ok(Self {
            array: Arc::clone(&self.array),
            indices: self.indices.__getitem__(key)?,
        })
    }
    pub fn __len__(&self) -> PyResult<usize> {
        self.indices.__len__()
    }
}
