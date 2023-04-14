use crate::index::Index;
use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::types::PySlice;

#[derive(FromPyObject)]
pub enum Key<'a> {
    Slice(&'a PySlice),
    ArrayIndices(&'a PyArray1<Index>),
    ArrayMask(&'a PyArray1<bool>),
}
