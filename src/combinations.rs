use std::sync::{Arc, RwLock};

use itertools::Itertools;
use pyo3::prelude::*;

use crate::{array_view_indices::ArrayViewIndices, error_handlers::cannot_read};

#[pyfunction]
pub fn product_2(
    indices: Vec<PyRef<ArrayViewIndices>>,
) -> PyResult<(Vec<ArrayViewIndices>, Vec<ArrayViewIndices>)> {
    if indices.is_empty() {
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
    for (i, j) in (0..num_entities).cartesian_product(0..num_entities) {
        if i == j {
            continue;
        }
        for (component_index, indices) in read_indices.iter().enumerate() {
            unsafe {
                indices1
                    .get_unchecked_mut(component_index)
                    .push(*indices.get_unchecked(i));
                indices2
                    .get_unchecked_mut(component_index)
                    .push(*indices.get_unchecked(j));
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
