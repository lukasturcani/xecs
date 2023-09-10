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
        let original_ealier = earlier.0;
        let duration = self
            .0
            .and_then(|x| x.checked_duration_since(earlier.0.take().unwrap()))
            .ok_or_else(|| PyRuntimeError::new_err("input was not earlier"))
            .map(|x| Duration(Some(x)));
        earlier.0 = original_ealier;
        duration
    }
    fn elapsed(&self) -> Duration {
        Duration(Some(self.0.map(|x| x.elapsed()).unwrap()))
    }
    fn checked_add(&self, duration: &mut Duration) -> PyResult<Self> {
        let original_duration = duration.0;
        let instant = self
            .0
            .and_then(|x| x.checked_add(duration.0.take().unwrap()))
            .ok_or_else(|| PyRuntimeError::new_err("overflow"))
            .map(|x| Self(Some(x)));
        duration.0 = original_duration;
        instant
    }
    fn checked_sub(&self, duration: &mut Duration) -> PyResult<Self> {
        let original_duration = duration.0;
        let instant = self
            .0
            .and_then(|x| x.checked_sub(duration.0.take().unwrap()))
            .ok_or_else(|| PyRuntimeError::new_err("overflow"))
            .map(|x| Self(Some(x)));
        duration.0 = original_duration;
        instant
    }
}

/// Represents a span of time.
#[pyclass(module = "xecs")]
pub struct Duration(Option<time::Duration>);

#[pymethods]
impl Duration {
    /// Create a new duration.
    ///
    /// Parameters:
    ///     secs (int): The number of whole seconds.
    ///     nanos (int): The number of additional nanoseconds.
    /// Returns:
    ///     Duration: The duration.
    #[staticmethod]
    fn new(secs: u64, nanos: u32) -> Self {
        Self(Some(time::Duration::new(secs, nanos)))
    }
    /// Create a new duration from a specified number of milliseconds.
    ///
    /// Parameters:
    ///     millis (int): The number of milliseconds.
    /// Returns:
    ///     Duration: The duration.
    #[staticmethod]
    fn from_millis(millis: u64) -> Self {
        Self(Some(time::Duration::from_millis(millis)))
    }
    /// Create a new duration from a specified number of microseconds.
    ///
    /// Parameters:
    ///     micros (int): The number of microseconds.
    /// Returns:
    ///     Duration: The duration.
    #[staticmethod]
    fn from_micros(micros: u64) -> Self {
        Self(Some(time::Duration::from_micros(micros)))
    }
    /// Create a new duration from a specified number of nanoseconds.
    ///
    /// Parameters:
    ///     nanos (int): The number of nanoseconds.
    /// Returns:
    ///     Duration: The duration.
    #[staticmethod]
    fn from_nanos(nanos: u64) -> Self {
        Self(Some(time::Duration::from_nanos(nanos)))
    }
    /// Return ``True`` if the duration spans no time.
    ///
    /// Returns:
    ///     bool: Whether the duration spans any time or not.
    fn is_zero(&self) -> bool {
        self.0.map(|x| x.is_zero()).unwrap()
    }
    /// Return the total number of whole seconds in the duration.
    ///
    /// Returns:
    ///     int: The total number of whole seconds.
    fn as_secs(&self) -> u64 {
        self.0.map(|x| x.as_secs()).unwrap()
    }
    /// Return the fractional part of this duration, in whole microseconds.
    ///
    /// This method does **not** return the lenght of the duration when
    /// represented by microseconds. The returned number always represents
    /// a fractional portion of a second.
    ///
    /// Returns:
    ///     int: The subsecond microseconds in the duration.
    fn subsec_micros(&self) -> u32 {
        self.0.map(|x| x.subsec_micros()).unwrap()
    }
    /// Return the fractional part of this duration, in whole nanoseconds.
    ///
    /// This method does **not** return the lenght of the duration when
    /// represented by nanoseconds. The returned number always represents
    /// a fractional portion of a second.
    ///
    /// Returns:
    ///     int: The subsecond nanoseconds in the duration.
    fn subsec_nanos(&self) -> u32 {
        self.0.map(|x| x.subsec_nanos()).unwrap()
    }
    /// Return the total number of whole milliseconds in this duration.
    ///
    /// Returns:
    ///     int: The number of whole milliseconds.
    fn as_millis(&self) -> u128 {
        self.0.map(|x| x.as_millis()).unwrap()
    }
    /// Return the total number of whole microseconds in this duration.
    ///
    /// Returns:
    ///     int: The number of whole microseconds.
    fn as_micros(&self) -> u128 {
        self.0.map(|x| x.as_micros()).unwrap()
    }
    /// Return the total number of whole nanoseconds in this duration.
    ///
    /// Returns:
    ///     int: The number of whole nanoseconds.
    fn as_nanos(&self) -> u128 {
        self.0.map(|x| x.as_nanos()).unwrap()
    }
    /// Add a duration inplace.
    ///
    /// Parameters:
    ///     rhs (Duration): The other duration.
    fn checked_add(&mut self, rhs: &mut Self) -> PyResult<()> {
        let original_lhs = self.0;
        let original_rhs = rhs.0;
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
    /// Subtract a duration inplace.
    ///
    /// Parameters:
    ///     rhs (Duration): The other duration.
    fn checked_sub(&mut self, rhs: &mut Self) -> PyResult<()> {
        let original_lhs = self.0;
        let original_rhs = rhs.0;
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
    /// Subtract a duration.
    ///
    /// Parameters:
    ///     rhs (Duration): The other duration.
    /// Returns:
    ///     Duration: The new duration. Zero if the result would
    ///     have been negative
    fn saturating_sub(&mut self, rhs: &mut Self) -> Self {
        let original_lhs = self.0;
        let original_rhs = rhs.0;
        let result = self.0.take().unwrap().saturating_sub(rhs.0.take().unwrap());
        self.0 = original_lhs;
        rhs.0 = original_rhs;
        Self(Some(result))
    }
    /// Multiply a duration inplace.
    ///
    /// Parameters:
    ///     rhs (Duration): The other duration.
    fn checked_mul(&mut self, rhs: u32) -> PyResult<()> {
        let original_lhs = self.0;
        if let duration @ Some(_) = self.0.take().unwrap().checked_mul(rhs) {
            self.0 = duration;
            Ok(())
        } else {
            self.0 = original_lhs;
            Err(PyRuntimeError::new_err("overflow"))
        }
    }
    /// Divide a duration inplace.
    ///
    /// Parameters:
    ///     rhs (Duration): The other duration.
    fn checked_div(&mut self, rhs: u32) -> PyResult<()> {
        let original_lhs = self.0;
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
        let mut clone = Self(self.0);
        clone.checked_add(rhs)?;
        Ok(clone)
    }
    fn __iadd__(&mut self, rhs: &mut Self) -> PyResult<()> {
        self.checked_add(rhs)
    }
    fn __sub__(&mut self, rhs: &mut Self) -> PyResult<Self> {
        let mut clone = Self(self.0);
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
    fn update_with_delta(&mut self, delta: &Duration) {
        self.delta = delta.0.unwrap();
        self.last_update = Some(self.last_update.unwrap_or(self.startup) + self.delta);
        self.elapsed += self.delta;
    }
    fn elapsed(&self) -> Duration {
        Duration(Some(self.elapsed))
    }
}
