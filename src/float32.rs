use crate::error_handlers::cannot_read;
use crate::{array_view_indices::ArrayViewIndices, error_handlers::cannot_write};
use itertools::izip;
use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use std::sync::{Arc, RwLock};

#[derive(FromPyObject)]
enum Float32Rhs<'a> {
    F32(f32),
    Float32(PyRef<'a, Float32>),
    PyArrayF32(&'a PyArray1<f32>),
    VecF32(Vec<f32>),
}

#[derive(FromPyObject)]
enum PowRhs<'a> {
    I32(i32),
    F32(f32),
    Float32(PyRef<'a, Float32>),
    // TODO: I can probably pretty safely add support for Int32 here.
    PyArrayF32(&'a PyArray1<f32>),
    VecI32(Vec<i32>),
    VecF32(Vec<f32>),
}

/// An array of float32 values.
#[pyclass(module = "xecs")]
pub struct Float32 {
    array: Arc<RwLock<Vec<f32>>>,
    indices: ArrayViewIndices,
}

#[pymethods]
impl Float32 {
    #[staticmethod]
    fn p_from_value(value: f32, num: usize) -> PyResult<Self> {
        Ok(Self {
            array: Arc::new(RwLock::new(vec![value; num])),
            indices: ArrayViewIndices(Arc::new(RwLock::new((0_u32..(num as u32)).collect()))),
        })
    }

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
    fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<f32>>> {
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
    ///     values (float | list[float]): The new values.
    fn fill(&mut self, values: Float32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match values {
            Float32Rhs::F32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) = other;
                    }
                }
            }
            Float32Rhs::Float32(float32) => {
                if !Arc::ptr_eq(&self.array, &float32.array) {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) =
                                *other_array.get_unchecked(other_index as usize);
                        }
                    }
                }
            }
            Float32Rhs::PyArrayF32(py_array) => {
                for (&index, &value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) = value;
                    }
                }
            }
            Float32Rhs::VecF32(vec) => {
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
    ///     float: The value at `index`.
    fn get(&self, index: usize) -> PyResult<f32> {
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
        result += &format!("<xecs.Float32 {view:?}>");
        Ok(result)
    }
    fn __str__(&self) -> PyResult<String> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let view: Vec<_> = indices
            .iter()
            .map(|index| unsafe { array.get_unchecked(*index as usize) })
            .collect();

        Ok(format!("<xecs.Float32 {view:#?}>"))
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
    fn __setitem__(&mut self, key: &PyArray1<bool>, rhs: Float32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mask = key.readonly();
        let mask = mask.as_array();
        match rhs {
            Float32Rhs::F32(other) => {
                for (&index, &keep) in indices.iter().zip(mask) {
                    if keep {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) = other;
                        }
                    }
                }
            }
            Float32Rhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
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
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
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
            Float32Rhs::PyArrayF32(py_array) => {
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
            Float32Rhs::VecF32(vec) => {
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
    fn __add__(&self, py: Python, rhs: Float32Rhs) -> PyResult<Py<PyArray1<f32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            Float32Rhs::F32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) + other);
                    }
                }
            }
            Float32Rhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            result.push(*array.get_unchecked(index as usize) + other);
                        }
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            result.push(
                                *array.get_unchecked(index as usize)
                                    + other_array.get_unchecked(other_index as usize),
                            );
                        }
                    }
                }
            }
            Float32Rhs::PyArrayF32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) + value);
                    }
                }
            }
            Float32Rhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) + value);
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
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
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            *array.get_unchecked_mut(index as usize) += other;
                        }
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) +=
                                other_array.get_unchecked(other_index as usize);
                        }
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
            Float32Rhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) += value;
                    }
                }
            }
        }
        Ok(())
    }
    fn __sub__(&self, py: Python, rhs: Float32Rhs) -> PyResult<Py<PyArray1<f32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            Float32Rhs::F32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) - other);
                    }
                }
            }
            Float32Rhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            result.push(*array.get_unchecked(index as usize) - other);
                        }
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            result.push(
                                *array.get_unchecked(index as usize)
                                    - other_array.get_unchecked(other_index as usize),
                            );
                        }
                    }
                }
            }
            Float32Rhs::PyArrayF32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) - value);
                    }
                }
            }
            Float32Rhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) - value);
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    fn __isub__(&mut self, rhs: Float32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            Float32Rhs::F32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) -= other;
                    }
                }
            }
            Float32Rhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            *array.get_unchecked_mut(index as usize) -= other;
                        }
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) -=
                                other_array.get_unchecked(other_index as usize);
                        }
                    }
                }
            }
            Float32Rhs::PyArrayF32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) -= value;
                    }
                }
            }
            Float32Rhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) -= value;
                    }
                }
            }
        }
        Ok(())
    }
    fn __mul__(&self, py: Python, rhs: Float32Rhs) -> PyResult<Py<PyArray1<f32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            Float32Rhs::F32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) * other);
                    }
                }
            }
            Float32Rhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            result.push(*array.get_unchecked(index as usize) * other);
                        }
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            result.push(
                                *array.get_unchecked(index as usize)
                                    * other_array.get_unchecked(other_index as usize),
                            );
                        }
                    }
                }
            }
            Float32Rhs::PyArrayF32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) * value);
                    }
                }
            }
            Float32Rhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) * value);
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    fn __imul__(&mut self, rhs: Float32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            Float32Rhs::F32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) *= other;
                    }
                }
            }
            Float32Rhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            *array.get_unchecked_mut(index as usize) *= other;
                        }
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) *=
                                other_array.get_unchecked(other_index as usize);
                        }
                    }
                }
            }
            Float32Rhs::PyArrayF32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) *= value;
                    }
                }
            }
            Float32Rhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) *= value;
                    }
                }
            }
        }
        Ok(())
    }
    fn __truediv__(&self, py: Python, rhs: Float32Rhs) -> PyResult<Py<PyArray1<f32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            Float32Rhs::F32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) / other);
                    }
                }
            }
            Float32Rhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            result.push(*array.get_unchecked(index as usize) / other);
                        }
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            result.push(
                                *array.get_unchecked(index as usize)
                                    / other_array.get_unchecked(other_index as usize),
                            );
                        }
                    }
                }
            }
            Float32Rhs::PyArrayF32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) / value);
                    }
                }
            }
            Float32Rhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) / value);
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    fn __itruediv__(&mut self, rhs: Float32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            Float32Rhs::F32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) /= other;
                    }
                }
            }
            Float32Rhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            *array.get_unchecked_mut(index as usize) /= other;
                        }
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) /=
                                other_array.get_unchecked(other_index as usize);
                        }
                    }
                }
            }
            Float32Rhs::PyArrayF32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) /= value;
                    }
                }
            }
            Float32Rhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) /= value;
                    }
                }
            }
        }
        Ok(())
    }
    fn __floordiv__(&self, py: Python, rhs: Float32Rhs) -> PyResult<Py<PyArray1<f32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            Float32Rhs::F32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).div_euclid(other));
                    }
                }
            }
            Float32Rhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            result.push(array.get_unchecked(index as usize).div_euclid(other));
                        }
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            result.push(
                                array
                                    .get_unchecked(index as usize)
                                    .div_euclid(*other_array.get_unchecked(other_index as usize)),
                            );
                        }
                    }
                }
            }
            Float32Rhs::PyArrayF32(py_array) => {
                for (&index, &value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).div_euclid(value));
                    }
                }
            }
            Float32Rhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).div_euclid(value));
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    fn __ifloordiv__(&mut self, rhs: Float32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            Float32Rhs::F32(other) => {
                for &index in indices.iter() {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.div_euclid(other);
                }
            }
            Float32Rhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        let other = unsafe { *array.get_unchecked(other_index as usize) };
                        let a = unsafe { array.get_unchecked_mut(index as usize) };
                        *a = a.div_euclid(other);
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        let other = unsafe { other_array.get_unchecked(other_index as usize) };
                        let a = unsafe { array.get_unchecked_mut(index as usize) };
                        *a = a.div_euclid(*other);
                    }
                }
            }
            Float32Rhs::PyArrayF32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.div_euclid(*value);
                }
            }
            Float32Rhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.div_euclid(value);
                }
            }
        }
        Ok(())
    }
    fn __mod__(&self, py: Python, rhs: Float32Rhs) -> PyResult<Py<PyArray1<f32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            Float32Rhs::F32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) % other);
                    }
                }
            }
            Float32Rhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            result.push(*array.get_unchecked(index as usize) % other);
                        }
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            result.push(
                                *array.get_unchecked(index as usize)
                                    % other_array.get_unchecked(other_index as usize),
                            );
                        }
                    }
                }
            }
            Float32Rhs::PyArrayF32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) % value);
                    }
                }
            }
            Float32Rhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) % value);
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    fn __imod__(&mut self, rhs: Float32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            Float32Rhs::F32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) %= other;
                    }
                }
            }
            Float32Rhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            *array.get_unchecked_mut(index as usize) %= other;
                        }
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) %=
                                other_array.get_unchecked(other_index as usize);
                        }
                    }
                }
            }
            Float32Rhs::PyArrayF32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) %= value;
                    }
                }
            }
            Float32Rhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) %= value;
                    }
                }
            }
        }
        Ok(())
    }
    #[args(_modulo = "None")]
    fn __pow__(&self, py: Python, rhs: PowRhs, _modulo: &PyAny) -> PyResult<Py<PyArray1<f32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            PowRhs::I32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).powi(other));
                    }
                }
            }
            PowRhs::F32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).powf(other));
                    }
                }
            }
            PowRhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            result.push(array.get_unchecked(index as usize).powf(other));
                        }
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = other_array.get_unchecked(other_index as usize);
                            result.push(array.get_unchecked(index as usize).powf(*other));
                        }
                    }
                }
            }
            PowRhs::PyArrayF32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).powf(*value));
                    }
                }
            }
            PowRhs::VecI32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).powi(value));
                    }
                }
            }
            PowRhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).powf(value));
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    #[args(_modulo = "None")]
    fn __ipow__(&mut self, rhs: PowRhs, _modulo: &PyAny) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            PowRhs::I32(other) => {
                for &index in indices.iter() {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.powi(other);
                }
            }
            PowRhs::F32(other) => {
                for &index in indices.iter() {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.powf(other);
                }
            }
            PowRhs::Float32(float32) => {
                if Arc::ptr_eq(&self.array, &float32.array) {
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        let other = unsafe { *array.get_unchecked(other_index as usize) };
                        let a = unsafe { array.get_unchecked_mut(index as usize) };
                        *a = a.powf(other);
                    }
                } else {
                    let other_array = float32.array.read().map_err(cannot_read)?;
                    let other_indices = float32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        let other = unsafe { other_array.get_unchecked(other_index as usize) };
                        let a = unsafe { array.get_unchecked_mut(index as usize) };
                        *a = a.powf(*other);
                    }
                }
            }
            PowRhs::PyArrayF32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.powf(*value);
                }
            }
            PowRhs::VecI32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.powi(value);
                }
            }
            PowRhs::VecF32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.powf(value);
                }
            }
        }
        Ok(())
    }
    fn __richcmp__(
        &self,
        py: Python,
        other: Float32Rhs,
        op: CompareOp,
    ) -> PyResult<Py<PyArray1<bool>>> {
        match op {
            CompareOp::Lt => lt(py, self, other),
            CompareOp::Le => le(py, self, other),
            CompareOp::Gt => gt(py, self, other),
            CompareOp::Ge => ge(py, self, other),
            CompareOp::Eq => eq(py, self, other),
            CompareOp::Ne => ne(py, self, other),
        }
    }
}

