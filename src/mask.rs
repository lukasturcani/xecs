use pyo3::prelude::*;

#[pyclass]
pub struct Mask(Vec<bool>);

impl Mask {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &bool> + '_ {
        self.0.iter()
    }
}
