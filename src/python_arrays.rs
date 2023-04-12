use crate::array_view::{ArrayView, Key, Value};
use numpy::PyArray1;
use pyo3::prelude::*;

#[derive(FromPyObject)]
pub enum ValueF64<'a> {
    One(f64),
    Many(&'a PyArray1<f64>),
}

#[pyclass]
pub struct Float64(ArrayView<f64>);

#[pymethods]
impl Float64 {
    #[staticmethod]
    fn from_numpy(array: &PyArray1<f64>) -> PyResult<Self> {
        ArrayView::from_numpy(array).map(Self)
    }

    fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<f64>>> {
        self.0.numpy(py)
    }

    fn p_spawn(&mut self, num: usize) {
        self.0.p_spawn(num)
    }

    #[staticmethod]
    fn p_create_pool(size: usize) -> Float64 {
        Self(ArrayView::p_create_pool(size))
    }

    fn __getitem__(&self, key: Key) -> PyResult<Self> {
        self.0.__getitem__(key).map(Self)
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
