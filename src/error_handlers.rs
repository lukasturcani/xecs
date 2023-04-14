use pyo3::exceptions::{PyIndexError, PyRuntimeError};
use pyo3::prelude::*;

pub fn cannot_write<T>(_err: T) -> PyErr {
    PyRuntimeError::new_err("cannot mutate array")
}

pub fn cannot_read<T>(_err: T) -> PyErr {
    PyRuntimeError::new_err("cannot read array")
}

pub fn bad_index() -> PyErr {
    PyIndexError::new_err("index out of range")
}
