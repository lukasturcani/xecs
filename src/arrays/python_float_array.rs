use crate::array_view_indices::ArrayViewIndices;
use crate::arrays::array::Array;
use crate::arrays::float_rhs::FloatRhs;
use crate::arrays::setitem;
use crate::arrays::zip;
use crate::error_handlers::{bad_index, cannot_read, cannot_write};
use crate::getitem_key::GetItemKey;
use numpy::PyArray1;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;

macro_rules! float_cmp {
    ($self:expr, $other:expr, $type:ty, $op:tt) => {
        {
            let self_array = $self.array.read().map_err(cannot_read)?;
            let self_indices = $self.indices.0.read().map_err(cannot_read)?;
            let indices = ArrayViewIndices::with_capacity(self_indices.len());
            {
                let mut out_indices = indices.0.write().map_err(cannot_write)?;
                zip::float_match!(
                    self_array,
                    self_indices,
                    $other,
                    $type,
                    |index: &u32, a: &$type, b: &$type| {
                        if a $op b {
                            out_indices.push(*index);
                        }
                    }
                );
            }
            Ok(indices)
        }

    }

}

macro_rules! float_iop {
    ($self:expr, $other:expr, $type:ty, $op:tt) => {
        {
            zip::float_rhs_mut!($self, $other, $type, |_, a: &mut $type, b: &$type| {
                *a $op b
            });
            Ok(())
        }
    }
}

