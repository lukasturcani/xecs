use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::types::PySlice;

#[derive(FromPyObject)]
pub enum GetItemKey<'a> {
    Slice(&'a PySlice),
    ArrayMask(&'a PyArray1<bool>),
}
