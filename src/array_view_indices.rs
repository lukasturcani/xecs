use crate::index::Index;
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass]
pub struct MultipleArrayViewIndices(pub Vec<Arc<Vec<Index>>>);

#[pyclass]
pub struct ArrayViewIndices(pub Arc<Vec<Index>>);