fn lt(py: Python, lhs: &Float32, rhs: Float32Rhs) -> PyResult<Py<PyArray1<bool>>> {
    let array = lhs.array.read().map_err(cannot_write)?;
    let indices = lhs.indices.0.read().map_err(cannot_read)?;
    let mut result = Vec::with_capacity(indices.len());
    match rhs {
        Float32Rhs::F32(other) => {
            for &index in indices.iter() {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) < other);
                }
            }
        }
        Float32Rhs::Float32(float32) => {
            let other_array = float32.array.read().map_err(cannot_read)?;
            let other_indices = float32.indices.0.read().map_err(cannot_read)?;
            for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                unsafe {
                    result.push(
                        array.get_unchecked(index as usize)
                            < other_array.get_unchecked(other_index as usize),
                    );
                }
            }
        }
        Float32Rhs::PyArrayF32(py_array) => {
            for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                unsafe {
                    result.push(array.get_unchecked(index as usize) < value);
                }
            }
        }
        Float32Rhs::VecF32(vec) => {
            for (&index, value) in indices.iter().zip(vec) {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) < value);
                }
            }
        }
    }
    Ok(PyArray1::from_vec(py, result).into_py(py))
}

fn le(py: Python, lhs: &Float32, rhs: Float32Rhs) -> PyResult<Py<PyArray1<bool>>> {
    let array = lhs.array.read().map_err(cannot_write)?;
    let indices = lhs.indices.0.read().map_err(cannot_read)?;
    let mut result = Vec::with_capacity(indices.len());
    match rhs {
        Float32Rhs::F32(other) => {
            for &index in indices.iter() {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) <= other);
                }
            }
        }
        Float32Rhs::Float32(float32) => {
            let other_array = float32.array.read().map_err(cannot_read)?;
            let other_indices = float32.indices.0.read().map_err(cannot_read)?;
            for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                unsafe {
                    result.push(
                        array.get_unchecked(index as usize)
                            <= other_array.get_unchecked(other_index as usize),
                    );
                }
            }
        }
        Float32Rhs::PyArrayF32(py_array) => {
            for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                unsafe {
                    result.push(array.get_unchecked(index as usize) <= value);
                }
            }
        }
        Float32Rhs::VecF32(vec) => {
            for (&index, value) in indices.iter().zip(vec) {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) <= value);
                }
            }
        }
    }
    Ok(PyArray1::from_vec(py, result).into_py(py))
}

