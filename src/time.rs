use pyo3::prelude::*;
use std::time;

#[pyclass]
struct Duration(time::Duration);

#[pymethods]
impl Duration {
    #[staticmethod]
    fn new(secs: u64, nanos: u32) -> Self {
        Self(time::Duration::new(secs, nanos))
    }
    #[staticmethod]
    fn from_millis(millis: u64) -> Self {
        Self(time::Duration::from_millis(millis))
    }
    #[staticmethod]
    fn from_micros(micros: u64) -> Self {
        Self(time::Duration::from_micros(micros))
    }
    #[staticmethod]
    fn from_nanos(nanos: u64) -> Self {
        Self(time::Duration::from_nanos(nanos))
    }
    fn is_zeros(&self) -> bool {
        self.0.is_zero()
    }
    fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }
    fn subsec_millis(&self) -> u32 {
        self.0.subsec_millis()
    }
    fn subsec_micros(&self) -> u32 {
        self.0.subsec_micros()
    }
    fn subsec_nanos(&self) -> u32 {
        self.0.subsec_nanos()
    }
    fn as_millis(&self) -> u128 {
        self.0.as_millis()
    }
    fn as_micros(&self) -> u128 {
        self.0.as_micros()
    }
    fn as_nanos(&self) -> u128 {
        self.0.as_nanos()
    }
    fn checked_add(self, rhs: Duration) {
        self.0.checked_add(rhs)
    }
}
