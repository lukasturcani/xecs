use crate::error_handlers::cannot_read;
use crate::{array_view_indices::ArrayViewIndices, error_handlers::cannot_write};
use itertools::izip;
use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use std::sync::{Arc, RwLock};

#[derive(FromPyObject)]
pub enum UInt32Rhs<'a> {
    U32(u32),
    UInt32(PyRef<'a, UInt32>),
    PyArrayU32(&'a PyArray1<u32>),
    VecU32(Vec<u32>),
}

/// An array of uint32 values.
#[pyclass(module = "xecs")]
pub struct UInt32 {
    array: Arc<RwLock<Vec<u32>>>,
    indices: ArrayViewIndices,
}

#[pymethods]
impl UInt32 {
    #[staticmethod]
    fn p_default_value() -> u32 {
        0
    }
    #[staticmethod]
    fn p_from_value(value: u32, num: usize) -> PyResult<Self> {
        Ok(Self {
            array: Arc::new(RwLock::new(vec![value; num])),
            indices: ArrayViewIndices(Arc::new(RwLock::new((0_u32..(num as u32)).collect()))),
        })
    }

    #[staticmethod]
    fn p_from_numpy(array: &PyArray1<u32>) -> PyResult<Self> {
        Ok(Self {
            array: Arc::new(RwLock::new(array.to_vec()?)),
            indices: ArrayViewIndices(Arc::new(RwLock::new(
                (0_u32..(array.len() as u32)).collect(),
            ))),
        })
    }
    #[staticmethod]
    pub fn p_from_indices(indices: &ArrayViewIndices, default: u32) -> PyResult<Self> {
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
    /// Copy the elements into a NumPy array.
    ///
    /// Returns:
    ///     numpy.ndarray: The NumPy array.
    fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<u32>>> {
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
    ///     values (int | list[int]): The new values.
    pub fn fill(&mut self, values: UInt32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match values {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) = other;
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if !Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) =
                                *other_array.get_unchecked(other_index as usize);
                        }
                    }
                }
            }
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, &value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) = value;
                    }
                }
            }
            UInt32Rhs::VecU32(vec) => {
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
    ///     int: The value at `index`.
    fn get(&self, index: usize) -> PyResult<u32> {
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
        result += &format!("<xecs.UInt32 {view:?}>");
        Ok(result)
    }
    fn __str__(&self) -> PyResult<String> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let view: Vec<_> = indices
            .iter()
            .map(|index| unsafe { array.get_unchecked(*index as usize) })
            .collect();

        Ok(format!("<xecs.UInt32 {view:#?}>"))
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
    fn __setitem__(&mut self, key: &PyArray1<bool>, rhs: UInt32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mask = key.readonly();
        let mask = mask.as_array();
        match rhs {
            UInt32Rhs::U32(other) => {
                for (&index, &keep) in indices.iter().zip(mask) {
                    if keep {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) = other;
                        }
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
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
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
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
            UInt32Rhs::PyArrayU32(py_array) => {
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
            UInt32Rhs::VecU32(vec) => {
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
    fn __add__(&self, py: Python, rhs: UInt32Rhs) -> PyResult<Py<PyArray1<u32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) + other);
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            result.push(*array.get_unchecked(index as usize) + other);
                        }
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
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
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) + value);
                    }
                }
            }
            UInt32Rhs::VecU32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) + value);
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    fn __iadd__(&mut self, rhs: UInt32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) += other;
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            *array.get_unchecked_mut(index as usize) += other;
                        }
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) +=
                                other_array.get_unchecked(other_index as usize);
                        }
                    }
                }
            }
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) += value;
                    }
                }
            }
            UInt32Rhs::VecU32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) += value;
                    }
                }
            }
        }
        Ok(())
    }
    fn __sub__(&self, py: Python, rhs: UInt32Rhs) -> PyResult<Py<PyArray1<u32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) - other);
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            result.push(*array.get_unchecked(index as usize) - other);
                        }
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
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
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) - value);
                    }
                }
            }
            UInt32Rhs::VecU32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) - value);
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    fn __isub__(&mut self, rhs: UInt32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) -= other;
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            *array.get_unchecked_mut(index as usize) -= other;
                        }
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) -=
                                other_array.get_unchecked(other_index as usize);
                        }
                    }
                }
            }
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) -= value;
                    }
                }
            }
            UInt32Rhs::VecU32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) -= value;
                    }
                }
            }
        }
        Ok(())
    }
    fn __mul__(&self, py: Python, rhs: UInt32Rhs) -> PyResult<Py<PyArray1<u32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) * other);
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            result.push(*array.get_unchecked(index as usize) * other);
                        }
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
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
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) * value);
                    }
                }
            }
            UInt32Rhs::VecU32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) * value);
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    fn __imul__(&mut self, rhs: UInt32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) *= other;
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            *array.get_unchecked_mut(index as usize) *= other;
                        }
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) *=
                                other_array.get_unchecked(other_index as usize);
                        }
                    }
                }
            }
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) *= value;
                    }
                }
            }
            UInt32Rhs::VecU32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) *= value;
                    }
                }
            }
        }
        Ok(())
    }
    fn __truediv__(&self, py: Python, rhs: UInt32Rhs) -> PyResult<Py<PyArray1<u32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) / other);
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            result.push(*array.get_unchecked(index as usize) / other);
                        }
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
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
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) / value);
                    }
                }
            }
            UInt32Rhs::VecU32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) / value);
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    fn __itruediv__(&mut self, rhs: UInt32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) /= other;
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            *array.get_unchecked_mut(index as usize) /= other;
                        }
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) /=
                                other_array.get_unchecked(other_index as usize);
                        }
                    }
                }
            }
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) /= value;
                    }
                }
            }
            UInt32Rhs::VecU32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) /= value;
                    }
                }
            }
        }
        Ok(())
    }
    fn __floordiv__(&self, py: Python, rhs: UInt32Rhs) -> PyResult<Py<PyArray1<u32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).div_euclid(other));
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            result.push(array.get_unchecked(index as usize).div_euclid(other));
                        }
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
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
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, &value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).div_euclid(value));
                    }
                }
            }
            UInt32Rhs::VecU32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).div_euclid(value));
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    fn __ifloordiv__(&mut self, rhs: UInt32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.div_euclid(other);
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        let other = unsafe { *array.get_unchecked(other_index as usize) };
                        let a = unsafe { array.get_unchecked_mut(index as usize) };
                        *a = a.div_euclid(other);
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        let other = unsafe { other_array.get_unchecked(other_index as usize) };
                        let a = unsafe { array.get_unchecked_mut(index as usize) };
                        *a = a.div_euclid(*other);
                    }
                }
            }
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.div_euclid(*value);
                }
            }
            UInt32Rhs::VecU32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.div_euclid(value);
                }
            }
        }
        Ok(())
    }
    fn __mod__(&self, py: Python, rhs: UInt32Rhs) -> PyResult<Py<PyArray1<u32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) % other);
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            result.push(*array.get_unchecked(index as usize) % other);
                        }
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
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
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) % value);
                    }
                }
            }
            UInt32Rhs::VecU32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(*array.get_unchecked(index as usize) % value);
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    fn __imod__(&mut self, rhs: UInt32Rhs) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) %= other;
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            *array.get_unchecked_mut(index as usize) %= other;
                        }
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            *array.get_unchecked_mut(index as usize) %=
                                other_array.get_unchecked(other_index as usize);
                        }
                    }
                }
            }
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        *array.get_unchecked_mut(index as usize) %= value;
                    }
                }
            }
            UInt32Rhs::VecU32(vec) => {
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
    fn __pow__(&self, py: Python, rhs: UInt32Rhs, _modulo: &PyAny) -> PyResult<Py<PyArray1<u32>>> {
        let array = self.array.read().map_err(cannot_read)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        let mut result = Vec::with_capacity(indices.len());
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).pow(other));
                    }
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = *array.get_unchecked(other_index as usize);
                            assert!(other > 0);
                            result.push(array.get_unchecked(index as usize).pow(other as u32));
                        }
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        unsafe {
                            let other = other_array.get_unchecked(other_index as usize);
                            assert!(*other > 0);
                            result.push(array.get_unchecked(index as usize).pow(*other as u32));
                        }
                    }
                }
            }
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).pow(*value));
                    }
                }
            }
            UInt32Rhs::VecU32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    unsafe {
                        result.push(array.get_unchecked(index as usize).pow(value));
                    }
                }
            }
        }
        Ok(PyArray1::from_vec(py, result).into_py(py))
    }
    #[args(_modulo = "None")]
    fn __ipow__(&mut self, rhs: UInt32Rhs, _modulo: &PyAny) -> PyResult<()> {
        let mut array = self.array.write().map_err(cannot_write)?;
        let indices = self.indices.0.read().map_err(cannot_read)?;
        match rhs {
            UInt32Rhs::U32(other) => {
                for &index in indices.iter() {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.pow(other);
                }
            }
            UInt32Rhs::UInt32(uint32) => {
                if Arc::ptr_eq(&self.array, &uint32.array) {
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        let other = unsafe { *array.get_unchecked(other_index as usize) };
                        assert!(other > 0);
                        let a = unsafe { array.get_unchecked_mut(index as usize) };
                        *a = a.pow(other as u32);
                    }
                } else {
                    let other_array = uint32.array.read().map_err(cannot_read)?;
                    let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
                    for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                        let other = unsafe { other_array.get_unchecked(other_index as usize) };
                        assert!(*other > 0);
                        let a = unsafe { array.get_unchecked_mut(index as usize) };
                        *a = a.pow(*other as u32);
                    }
                }
            }
            UInt32Rhs::PyArrayU32(py_array) => {
                for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.pow(*value);
                }
            }
            UInt32Rhs::VecU32(vec) => {
                for (&index, value) in indices.iter().zip(vec) {
                    let a = unsafe { array.get_unchecked_mut(index as usize) };
                    *a = a.pow(value);
                }
            }
        }
        Ok(())
    }
    fn __richcmp__(
        &self,
        py: Python,
        other: UInt32Rhs,
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

fn lt(py: Python, lhs: &UInt32, rhs: UInt32Rhs) -> PyResult<Py<PyArray1<bool>>> {
    let array = lhs.array.read().map_err(cannot_write)?;
    let indices = lhs.indices.0.read().map_err(cannot_read)?;
    let mut result = Vec::with_capacity(indices.len());
    match rhs {
        UInt32Rhs::U32(other) => {
            for &index in indices.iter() {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) < other);
                }
            }
        }
        UInt32Rhs::UInt32(uint32) => {
            let other_array = uint32.array.read().map_err(cannot_read)?;
            let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
            for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                unsafe {
                    result.push(
                        array.get_unchecked(index as usize)
                            < other_array.get_unchecked(other_index as usize),
                    );
                }
            }
        }
        UInt32Rhs::PyArrayU32(py_array) => {
            for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                unsafe {
                    result.push(array.get_unchecked(index as usize) < value);
                }
            }
        }
        UInt32Rhs::VecU32(vec) => {
            for (&index, value) in indices.iter().zip(vec) {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) < value);
                }
            }
        }
    }
    Ok(PyArray1::from_vec(py, result).into_py(py))
}

