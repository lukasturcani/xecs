use crate::error_handlers::cannot_read;
use crate::{array_view_indices::ArrayViewIndices, error_handlers::cannot_write};
use itertools::izip;
use numpy::PyArray1;
use pyo3::prelude::*;
use std::sync::{Arc, RwLock};

#[derive(FromPyObject)]
enum BoolRhs<'a> {
    BoolValue(bool),
    Bool(PyRef<'a, Bool>),
    PyArrayBool(&'a PyArray1<bool>),
    VecBool(Vec<bool>),
}

/// An array of boolean values.
#[pyclass(module = "xecs")]
pub struct Bool {
    array: Arc<RwLock<Vec<bool>>>,
    indices: ArrayViewIndices,
}

#[pymethods]
impl Bool {
    #[staticmethod]
    fn p_from_value(value: bool, num: usize) -> PyResult<Self> {
        Ok(Self {
            array: Arc::new(RwLock::new(vec![value; num])),
            indices: ArrayViewIndices(Arc::new(RwLock::new((0_u32..(num as u32)).collect()))),
        })
    }

    #[staticmethod]
    fn p_from_numpy(array: &PyArray1<bool>) -> PyResult<Self> {
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
                false;
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
    /// Copy the elements into a NumPy array.
    ///
    /// Returns:
    ///     numpy.ndarray: The NumPy array.
    fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<bool>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let vec = indices
            .iter()
            .map(|&index| unsafe { *array.get_unchecked(index as usize) })
            .collect();
        Ok(PyArray1::from_vec(py, vec).into_py(py))
    }
    /// Set the values of the array.
    ///
    /// Parameters:
    ///     values (bool | list[bool]): The new values.
    fn fill(&mut self, values: BoolRhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match values {
            BoolRhs::BoolValue(other) => {
                for &index in indices.iter() {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) = other;
                    }
                }
            }
            BoolRhs::Bool(b) => {
                if !Arc::ptr_eq(&self.array, &b.array) {
                    let other_array = b.array.read().map_err(cannot_read)?;
                    let other_indices = b.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) =
                                *other_array.get_unchecked(other_index as usize);
                        }
                    }
                }
            }
            BoolRhs::PyArrayBool(py_array) => {
                for (&index, &value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) = value;
                    }
                }
            }
            BoolRhs::VecBool(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) = value;
                    }
                }
            }
        }
        Ok(())
    }
    /// Get the value at a specific index.
    ///
    /// Parameters:
    ///     index (int): The index where the value is located.
    /// Returns:
    ///     bool: The value at `index`.
    fn get(&self, index: usize) -> PyResult<bool> {
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let array = self.array.read().map_err(cannot_read)?;
        Ok(unsafe { *array.get_unchecked(*indices.get_unchecked(index) as usize) })
    }
    /// Get a string representation.
    ///
    /// Returns:
    ///     str: The string representation.
    fn to_str(&self) -> PyResult<String> {
        let mut result = String::new();
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let view: Vec<_> = indices
            .iter()
            .map(|index| unsafe { array.get_unchecked(*index as usize) })
            .collect();
        result += &format!("<xecs.Bool {view:?}>");
        Ok(result)
    }
    fn __str__(&self) -> PyResult<String> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let view: Vec<_> = indices
            .iter()
            .map(|index| unsafe { array.get_unchecked(*index as usize) })
            .collect();

        Ok(format!("<xecs.Bool {view:#?}>"))
    }
    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.indices.0.read().map_err(cannot_read)?.len())
    }
    fn __getitem__(&self, key: &PyArray1<bool>) -> PyResult<Self> {
        Ok(Self {
            array: Arc::clone(&self.array),
            indices: self.indices.__getitem__(key)?,
        })
    }
    fn __setitem__(&mut self, key: &PyArray1<bool>, rhs: BoolRhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mask = key.readonly();
        let mask = mask.as_array();
        match rhs {
            BoolRhs::BoolValue(other) => {
                for (&index, &keep) in indices.iter().zip(mask) {
                    if keep {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) = other;
                        }
                    }
                }
            }
            BoolRhs::Bool(b) => {
                if Arc::ptr_eq(&self.array, &b.array) {
                    let other_indices = b.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index, &keep) in
                        izip!(indices.iter(), other_indices.iter(), mask.iter())
                    {
                        if keep {
                            unsafe {
                                let other = *array.get_unchecked(other_index as usize);
                                *array.get_unchecked_mut(index as usize) = other;
                            }
                        }
                    }
                } else {
                    let other_array = b.array.read().map_err(cannot_read)?;
                    let other_indices = b.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index, &keep) in
                        izip!(indices.iter(), other_indices.iter(), mask.iter())
                    {
                        if keep {
                            unsafe {
                                *array.get_unchecked_mut(index as usize) =
                                    *other_array.get_unchecked(other_index as usize);
                            }
                        }
                    }
                }
            }
            BoolRhs::PyArrayBool(py_array) => {
                for (&index, &value, &keep) in
                    izip!(indices.iter(), py_array.readonly().as_array(), mask.iter())
                {
                    if keep {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) = value;
                        }
                    }
                }
            }
            BoolRhs::VecBool(vec) => {
                for (&index, value, &keep) in izip!(indices.iter(), vec, mask.iter()) {
                    if keep {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) = value;
                        }
                    }
                }
            }
        }
        Ok(())
    }
    fn __eq__(&self, py: Python, other: BoolRhs) -> PyResult<Py<PyArray1<bool>>> {
        let array = self.array.read().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match other {
            BoolRhs::BoolValue(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) == other);
                    }
                }
            }
            BoolRhs::Bool(b) => {
                let other_array = b.array.read().map_err(cannot_read)?;
                let other_indices = b.indices.0.read().map_err(cannot_read)?;
                for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                    unsafe {
                        result.push(
                            array.get_unchecked(index as usize)
                                == other_array.get_unchecked(other_index as usize),
                        );
                    }
                }
            }
            BoolRhs::PyArrayBool(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(array.get_unchecked(index as usize) == value);
                    }
                }
            }
            BoolRhs::VecBool(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) == value);
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    fn __ne__(&self, py: Python, other: BoolRhs) -> PyResult<Py<PyArray1<bool>>> {
        let array = self.array.read().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match other {
            BoolRhs::BoolValue(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) != other);
                    }
                }
            }
            BoolRhs::Bool(b) => {
                let other_array = b.array.read().map_err(cannot_read)?;
                let other_indices = b.indices.0.read().map_err(cannot_read)?;
                for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                    unsafe {
                        result.push(
                            array.get_unchecked(index as usize)
                                != other_array.get_unchecked(other_index as usize),
                        );
                    }
                }
            }
            BoolRhs::PyArrayBool(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(array.get_unchecked(index as usize) != value);
                    }
                }
            }
            BoolRhs::VecBool(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) != value);
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
}
