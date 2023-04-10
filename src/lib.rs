use itertools::izip;
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
            indices: (0..self.0.read().map_err(cannot_read)?.len() as u32).collect(),
        })
    }
}

struct ArrayView<T> {
    array: Arc<RwLock<Vec<T>>>,
    indices: Vec<u32>,
}

#[derive(FromPyObject)]
enum Key<'a> {
    Slice(&'a PySlice),
    ArrayIndices(&'a PyArray1<u32>),
    ArrayMask(&'a PyArray1<bool>),
}

enum Value<'a, T> {
    One(T),
    Many(&'a PyArray1<T>),
}

impl<T> ArrayView<T>
where
    T: numpy::Element + Copy,
{
    fn __getitem__(&self, key: Key) -> PyResult<Self> {
        let indices = match key {
            Key::Slice(slice) => {
                let mut new_indices = Vec::with_capacity(self.indices.len());
                let indices = slice.indices(self.indices.len() as i64)?;
                for index in (indices.start..indices.stop).step_by(indices.step as usize) {
                    new_indices.push(*unsafe { self.indices.get_unchecked(index as usize) })
                }
                new_indices
            }
            Key::ArrayIndices(indices) => {
                let mut new_indices = Vec::with_capacity(indices.len());
                for &index in indices.readonly().as_array() {
                    new_indices.push(*self.indices.get(index as usize).ok_or_else(bad_index)?);
                }
                new_indices
            }
            Key::ArrayMask(mask) => {
                let mut new_indices = Vec::with_capacity(self.indices.len());
                for (&keep, &index) in mask.readonly().as_array().iter().zip(self.indices.iter()) {
                    if keep {
                        new_indices.push(index);
                    }
                }
                new_indices
            }
        };
        Ok(Self {
            array: Arc::clone(&self.array),
            indices,
        })
    }

    fn __setitem__(&mut self, key: Key, value: Value<T>) -> PyResult<()> {
        match (key, value) {
            (Key::Slice(slice), Value::One(item)) => {
                let indices = slice.indices(self.indices.len() as i64)?;
                let mut array = self.array.write().map_err(cannot_write)?;
                for index in (indices.start..indices.stop).step_by(indices.step as usize) {
                    unsafe {
                        *array.get_unchecked_mut(
                            *self.indices.get_unchecked(index as usize) as usize
                        ) = item;
                    };
                }
            }
            (Key::ArrayIndices(indices), Value::One(item)) => {
                let mut array = self.array.write().map_err(cannot_write)?;
                for &index in indices.readonly().as_array() {
                    let array_index = *self.indices.get(index as usize).ok_or_else(bad_index)?;
                    unsafe {
                        *array.get_unchecked_mut(array_index as usize) = item;
                    }
                }
            }
            (Key::ArrayMask(mask), Value::One(item)) => {
                let mut array = self.array.write().map_err(cannot_write)?;
                for (&keep, &index) in mask.readonly().as_array().iter().zip(self.indices.iter()) {
                    if keep {
                        unsafe {
                            *array.get_unchecked_mut(
                                *self.indices.get_unchecked(index as usize) as usize
                            ) = item;
                        }
                    }
                }
            }
            (Key::Slice(slice), Value::Many(items)) => {
                let indices = slice.indices(self.indices.len() as i64)?;
                let mut array = self.array.write().map_err(cannot_write)?;
                for (index, &item) in (indices.start..indices.stop)
                    .step_by(indices.step as usize)
                    .zip(items.readonly().as_array())
                {
                    unsafe {
                        *array.get_unchecked_mut(
                            *self.indices.get_unchecked(index as usize) as usize
                        ) = item;
                    };
                }
            }
            (Key::ArrayIndices(indices), Value::Many(items)) => {
                let mut array = self.array.write().map_err(cannot_write)?;
                for (&index, &item) in indices
                    .readonly()
                    .as_array()
                    .iter()
                    .zip(items.readonly().as_array())
                {
                    let array_index = *self.indices.get(index as usize).ok_or_else(bad_index)?;
                    unsafe {
                        *array.get_unchecked_mut(array_index as usize) = item;
                    }
                }
            }
            (Key::ArrayMask(mask), Value::Many(items)) => {
                let mut array = self.array.write().map_err(cannot_write)?;
                for (&keep, &index, &item) in izip!(
                    mask.readonly().as_array().iter(),
                    self.indices.iter(),
                    items.readonly().as_array()
                ) {
                    if keep {
                        unsafe {
                            *array.get_unchecked_mut(
                                *self.indices.get_unchecked(index as usize) as usize
                            ) = item;
                        }
                    }
                }
            }
        }
        Ok(())
    }
    fn __len__(&self) -> usize {
        self.indices.len()
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

#[derive(FromPyObject)]
enum ValueF64<'a> {
    One(f64),
    Many(&'a PyArray1<f64>),
}

#[pyclass]
struct ArrayViewF64(ArrayView<f64>);

#[pymethods]
impl ArrayViewF64 {
    fn __getitem__(&self, key: Key) -> PyResult<Self> {
        Ok(Self(self.0.__getitem__(key)?))
    }

    fn __setitem__(&mut self, key: Key, value: ValueF64) -> PyResult<()> {
        match value {
            ValueF64::One(one) => self.0.__setitem__(key, Value::One(one)),
            ValueF64::Many(many) => self.0.__setitem__(key, Value::Many(many)),
        }
    }

    fn __len__(&self) -> usize {
        self.0.__len__()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn necs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ArrayF64>()?;
    m.add_class::<ArrayViewF64>()?;
    Ok(())
}