fn le(py: Python, lhs: &UInt32, rhs: UInt32Rhs) -> PyResult<Py<PyArray1<bool>>> {
    let array = lhs.array.read().map_err(cannot_write)?;
    let indices = lhs.indices.0.read().map_err(cannot_read)?;
    let mut result = Vec::with_capacity(indices.len());
    match rhs {
        UInt32Rhs::U32(other) => {
            for &index in indices.iter() {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) <= other);
                }
            }
        }
        UInt32Rhs::UInt32(uint32) => {
            let other_array = uint32.array.read().map_err(cannot_read)?;
            let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
            for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                unsafe {
                    result.push(
                        array.get_unchecked(index as usize)
                            <= other_array.get_unchecked(other_index as usize),
                    );
                }
            }
        }
        UInt32Rhs::PyArrayU32(py_array) => {
            for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                unsafe {
                    result.push(array.get_unchecked(index as usize) <= value);
                }
            }
        }
        UInt32Rhs::VecU32(vec) => {
            for (&index, value) in indices.iter().zip(vec) {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) <= value);
                }
            }
        }
    }
    Ok(PyArray1::from_vec(py, result).into_py(py))
}

fn gt(py: Python, lhs: &UInt32, rhs: UInt32Rhs) -> PyResult<Py<PyArray1<bool>>> {
    let array = lhs.array.read().map_err(cannot_write)?;
    let indices = lhs.indices.0.read().map_err(cannot_read)?;
    let mut result = Vec::with_capacity(indices.len());
    match rhs {
        UInt32Rhs::U32(other) => {
            for &index in indices.iter() {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) > other);
                }
            }
        }
        UInt32Rhs::UInt32(uint32) => {
            let other_array = uint32.array.read().map_err(cannot_read)?;
            let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
            for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                unsafe {
                    result.push(
                        array.get_unchecked(index as usize)
                            > other_array.get_unchecked(other_index as usize),
                    );
                }
            }
        }
        UInt32Rhs::PyArrayU32(py_array) => {
            for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                unsafe {
                    result.push(array.get_unchecked(index as usize) > value);
                }
            }
        }
        UInt32Rhs::VecU32(vec) => {
            for (&index, value) in indices.iter().zip(vec) {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) > value);
                }
            }
        }
    }
    Ok(PyArray1::from_vec(py, result).into_py(py))
}