macro_rules! float_array {
    (impl Array<$type:ty>) => {
        impl Array<$type> {
            pub fn __setitem__(&mut self, key: GetItemKey, value: FloatRhs) -> PyResult<()> {
                setitem::float!(self, key, value, $type);
                Ok(())
            }

            pub fn __iadd__(&mut self, other: FloatRhs) -> PyResult<()> {
                float_iop!(self, other, $type, +=)
            }

            pub fn __isub__(&mut self, other: FloatRhs) -> PyResult<()> {
                float_iop!(self, other, $type, -=)
            }

            pub fn __imul__(&mut self, other: FloatRhs) -> PyResult<()> {
                float_iop!(self, other, $type, *=)
            }
            pub fn __itruediv__(&mut self, other: FloatRhs) -> PyResult<()> {
                float_iop!(self, other, $type, /=)
            }
            pub fn __ifloordiv__(&mut self, other: FloatRhs) -> PyResult<()> {
                zip::float_rhs_mut!(self, other, $type, |_, a: &mut $type, b: &$type| {
                    *a = a.div_euclid(*b)
                });
                Ok(())
            }
            pub fn __imod__(&mut self, other: FloatRhs) -> PyResult<()> {
                float_iop!(self, other, $type, %=)
            }
            pub fn __ipow__(&mut self, other: FloatRhs) -> PyResult<()> {
                let mut self_array = self.array.write().map_err(cannot_write)?;
                let self_indices = self.indices.0.read().map_err(cannot_read)?;
                let powf = |_, a: &mut $type, b: &$type| {
                    *a = a.powf(*b)
                };
                let powi = |_, a: &mut $type, b: &i32| {
                    *a = a.powi(*b)
                };
                match other {
                    FloatRhs::I64(other_value) => {
                        zip::value_mut!(self_array, self_indices, other_value as i32, powi);
                    }
                    FloatRhs::F64(other_value) => {
                        zip::value_mut!(self_array, self_indices, other_value as $type, powf);
                    }
                    FloatRhs::Float32(other_array) => {
                        zip::array_mut!(self_array, self_indices, other_array, $type, powf);
                    }
                    FloatRhs::Float64(other_array) => {
                        zip::array_mut!(self_array, self_indices, other_array, $type, powf);
                    }
                    FloatRhs::Int8(other_array) => {
                        zip::array_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatRhs::Int16(other_array) => {
                        zip::array_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatRhs::Int32(other_array) => {
                        zip::array_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatRhs::Int64(other_array) => {
                        zip::array_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatRhs::UInt8(other_array) => {
                        zip::array_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatRhs::UInt16(other_array) => {
                        zip::array_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatRhs::UInt32(other_array) => {
                        zip::array_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatRhs::UInt64(other_array) => {
                        zip::array_mut!(self_array, self_indices, other_array, i32, powi);
                    }
                    FloatRhs::PyArrayF32(py_array) => {
                        zip::py_array_mut!(self_array, self_indices, py_array, $type, powf);
                    }
                    FloatRhs::PyArrayF64(py_array) => {
                        zip::py_array_mut!(self_array, self_indices, py_array, $type, powf);
                    }
                    FloatRhs::PyArrayI8(py_array) => {
                        zip::py_array_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatRhs::PyArrayI16(py_array) => {
                        zip::py_array_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatRhs::PyArrayI32(py_array) => {
                        zip::py_array_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatRhs::PyArrayI64(py_array) => {
                        zip::py_array_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatRhs::PyArrayU8(py_array) => {
                        zip::py_array_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatRhs::PyArrayU16(py_array) => {
                        zip::py_array_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatRhs::PyArrayU32(py_array) => {
                        zip::py_array_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                    FloatRhs::PyArrayU64(py_array) => {
                        zip::py_array_mut!(self_array, self_indices, py_array, i32, powi);
                    }
                }
                Ok(())
            }
            pub fn __lt__(&self, other: FloatRhs) -> PyResult<ArrayViewIndices> {
                float_cmp!(self, other, $type, <)
            }
            pub fn __le__(&self, other: FloatRhs) -> PyResult<ArrayViewIndices> {
                float_cmp!(self, other, $type, <=)
            }
            pub fn __gt__(&self, other: FloatRhs) -> PyResult<ArrayViewIndices> {
                float_cmp!(self, other, $type, >)
            }
            pub fn __ge__(&self, other: FloatRhs) -> PyResult<ArrayViewIndices> {
                float_cmp!(self, other, $type, >=)
            }
            pub fn __eq__(&self, other: FloatRhs) -> PyResult<ArrayViewIndices> {
                float_cmp!(self, other, $type, ==)
            }
            pub fn __ne__(&self, other: FloatRhs) -> PyResult<ArrayViewIndices> {
                float_cmp!(self, other, $type, !=)
            }
        }
    };
}

macro_rules! python_float_array {
    (pub struct $name:ident($type:ty)) => {
        #[pyclass]
        #[derive(Debug)]
        pub struct $name(pub Array<$type>);
        #[pymethods]
        impl $name {
            #[staticmethod]
            pub fn p_with_indices(indices: &ArrayViewIndices) -> PyResult<Self> {
                Array::p_with_indices(indices, 0.0).map(Self)
            }
            #[staticmethod]
            pub fn p_from_numpy(array: &PyArray1<$type>) -> PyResult<Self> {
                Array::p_from_numpy(array).map(Self)
            }
            pub fn p_new_view_with_indices(&self, indices: &ArrayViewIndices) -> Self {
                Self(self.0.p_new_view_with_indices(indices))
            }
            pub fn numpy(&self, py: Python) -> PyResult<Py<PyArray1<$type>>> {
                self.0.numpy(py)
            }
            pub fn __len__(&self) -> PyResult<usize> {
                self.0.__len__()
            }
            pub fn __getitem__(&self, key: GetItemKey) -> PyResult<Self> {
                self.0.__getitem__(key).map(Self)
            }
            pub fn __setitem__(&mut self, key: GetItemKey, value: FloatRhs) -> PyResult<()> {
                self.0.__setitem__(key, value)
            }
            pub fn __iadd__(&mut self, other: FloatRhs) -> PyResult<()> {
                self.0.__iadd__(other)
            }
            pub fn __isub__(&mut self, other: FloatRhs) -> PyResult<()> {
                self.0.__isub__(other)
            }
            pub fn __imul__(&mut self, other: FloatRhs) -> PyResult<()> {
                self.0.__imul__(other)
            }
            pub fn __itruediv__(&mut self, other: FloatRhs) -> PyResult<()> {
                self.0.__itruediv__(other)
            }
            pub fn __ifloordiv__(&mut self, other: FloatRhs) -> PyResult<()> {
                self.0.__ifloordiv__(other)
            }
            pub fn __imod__(&mut self, other: FloatRhs) -> PyResult<()> {
                self.0.__imod__(other)
            }
            #[args(modulo = "None")]
            pub fn __ipow__(&mut self, other: FloatRhs, _modulo: &PyAny) -> PyResult<()> {
                self.0.__ipow__(other)
            }
            pub fn __richcmp__(
                &mut self,
                other: FloatRhs,
                op: CompareOp,
            ) -> PyResult<ArrayViewIndices> {
                match op {
                    CompareOp::Lt => self.0.__lt__(other),
                    CompareOp::Le => self.0.__le__(other),
                    CompareOp::Gt => self.0.__gt__(other),
                    CompareOp::Ge => self.0.__ge__(other),
                    CompareOp::Eq => self.0.__eq__(other),
                    CompareOp::Ne => self.0.__ne__(other),
                }
            }
            pub fn __repr__(&self) -> PyResult<String> {
                Ok(format!("{}({:?})", stringify!($name), self.0.to_vec()?))
            }
            pub fn __str__(&self) -> PyResult<String> {
                Ok(format!("{}({:?})", stringify!($name), self.0.to_vec()?))
            }
        }
    };
}

float_array! { impl Array<f32> }
float_array! { impl Array<f64> }

python_float_array! {
    pub struct Float32(f32)
}

python_float_array! {
    pub struct Float64(f64)
}
