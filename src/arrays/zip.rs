macro_rules! py_array_mut {
    ($self_array:expr, $self_indices:expr, $other:expr, $type:ty, $($fn:tt)*) => {
        let f = $($fn)*;
        for (self_index, &other_value) in $self_indices
            .iter()
            .zip($other.readonly().as_array().iter())
        {
            let self_value = unsafe { $self_array.get_unchecked_mut(*self_index as usize) };
            f(self_index, self_value, &(other_value as $type))
        }
    };
}

pub(crate) use py_array_mut;

macro_rules! value {
    ($self_array:expr, $self_indices:expr, $other_value:expr, $self_type:ty, $($fn:tt)*) => {
        let mut f = $($fn)*;
        for self_index in $self_indices.iter() {
            let self_value = unsafe { $self_array.get_unchecked(*self_index as usize) };
            f(self_index, &(*self_value as $self_type), &$other_value)
        }
    }
}

pub(crate) use value;

macro_rules! value_mut {
    ($self_array:expr, $self_indices:expr, $other_value:expr, $($fn:tt)*) => {
        let f = $($fn)*;
        for self_index in $self_indices.iter() {
            let self_value = unsafe { $self_array.get_unchecked_mut(*self_index as usize) };
            f(self_index, self_value, &$other_value)
        }
    }
}

pub(crate) use value_mut;

macro_rules! array_mut {
    ($self_array:expr, $self_indices:expr, $other:expr, $type:ty, $($fn:tt)*) => {
        let f = $($fn)*;
        let other_array = $other.0.array.read().map_err(cannot_write)?;
        let other_indices = $other.0.indices.0.read().map_err(cannot_read)?;
        for (self_index, &other_index) in $self_indices.iter().zip(other_indices.iter()) {
            let self_value = unsafe { $self_array.get_unchecked_mut(*self_index as usize) };
            let other_value = unsafe { other_array.get_unchecked(other_index as usize) };
            f(self_index, self_value, &(*other_value as $type))
        }
    };
}

pub(crate) use array_mut;

