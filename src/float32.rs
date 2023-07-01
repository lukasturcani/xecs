use crate::error_handlers::cannot_read;
use crate::getitem_key::GetItemKey;
use crate::{array_view_indices::ArrayViewIndices, error_handlers::cannot_write};
use numpy::PyArray1;
use pyo3::prelude::*;
use std::sync::{Arc, RwLock};

#[derive(FromPyObject)]
enum Float32Rhs<'a> {
    F32(f32),
    Float32(PyRef<'a, Float32>),
    PyArrayF32(&'a PyArray1<f32>),
}

#[pyclass]
pub struct Float32 {
    array: Arc<RwLock<Vec<f32>>>,
    indices: ArrayViewIndices,
}

#[pymethods]
impl Float32 {
    #[staticmethod]
    fn p_from_numpy(array: &PyArray1<f32>) -> PyResult<Self> {
        Ok(Self {
            array: Arc::new(RwLock::new(array.to_vec()?)),
            indices: ArrayViewIndices(Arc::new(RwLock::new(
                (0_u32..(array.len() as u32)).collect(),
            ))),
        })
    }
    #[staticmethod]
    fn p_with_indices(indices: &ArrayViewIndices) -> PyResult<Self> {
        Ok(Self {
            array: Arc::new(RwLock::new(vec![
                0.0;
                indices
                    .0
                    .read()
                    .map_err(cannot_read)?
                    .capacity()
            ])),
            indices: ArrayViewIndices(Arc::clone(&indices.0)),
        })
    }
    fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<f32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let vec = indices
            .iter()
            .map(|&index| unsafe { *array.get_unchecked(index as usize) })
            .collect();
        Ok(PyArray1::from_vec(py, vec).into_py(py))
    }
    fn __getitem__(&self, key: GetItemKey) -> PyResult<Self> {
        Ok(Self {
            array: Arc::clone(&self.array),
            indices: self.indices.__getitem__(key)?,
        })
    }
    fn __iadd__(&mut self, rhs: Float32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            Float32Rhs::F32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) += other;
                    }
                }
            }
            Float32Rhs::Float32(float32) => {
                let other_array = float32.array.read().map_err(cannot_read)?;
                let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) +=
                            other_array.get_unchecked(other_index as usize);
                    }
                }
            }
            Float32Rhs::PyArrayF32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) += value;
                    }
                }
            }
        }
        Ok(())
    }
}