fn ge(py: Python, lhs: &UInt32, rhs: UInt32Rhs) -> PyResult<Py<PyArray1<bool>>> {
    let array = lhs.array.read().map_err(cannot_write)?;
    let indices = lhs.indices.0.read().map_err(cannot_read)?;
    let mut result = Vec::with_capacity(indices.len());
    match rhs {
        UInt32Rhs::U32(other) => {
            for &index in indices.iter() {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) >= other);
                }
            }
        }
        UInt32Rhs::UInt32(uint32) => {
            let other_array = uint32.array.read().map_err(cannot_read)?;
            let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
            for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                unsafe {
                    result.push(
                        array.get_unchecked(index as usize)
                            >= other_array.get_unchecked(other_index as usize),
                    );
                }
            }
        }
        UInt32Rhs::PyArrayU32(py_array) => {
            for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                unsafe {
                    result.push(array.get_unchecked(index as usize) >= value);
                }
            }
        }
        UInt32Rhs::VecU32(vec) => {
            for (&index, value) in indices.iter().zip(vec) {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) >= value);
                }
            }
        }
    }
    Ok(PyArray1::from_vec(py, result).into_py(py))
}

fn eq(py: Python, lhs: &UInt32, rhs: UInt32Rhs) -> PyResult<Py<PyArray1<bool>>> {
    let array = lhs.array.read().map_err(cannot_write)?;
    let indices = lhs.indices.0.read().map_err(cannot_read)?;
    let mut result = Vec::with_capacity(indices.len());
    match rhs {
        UInt32Rhs::U32(other) => {
            for &index in indices.iter() {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) == other);
                }
            }
        }
        UInt32Rhs::UInt32(uint32) => {
            let other_array = uint32.array.read().map_err(cannot_read)?;
            let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
            for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                unsafe {
                    result.push(
                        array.get_unchecked(index as usize)
                            == other_array.get_unchecked(other_index as usize),
                    );
                }
            }
        }
        UInt32Rhs::PyArrayU32(py_array) => {
            for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                unsafe {
                    result.push(array.get_unchecked(index as usize) == value);
                }
            }
        }
        UInt32Rhs::VecU32(vec) => {
            for (&index, value) in indices.iter().zip(vec) {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) == value);
                }
            }
        }
    }
    Ok(PyArray1::from_vec(py, result).into_py(py))
}

