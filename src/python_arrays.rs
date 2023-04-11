use crate::array::Array;
use crate::array_view::{ArrayView, Key, Value};
use numpy::PyArray1;
use pyo3::prelude::*;

#[pyclass]
pub struct Float64Array(Array<f64>);

#[pymethods]
impl Float64Array {
    #[staticmethod]
    fn from_numpy(array: &PyArray1<f64>) -> PyResult<Self> {
        Array::from_numpy(array).map(Self)
    }

    fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<f64>>> {
        self.0.numpy(py)
    }

    fn view(&self) -> PyResult<Float64> {
        self.0.view().map(Float64)
    }
}

#[derive(FromPyObject)]
pub enum ValueF64<'a> {
    One(f64),
    Many(&'a PyArray1<f64>),
}

#[pyclass]
pub struct Float64(ArrayView<f64>);

#[pymethods]
impl Float64 {
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