fn gt(py: Python, lhs: &Float32, rhs: Float32Rhs) -> PyResult<Py<PyArray1<bool>>> {
    let array = lhs.array.read().map_err(cannot_write)?;
    let indices = lhs.indices.0.read().map_err(cannot_read)?;
    let mut result = Vec::with_capacity(indices.len());
    match rhs {
        Float32Rhs::F32(other) => {
            for &index in indices.iter() {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) > other);
                }
            }
        }
        Float32Rhs::Float32(float32) => {
            let other_array = float32.array.read().map_err(cannot_read)?;
            let other_indices = float32.indices.0.read().map_err(cannot_read)?;
            for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                unsafe {
                    result.push(
                        array.get_unchecked(index as usize)
                            > other_array.get_unchecked(other_index as usize),
                    );
                }
            }
        }
        Float32Rhs::PyArrayF32(py_array) => {
            for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                unsafe {
                    result.push(array.get_unchecked(index as usize) > value);
                }
            }
        }
        Float32Rhs::VecF32(vec) => {
            for (&index, value) in indices.iter().zip(vec) {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) > value);
                }
            }
        }
    }
    Ok(PyArray1::from_vec(py, result).into_py(py))
}

fn ge(py: Python, lhs: &Float32, rhs: Float32Rhs) -> PyResult<Py<PyArray1<bool>>> {
    let array = lhs.array.read().map_err(cannot_write)?;
    let indices = lhs.indices.0.read().map_err(cannot_read)?;
    let mut result = Vec::with_capacity(indices.len());
    match rhs {
        Float32Rhs::F32(other) => {
            for &index in indices.iter() {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) >= other);
                }
            }
        }
        Float32Rhs::Float32(float32) => {
            let other_array = float32.array.read().map_err(cannot_read)?;
            let other_indices = float32.indices.0.read().map_err(cannot_read)?;
            for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                unsafe {
                    result.push(
                        array.get_unchecked(index as usize)
                            >= other_array.get_unchecked(other_index as usize),
                    );
                }
            }
        }
        Float32Rhs::PyArrayF32(py_array) => {
            for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                unsafe {
                    result.push(array.get_unchecked(index as usize) >= value);
                }
            }
        }
        Float32Rhs::VecF32(vec) => {
            for (&index, value) in indices.iter().zip(vec) {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) >= value);
                }
            }
        }
    }
    Ok(PyArray1::from_vec(py, result).into_py(py))
}