fn ne(py: Python, lhs: &UInt32, rhs: UInt32Rhs) -> PyResult<Py<PyArray1<bool>>> {
    let array = lhs.array.read().map_err(cannot_write)?;
    let indices = lhs.indices.0.read().map_err(cannot_read)?;
    let mut result = Vec::with_capacity(indices.len());
    match rhs {
        UInt32Rhs::U32(other) => {
            for &index in indices.iter() {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) != other);
                }
            }
        }
        UInt32Rhs::UInt32(uint32) => {
            let other_array = uint32.array.read().map_err(cannot_read)?;
            let other_indices = uint32.indices.0.read().map_err(cannot_read)?;
            for (&index, &other_index) in indices.iter().zip(other_indices.iter()) {
                unsafe {
                    result.push(
                        array.get_unchecked(index as usize)
                            != other_array.get_unchecked(other_index as usize),
                    );
                }
            }
        }
        UInt32Rhs::PyArrayU32(py_array) => {
            for (&index, value) in indices.iter().zip(py_array.readonly().as_array()) {
                unsafe {
                    result.push(array.get_unchecked(index as usize) != value);
                }
            }
        }
        UInt32Rhs::VecU32(vec) => {
            for (&index, value) in indices.iter().zip(vec) {
                unsafe {
                    result.push(*array.get_unchecked(index as usize) != value);
                }
            }
        }
    }
    Ok(PyArray1::from_vec(py, result).into_py(py))
}
