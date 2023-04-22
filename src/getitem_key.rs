use crate::index::Index;
use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::types::PySlice;

#[derive(FromPyObject)]
pub enum GetItemKey<'a> {
    Slice(&'a PySlice),
    PyArrayIndices(&'a PyArray1<Index>),
    PyArrayMask(&'a PyArray1<bool>),
    VectorIndices(Vec<usize>),
    VectorMask(Vec<bool>),
}
