use std::sync::{Arc, RwLock};

use itertools::Itertools;
use pyo3::prelude::*;

use crate::{array_view_indices::ArrayViewIndices, error_handlers::cannot_read};

#[pyfunction]
pub fn combinations_2(
    indices: Vec<PyRef<ArrayViewIndices>>,
) -> PyResult<(Vec<ArrayViewIndices>, Vec<ArrayViewIndices>)> {
    if indices.len() == 0 {
        return Ok((Vec::new(), Vec::new()));
    }
    let mut read_indices = Vec::with_capacity(indices.len());
    for i in indices.iter() {
        read_indices.push(i.0.read().map_err(cannot_read)?);
    }
    let num_entities = read_indices.first().unwrap().len();
    let mut indices1 = Vec::with_capacity(indices.len());
    let mut indices2 = Vec::with_capacity(indices.len());
    let num_indices = (1..num_entities).sum();
    for _ in 0..indices.len() {
        indices1.push(Vec::with_capacity(num_indices));
        indices2.push(Vec::with_capacity(num_indices));
    }
    for x in (0..num_entities).combinations(2) {
        let i: usize = unsafe { *x.get_unchecked(0) };
        let j: usize = unsafe { *x.get_unchecked(1) };
        for (component_index, indices) in read_indices.iter().enumerate() {
            unsafe {
                let a = *indices.get_unchecked(i);
                let b = *indices.get_unchecked(j);
                indices1.get_unchecked_mut(component_index).push(a);
                indices2.get_unchecked_mut(component_index).push(b);
            }
        }
    }
    Ok((
        indices1.into_iter().map(vec_to_indices).collect(),
        indices2.into_iter().map(vec_to_indices).collect(),
    ))
}

fn vec_to_indices(vec: Vec<u32>) -> ArrayViewIndices {
    ArrayViewIndices(Arc::new(RwLock::new(vec)))
}
