use pyo3::{exceptions::PyRuntimeError, prelude::*, pyclass::CompareOp};
use std::time;

#[pyclass]
pub struct Instant(Option<time::Instant>);

#[pymethods]
impl Instant {
    #[staticmethod]
    fn now() -> Self {
        Self(Some(time::Instant::now()))
    }
    fn checked_duration_since(&self, earlier: &mut Self) -> PyResult<Duration> {
        let original_ealier = earlier.0.clone();
        let duration = self
            .0
            .map(|x| x.checked_duration_since(earlier.0.take().unwrap()))
            .flatten()
            .ok_or_else(|| PyRuntimeError::new_err("input was not earlier"))
            .map(|x| Duration(Some(x)));
        earlier.0 = original_ealier;
        duration
    }
    fn elapsed(&self) -> Duration {
        Duration(Some(self.0.map(|x| x.elapsed()).unwrap()))
    }
    fn checked_add(&self, duration: &mut Duration) -> PyResult<Self> {
        let original_duration = duration.0.clone();
        let instant = self
            .0
            .map(|x| x.checked_add(duration.0.take().unwrap()))
            .flatten()
            .ok_or_else(|| PyRuntimeError::new_err("overflow"))
            .map(|x| Self(Some(x)));
        duration.0 = original_duration;
        instant
    }
    fn checked_sub(&self, duration: &mut Duration) -> PyResult<Self> {
        let original_duration = duration.0.clone();
        let instant = self
            .0
            .map(|x| x.checked_sub(duration.0.take().unwrap()))
            .flatten()
            .ok_or_else(|| PyRuntimeError::new_err("overflow"))
            .map(|x| Self(Some(x)));
        duration.0 = original_duration;
        instant
    }
}

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
    fn checked_add(&mut self, rhs: &mut Self) -> PyResult<()> {
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
    fn checked_sub(&mut self, rhs: &mut Self) -> PyResult<()> {
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
    fn saturating_sub(&mut self, rhs: &mut Self) -> Self {
        let original_lhs = self.0.clone();
        let original_rhs = rhs.0.clone();
        let result = self.0.take().unwrap().saturating_sub(rhs.0.take().unwrap());
        self.0 = original_lhs;
        rhs.0 = original_rhs;
        Self(Some(result))
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
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Ge => self.0 >= other.0,
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
        }
    }
    fn __add__(&mut self, rhs: &mut Self) -> PyResult<Self> {
        let mut clone = Self(self.0.clone());
        clone.checked_add(rhs)?;
        Ok(clone)
    }
    fn __iadd__(&mut self, rhs: &mut Self) -> PyResult<()> {
        self.checked_add(rhs)
    }
    fn __sub__(&mut self, rhs: &mut Self) -> PyResult<Self> {
        let mut clone = Self(self.0.clone());
        clone.checked_sub(rhs)?;
        Ok(clone)
    }
    fn __isub__(&mut self, rhs: &mut Self) -> PyResult<()> {
        self.checked_sub(rhs)
    }
}

#[pyclass]
pub struct Time {
    delta: time::Duration,
    elapsed: time::Duration,
    last_update: Option<time::Instant>,
    startup: time::Instant,
}

#[pymethods]
impl Time {
    #[staticmethod]
    fn default() -> Self {
        Self {
            delta: time::Duration::ZERO,
            elapsed: time::Duration::ZERO,
            last_update: None,
            startup: time::Instant::now(),
        }
    }
    fn delta(&self) -> Duration {
        Duration(Some(self.delta))
    }
    fn update(&mut self) {
        let now = time::Instant::now();
        self.delta = now - self.last_update.unwrap_or(self.startup);
        self.last_update = Some(now);
        self.elapsed += self.delta;
    }
    fn elapsed(&self) -> Duration {
        Duration(Some(self.elapsed))
    }
}