macro_rules! float_rhs_mut {
    ($self:expr, $other:expr, $type: ty, $($fn:tt)*) => {
        let mut self_array = $self.array.write().map_err(cannot_write)?;
        let self_indices = $self.indices.0.read().map_err(cannot_read)?;
        match $other {
            $crate::arrays::float_rhs::FloatRhs::I64(other_value) => {
                $crate::arrays::zip::value_mut!(self_array, self_indices, other_value as $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::F64(other_value) => {
                $crate::arrays::zip::value_mut!(self_array, self_indices, other_value as $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::Float32(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::Float64(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::Int8(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::Int16(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::Int32(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::Int64(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::UInt8(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::UInt16(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::UInt32(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::UInt64(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::PyArrayF32(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::PyArrayF64(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::PyArrayI8(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::PyArrayI16(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::PyArrayI32(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::PyArrayI64(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::PyArrayU8(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::PyArrayU16(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::PyArrayU32(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::float_rhs::FloatRhs::PyArrayU64(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
        }
    };
}

pub(crate) use float_rhs_mut;

macro_rules! int_rhs_mut {
    ($self:expr, $other:expr, $type: ty, $($fn:tt)*) => {
        let mut self_array = $self.array.write().map_err(cannot_write)?;
        let self_indices = $self.indices.0.read().map_err(cannot_read)?;
        match $other {
            $crate::arrays::int_rhs::IntRhs::I64(other_value) => {
                $crate::arrays::zip::value_mut!(self_array, self_indices, other_value as $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::Int8(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::Int16(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::Int32(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::Int64(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::UInt8(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::UInt16(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::UInt32(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::UInt64(other_array) => {
                $crate::arrays::zip::array_mut!(self_array, self_indices, other_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::PyArrayI8(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::PyArrayI16(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::PyArrayI32(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::PyArrayI64(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::PyArrayU8(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::PyArrayU16(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::PyArrayU32(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
            $crate::arrays::int_rhs::IntRhs::PyArrayU64(py_array) => {
                $crate::arrays::zip::py_array_mut!(self_array, self_indices, py_array, $type, $($fn)*);
            }
        }
    };
}

pub(crate) use int_rhs_mut;

macro_rules! py_array {
    ($self_array:expr, $self_indices:expr, $other:expr, $self_type:ty, $other_type:ty, $($fn:tt)*) => {
        let mut f = $($fn)*;
        for (self_index, &other_value) in $self_indices
            .iter()
            .zip($other.readonly().as_array().iter())
        {
            let self_value = unsafe { $self_array.get_unchecked(*self_index as usize) };
            f(self_index, &(*self_value as $self_type), &(other_value as $other_type))
        }
    };
}

pub(crate) use py_array;

macro_rules! array {
    ($self_array:expr, $self_indices:expr, $other:expr, $self_type:ty, $other_type:ty, $($fn:tt)*) => {
        let mut f = $($fn)*;
        let other_array = $other.0.array.read().map_err(cannot_write)?;
        let other_indices = $other.0.indices.0.read().map_err(cannot_read)?;
        for (self_index, &other_index) in $self_indices.iter().zip(other_indices.iter()) {
            let self_value = unsafe { $self_array.get_unchecked(*self_index as usize) };
            let other_value = unsafe { other_array.get_unchecked(other_index as usize) };
            f(self_index, &(*self_value as $self_type), &(*other_value as $other_type))
        }
    };
}

pub(crate) use array;

macro_rules! float_match {
    ($self_array:expr, $self_indices:expr, $other:expr, $type:ty, $($fn:tt)*) => {
        match $other {
            FloatRhs::I64(other_value) => {
                $crate::arrays::zip::value!($self_array, $self_indices, other_value as $type, $type, $($fn)*);
            }
            FloatRhs::F64(other_value) => {
                $crate::arrays::zip::value!($self_array, $self_indices, other_value as $type, $type, $($fn)*);
            }
            FloatRhs::Float32(other_array) => {
                $crate::arrays::zip::array!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatRhs::Float64(other_array) => {
                $crate::arrays::zip::array!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatRhs::Int8(other_array) => {
                $crate::arrays::zip::array!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatRhs::Int16(other_array) => {
                $crate::arrays::zip::array!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatRhs::Int32(other_array) => {
                $crate::arrays::zip::array!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatRhs::Int64(other_array) => {
                $crate::arrays::zip::array!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatRhs::UInt8(other_array) => {
                $crate::arrays::zip::array!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatRhs::UInt16(other_array) => {
                $crate::arrays::zip::array!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatRhs::UInt32(other_array) => {
                $crate::arrays::zip::array!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatRhs::UInt64(other_array) => {
                $crate::arrays::zip::array!($self_array, $self_indices, other_array, $type, $type, $($fn)*);
            }
            FloatRhs::PyArrayF32(py_array) => {
                $crate::arrays::zip::py_array!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatRhs::PyArrayF64(py_array) => {
                $crate::arrays::zip::py_array!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatRhs::PyArrayI8(py_array) => {
                $crate::arrays::zip::py_array!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatRhs::PyArrayI16(py_array) => {
                $crate::arrays::zip::py_array!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatRhs::PyArrayI32(py_array) => {
                $crate::arrays::zip::py_array!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatRhs::PyArrayI64(py_array) => {
                $crate::arrays::zip::py_array!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatRhs::PyArrayU8(py_array) => {
                $crate::arrays::zip::py_array!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatRhs::PyArrayU16(py_array) => {
                $crate::arrays::zip::py_array!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatRhs::PyArrayU32(py_array) => {
                $crate::arrays::zip::py_array!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
            FloatRhs::PyArrayU64(py_array) => {
                $crate::arrays::zip::py_array!($self_array, $self_indices, py_array, $type, $type, $($fn)*);
            }
        }
    }
}

pub(crate) use float_match;
