use crate::index::Index;
use pyo3::prelude::*;
use std::sync::{Arc, RwLock};

#[pyclass]
pub struct MultipleArrayViewIndices(pub Vec<Arc<RwLock<Vec<Index>>>>);

#[pyclass]
pub struct ArrayViewIndices(pub Arc<RwLock<Vec<Index>>>);

#[pymethods]
impl ArrayViewIndices {
    #[staticmethod]
    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(RwLock::new(Vec::with_capacity(capacity))))
    }
}
