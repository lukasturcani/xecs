use crate::array_view_indices::ArrayViewIndices;
use crate::arrays::array::Array;
use crate::arrays::float_rhs::FloatRhs;
use crate::arrays::int_rhs::IntRhs;
use crate::arrays::zip;
use crate::error_handlers::{cannot_read, cannot_write};
use crate::getitem_key::GetItemKey;
use numpy::PyArray1;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;

macro_rules! int_cmp {
    ($self:expr, $other:expr, $type:ty, $op:tt) => {
        {
            let self_array = $self.array.read().map_err(cannot_read)?;
            let self_indices = $self.indices.0.read().map_err(cannot_read)?;
            let indices = ArrayViewIndices::with_capacity(self_indices.len());
            {
                let mut out_indices = indices.0.write().map_err(cannot_write)?;
                match $other {
                    FloatRhs::I64(other_value) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::value!(self_array, self_indices, other_value as $type, $type, func);
                    }
                    FloatRhs::F64(other_value) => {
                        let func_f64 = |index: &u32, a: &f64, b: &f64| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::value!(self_array, self_indices, other_value as f64, f64, func_f64);
                    }
                    FloatRhs::Float32(other_array) => {
                        let func_f32 = |index: &u32, a: &f32, b: &f32| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::array!(self_array, self_indices, other_array, f32, f32, func_f32);
                    }
                    FloatRhs::Float64(other_array) => {
                        let func_f64 = |index: &u32, a: &f64, b: &f64| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::array!(self_array, self_indices, other_array, f64, f64, func_f64);
                    }
                    FloatRhs::Int8(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::array!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatRhs::Int16(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::array!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatRhs::Int32(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::array!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatRhs::Int64(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::array!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatRhs::UInt8(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::array!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatRhs::UInt16(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::array!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatRhs::UInt32(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::array!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatRhs::UInt64(other_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::array!(self_array, self_indices, other_array, $type, $type, func);
                    }
                    FloatRhs::PyArrayF32(py_array) => {
                        let func_f32 = |index: &u32, a: &f32, b: &f32| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::py_array!(self_array, self_indices, py_array, f32, f32, func_f32);
                    }
                    FloatRhs::PyArrayF64(py_array) => {
                        let func_f64 = |index: &u32, a: &f64, b: &f64| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::py_array!(self_array, self_indices, py_array, f64, f64, func_f64);
                    }
                    FloatRhs::PyArrayI8(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::py_array!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatRhs::PyArrayI16(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::py_array!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatRhs::PyArrayI32(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::py_array!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatRhs::PyArrayI64(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::py_array!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatRhs::PyArrayU8(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::py_array!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatRhs::PyArrayU16(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::py_array!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatRhs::PyArrayU32(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::py_array!(self_array, self_indices, py_array, $type, $type, func);
                    }
                    FloatRhs::PyArrayU64(py_array) => {
                        let func = |index: &u32, a: &$type, b: &$type| {
                            if a $op b {
                                out_indices.push(*index);
                            }
                        };
                        zip::py_array!(self_array, self_indices, py_array, $type, $type, func);
                    }
                };
            }
            Ok(indices)
        }
    }
}

macro_rules! int_iop {
    ($self:expr, $other:expr, $type:ty, $op:tt) => {
        {
            zip::int_rhs_mut!($self, $other, $type, |_, a: &mut $type, b: &$type| {
                *a $op b
            });
            Ok(())
        }
    }
}

macro_rules! int_array {
    (impl Array<$type:ty>) => {
        impl Array<$type> {
            // pub fn __setitem__(&mut self, key: GetItemKey, value: IntOpRhsValue) -> PyResult<()> {
            //     int_binary_op!(self.array, key, value, $type, |_, b| b);
            //     Ok(())
            // }
            pub fn __iadd__(&mut self, other: IntRhs) -> PyResult<()> {
                int_iop!(self, other, $type, +=)
            }
            pub fn __isub__(&mut self, other: IntRhs) -> PyResult<()> {
                int_iop!(self, other, $type, -=)
            }
            pub fn __imul__(&mut self, other: IntRhs) -> PyResult<()> {
                int_iop!(self, other, $type, *=)
            }
            pub fn __itruediv__(&mut self, other: IntRhs) -> PyResult<()> {
                int_iop!(self, other, $type, /=)
            }
            pub fn __ifloordiv__(&mut self, other: IntRhs) -> PyResult<()> {
                zip::int_rhs_mut!(self, other, $type, |_, a: &mut $type, b: &$type| {
                    *a = a.div_euclid(*b)
                });
                Ok(())
            }
            pub fn __imod__(&mut self, other: IntRhs) -> PyResult<()> {
                int_iop!(self, other, $type, %=)
            }
            pub fn __ipow__(&mut self, other: IntRhs) -> PyResult<()> {
                zip::int_rhs_mut!(self, other, $type, |_, a: &mut $type, b: &$type| {
                    *a = a.pow(*b as u32)
                });
                Ok(())
            }
            pub fn __lt__(&self, other: FloatRhs) -> PyResult<ArrayViewIndices> {
                int_cmp!(self, other, $type, <)
            }
            pub fn __le__(&self, other: FloatRhs) -> PyResult<ArrayViewIndices> {
                int_cmp!(self, other, $type, <=)
            }
            pub fn __gt__(&self, other: FloatRhs) -> PyResult<ArrayViewIndices> {
                int_cmp!(self, other, $type, >)
            }
            pub fn __ge__(&self, other: FloatRhs) -> PyResult<ArrayViewIndices> {
                int_cmp!(self, other, $type, >=)
            }
            pub fn __eq__(&self, other: FloatRhs) -> PyResult<ArrayViewIndices> {
                int_cmp!(self, other, $type, ==)
            }
            pub fn __ne__(&self, other: FloatRhs) -> PyResult<ArrayViewIndices> {
                int_cmp!(self, other, $type, !=)
            }
        }
    };
}

macro_rules! python_int_array {
    (pub struct $name:ident($type:ty)) => {
        #[pyclass]
        pub struct $name(pub Array<$type>);
        #[pymethods]
        impl $name {
            #[staticmethod]
            pub fn p_with_indices(indices: &ArrayViewIndices) -> PyResult<Self> {
                Array::p_with_indices(indices, 0).map(Self)
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
            pub fn __getitem__(&self, key: GetItemKey) -> PyResult<Self> {
                self.0.__getitem__(key).map(Self)
            }
            // pub fn __setitem__(&mut self, key: GetItemKey, value: IntOpRhsValue) -> PyResult<()> {
            //     self.0.__setitem__(key, value)
            // }
            pub fn __len__(&self) -> PyResult<usize> {
                self.0.__len__()
            }
            pub fn __iadd__(&mut self, other: IntRhs) -> PyResult<()> {
                self.0.__iadd__(other)
            }
            pub fn __isub__(&mut self, other: IntRhs) -> PyResult<()> {
                self.0.__isub__(other)
            }
            pub fn __imul__(&mut self, other: IntRhs) -> PyResult<()> {
                self.0.__imul__(other)
            }
            pub fn __itruediv__(&mut self, other: IntRhs) -> PyResult<()> {
                self.0.__itruediv__(other)
            }
            pub fn __ifloordiv__(&mut self, other: IntRhs) -> PyResult<()> {
                self.0.__ifloordiv__(other)
            }
            pub fn __imod__(&mut self, other: IntRhs) -> PyResult<()> {
                self.0.__imod__(other)
            }
            #[args(modulo = "None")]
            pub fn __ipow__(&mut self, other: IntRhs, _modulo: &PyAny) -> PyResult<()> {
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
        }
    };
}

int_array! { impl Array<i8> }
int_array! { impl Array<i16> }
int_array! { impl Array<i32> }
int_array! { impl Array<i64> }
int_array! { impl Array<u8> }
int_array! { impl Array<u16> }
int_array! { impl Array<u32> }
int_array! { impl Array<u64> }

python_int_array! {
    pub struct Int8(i8)
}

python_int_array! {
    pub struct Int16(i16)
}

python_int_array! {
    pub struct Int32(i32)
}

python_int_array! {
    pub struct Int64(i64)
}

python_int_array! {
    pub struct UInt8(u8)
}

python_int_array! {
    pub struct UInt16(u16)
}

python_int_array! {
    pub struct UInt32(u32)
}

python_int_array! {
    pub struct UInt64(u64)
}