fn eq(py: Python, lhs: &Float32, rhs: Float32Rhs) -> PyResult<Py<PyArray1<bool>>> {
    let array = lhs.array.read().map_err(cannot_write)?;
    let indices = lhs.indices.0.read().map_err(cannot_read)?;
    let mut result = Vec::with_capacity(indices.len());
    match rhs {
        Float32Rhs::F32(other) => {
            for &index in indices.iter() {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) == other);
                }
            }
        }
        Float32Rhs::Float32(float32) => {
            let other_array = float32.array.read().map_err(cannot_read)?;
            let other_indices = float32.indices.0.read().map_err(cannot_read)?;
            for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                unsafe {
                    result.push(
                        array.get_unchecked(index as usize)
                            == other_array.get_unchecked(other_index as usize),
                    );
                }
            }
        }
        Float32Rhs::PyArrayF32(py_array) => {
            for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                unsafe {
                    result.push(array.get_unchecked(index as usize) == value);
                }
            }
        }
        Float32Rhs::VecF32(vec) => {
            for (&index, value) in indices.iter().zip(vec) {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) == value);
                }
            }
        }
    }
    Ok(PyArray1::from_vec(py, result).into_py(py))
}

fn ne(py: Python, lhs: &Float32, rhs: Float32Rhs) -> PyResult<Py<PyArray1<bool>>> {
    let array = lhs.array.read().map_err(cannot_write)?;
    let indices = lhs.indices.0.read().map_err(cannot_read)?;
    let mut result = Vec::with_capacity(indices.len());
    match rhs {
        Float32Rhs::F32(other) => {
            for &index in indices.iter() {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) != other);
                }
            }
        }
        Float32Rhs::Float32(float32) => {
            let other_array = float32.array.read().map_err(cannot_read)?;
            let other_indices = float32.indices.0.read().map_err(cannot_read)?;
            for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                unsafe {
                    result.push(
                        array.get_unchecked(index as usize)
                            != other_array.get_unchecked(other_index as usize),
                    );
                }
            }
        }
        Float32Rhs::PyArrayF32(py_array) => {
            for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                unsafe {
                    result.push(array.get_unchecked(index as usize) != value);
                }
            }
        }
        Float32Rhs::VecF32(vec) => {
            for (&index, value) in indices.iter().zip(vec) {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) != value);
                }
            }
        }
    }
    Ok(PyArray1::from_vec(py, result).into_py(py))
}
