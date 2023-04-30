use crate::index::Index;
use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::types::PySlice;

#[derive(FromPyObject, Debug)]
pub enum GetItemKey<'a> {
    PyArrayMask(&'a PyArray1<bool>),
    VectorMask(Vec<bool>),
    Slice(&'a PySlice),
    PyArrayIndices(&'a PyArray1<Index>),
    VectorIndices(Vec<usize>),
}
