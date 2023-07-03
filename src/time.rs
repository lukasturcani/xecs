use pyo3::{exceptions::PyRuntimeError, prelude::*};
use std::time;

#[pyclass]
pub struct Duration(Option<time::Duration>);

#[pymethods]
impl Duration {
    #[staticmethod]
    fn new(secs: u64, nanos: u32) -> Self {
        Self(Some(time::Duration::new(secs, nanos)))
    }
    #[staticmethod]
    fn from_millis(millis: u64) -> Self {
        Self(Some(time::Duration::from_millis(millis)))
    }
    #[staticmethod]
    fn from_micros(micros: u64) -> Self {
        Self(Some(time::Duration::from_micros(micros)))
    }
    #[staticmethod]
    fn from_nanos(nanos: u64) -> Self {
        Self(Some(time::Duration::from_nanos(nanos)))
    }
    fn is_zero(&self) -> bool {
        self.0.map(|x| x.is_zero()).unwrap()
    }
    fn as_secs(&self) -> u64 {
        self.0.map(|x| x.as_secs()).unwrap()
    }
    fn subsec_micros(&self) -> u32 {
        self.0.map(|x| x.subsec_micros()).unwrap()
    }
    fn subsec_nanos(&self) -> u32 {
        self.0.map(|x| x.subsec_nanos()).unwrap()
    }
    fn as_millis(&self) -> u128 {
        self.0.map(|x| x.as_millis()).unwrap()
    }
    fn as_micros(&self) -> u128 {
        self.0.map(|x| x.as_micros()).unwrap()
    }
    fn as_nanos(&self) -> u128 {
        self.0.map(|x| x.as_nanos()).unwrap()
    }
    fn checked_add(&mut self, rhs: &mut Duration) -> PyResult<()> {
        let original_lhs = self.0.clone();
        let original_rhs = rhs.0.clone();
        if let duration @ Some(_) = self.0.take().unwrap().checked_add(rhs.0.take().unwrap()) {
            self.0 = duration;
            rhs.0 = original_rhs;
            Ok(())
        } else {
            self.0 = original_lhs;
            rhs.0 = original_rhs;
            Err(PyRuntimeError::new_err("overflow"))
        }
    }
    fn checked_sub(&mut self, rhs: &mut Duration) -> PyResult<()> {
        let original_lhs = self.0.clone();
        let original_rhs = rhs.0.clone();
        if let duration @ Some(_) = self.0.take().unwrap().checked_sub(rhs.0.take().unwrap()) {
            self.0 = duration;
            rhs.0 = original_rhs;
            Ok(())
        } else {
            self.0 = original_lhs;
            rhs.0 = original_rhs;
            Err(PyRuntimeError::new_err("overflow"))
        }
    }
    fn checked_mul(&mut self, rhs: u32) -> PyResult<()> {
        let original_lhs = self.0.clone();
        if let duration @ Some(_) = self.0.take().unwrap().checked_mul(rhs) {
            self.0 = duration;
            Ok(())
        } else {
            self.0 = original_lhs;
            Err(PyRuntimeError::new_err("overflow"))
        }
    }
    fn checked_div(&mut self, rhs: u32) -> PyResult<()> {
        let original_lhs = self.0.clone();
        if let duration @ Some(_) = self.0.take().unwrap().checked_div(rhs) {
            self.0 = duration;
            Ok(())
        } else {
            self.0 = original_lhs;
            Err(PyRuntimeError::new_err("overflow"))
        }
    }
}
