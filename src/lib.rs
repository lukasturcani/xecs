use numpy::PyArray1;
use pyo3::{
    exceptions::{PyIndexError, PyRuntimeError},
    prelude::*,
    types::PySlice,
};
use std::sync::{Arc, RwLock};

struct Array<T>(Arc<RwLock<Vec<T>>>);

fn cannot_read<T>(_err: T) -> PyErr {
    PyRuntimeError::new_err("cannot read array")
}
fn cannot_write<T>(_err: T) -> PyErr {
    PyRuntimeError::new_err("cannot mutate array")
}
fn bad_index() -> PyErr {
    PyIndexError::new_err("index out of range")
}

impl<T> Array<T>
where
    T: numpy::Element,
{
    fn from_numpy(array: &PyArray1<T>) -> PyResult<Self> {
        Ok(Self(Arc::new(RwLock::new(array.to_vec()?))))
    }

    fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<T>>> {
        let vec = self.0.read().map_err(cannot_read)?;
        Ok(PyArray1::from_vec(py, vec.clone()).into_py(py))
    }

    fn view(&self) -> PyResult<ArrayView<T>> {
        Ok(ArrayView {
            array: Arc::clone(&self.0),
            indices: (0..self.0.read().map_err(cannot_read)?.len()).collect(),
        })
    }
}

struct ArrayView<T> {
    array: Arc<RwLock<Vec<T>>>,
    indices: Vec<usize>,
}

#[derive(FromPyObject)]
enum Key<'a> {
    Slice(&'a PySlice),
    Indices(Vec<usize>),
    Mask(Vec<bool>),
}

impl<T> ArrayView<T>
where
    T: numpy::Element + Copy,
{
    fn __getitem__(&self, key: Key) -> PyResult<Self> {
        let indices = match key {
            Key::Slice(slice) => {
                println!("slice");
                let mut new_indices = Vec::with_capacity(self.indices.len());
                let indices = slice.indices(self.indices.len() as i64)?;
                for index in (indices.start..indices.stop).step_by(indices.step as usize) {
                    new_indices.push(*unsafe { self.indices.get_unchecked(index as usize) })
                }
                new_indices
            }
            Key::Indices(indices) => {
                println!("indices");
                let mut new_indices = Vec::with_capacity(indices.len());
                for index in indices {
                    new_indices.push(*self.indices.get(index).ok_or_else(bad_index)?);
                }
                new_indices
            }
            Key::Mask(mask) => {
                let mut new_indices = Vec::with_capacity(self.indices.len());
                for (keep, &index) in mask.into_iter().zip(self.indices.iter()) {
                    if keep {
                        new_indices.push(index);
                    }
                }
                println!("{:#?}", new_indices);
                new_indices
            }
        };
        Ok(Self {
            array: Arc::clone(&self.array),
            indices,
        })
    }

    fn __setitem__(&mut self, key: Key, value: T) -> PyResult<()> {
        match key {
            Key::Slice(slice) => {
                let indices = slice.indices(self.indices.len() as i64)?;
                let mut array = self.array.write().map_err(cannot_write)?;
                for index in (indices.start..indices.stop).step_by(indices.step as usize) {
                    unsafe {
                        *array.get_unchecked_mut(*self.indices.get_unchecked(index as usize)) =
                            value;
                    };
                }
            }
            Key::Indices(indices) => {
                let mut array = self.array.write().map_err(cannot_write)?;
                for index in indices {
                    let array_index = *self.indices.get(index as usize).ok_or_else(bad_index)?;
                    unsafe {
                        *array.get_unchecked_mut(array_index) = value;
                    }
                }
            }
            Key::Mask(mask) => {
                let mut array = self.array.write().map_err(cannot_write)?;
                for (keep, &index) in mask.into_iter().zip(self.indices.iter()) {
                    if keep {
                        unsafe {
                            *array.get_unchecked_mut(*self.indices.get_unchecked(index as usize)) =
                                value;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn assign_value_at_indices<T: Copy>(array: &PyArray1<T>, indices: Vec<usize>, value: T) {
    let start = array.data();
    unsafe {
        for index in indices {
            let item_pointer = start.add(index);
            *item_pointer = value;
        }
    }
}

#[pyclass]
struct ArrayF64(Array<f64>);

#[pymethods]
impl ArrayF64 {
    #[staticmethod]
    fn from_numpy(array: &PyArray1<f64>) -> PyResult<Self> {
        Array::from_numpy(array).map(Self)
    }

    fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<f64>>> {
        self.0.numpy(py)
    }

    fn view(&self) -> PyResult<ArrayViewF64> {
        self.0.view().map(ArrayViewF64)
    }
}

#[pyclass]
struct ArrayViewF64(ArrayView<f64>);

#[pymethods]
impl ArrayViewF64 {
    fn __getitem__(&self, key: Key) -> PyResult<Self> {
        Ok(Self(self.0.__getitem__(key)?))
    }

    fn __setitem__(&mut self, key: Key, value: f64) -> PyResult<()> {
        self.0.__setitem__(key, value)
    }
}

#[pyfunction]
fn assign_value_at_indices_bool(array: &PyArray1<bool>, indices: Vec<usize>, value: bool) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_i8(array: &PyArray1<i8>, indices: Vec<usize>, value: i8) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_i16(array: &PyArray1<i16>, indices: Vec<usize>, value: i16) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_i32(array: &PyArray1<i32>, indices: Vec<usize>, value: i32) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_i64(array: &PyArray1<i64>, indices: Vec<usize>, value: i64) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_u8(array: &PyArray1<u8>, indices: Vec<usize>, value: u8) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_u16(array: &PyArray1<u16>, indices: Vec<usize>, value: u16) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_u32(array: &PyArray1<u32>, indices: Vec<usize>, value: u32) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_u64(array: &PyArray1<u64>, indices: Vec<usize>, value: u64) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_f32(array: &PyArray1<f32>, indices: Vec<usize>, value: f32) {
    assign_value_at_indices(array, indices, value)
}

#[pyfunction]
fn assign_value_at_indices_f64(array: &PyArray1<f64>, indices: Vec<usize>, value: f64) {
    assign_value_at_indices(array, indices, value)
}

/// A Python module implemented in Rust.
#[pymodule]
fn necs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ArrayF64>()?;
    m.add_class::<ArrayViewF64>()?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_bool, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_i8, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_i16, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_i32, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_i64, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_u8, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_u16, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_u32, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_u64, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_f32, m)?)?;
    m.add_function(wrap_pyfunction!(assign_value_at_indices_f64, m)?)?;
    Ok(())
}
