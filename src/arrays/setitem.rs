use std::sync::{Arc, RwLock};

pub fn same_array<T, U>(a: &Arc<RwLock<Vec<T>>>, b: &Arc<RwLock<Vec<U>>>) -> bool {
    let alpha = unsafe {
        std::mem::transmute::<*const std::sync::RwLock<Vec<T>>, *const ()>(Arc::as_ptr(a))
    };
    let beta = unsafe {
        std::mem::transmute::<*const std::sync::RwLock<Vec<U>>, *const ()>(Arc::as_ptr(b))
    };
    alpha == beta
}

macro_rules! slice_value {
    ($self:expr, $slice:expr, $rhs:expr, $type:ty) => {
        let mut array = $self.array.write().map_err(cannot_write)?;
        let indices = $self.indices.0.read().map_err(cannot_read)?;
        let slice_indices = $slice.indices(indices.len() as i64)?;
        for index in (slice_indices.start..slice_indices.stop).step_by(slice_indices.step as usize)
        {
            let array_index = unsafe { indices.get_unchecked(index as usize) };
            let value = unsafe { array.get_unchecked_mut(*array_index as usize) };
            *value = $rhs as $type;
        }
    };
}

pub(crate) use slice_value;

macro_rules! indices_value {
    ($self:expr, $array_indices:expr, $rhs:expr, $type:ty) => {
        let mut array = $self.array.write().map_err(cannot_write)?;
        let indices = $self.indices.0.read().map_err(cannot_read)?;
        for &index in $array_indices {
            let array_index = indices.get(index as usize).ok_or_else(bad_index)?;
            let value = unsafe { array.get_unchecked_mut(*array_index as usize) };
            *value = $rhs as $type;
        }
    };
}

pub(crate) use indices_value;

macro_rules! mask_value {
    ($self:expr, $mask:expr, $rhs:expr, $type:ty) => {
        let mut array = $self.array.write().map_err(cannot_write)?;
        let indices = $self.indices.0.read().map_err(cannot_read)?;
        for (&write, &index) in $mask.iter().zip(indices.iter()) {
            if write {
                let value = unsafe { array.get_unchecked_mut(index as usize) };
                *value = $rhs as $type;
            }
        }
    };
}

pub(crate) use mask_value;

macro_rules! slice_array_inner {
    ($array:expr, $indices:expr, $slice:expr, $other_array:expr, $other_indices:expr, $type:ty) => {
        let slice_indices = $slice.indices($indices.len() as i64)?;
        for (index, &other_index) in (slice_indices.start..slice_indices.stop)
            .step_by(slice_indices.step as usize)
            .zip($other_indices.iter())
        {
            unsafe {
                let array_index = $indices.get_unchecked(index as usize);
                let self_value = $array.get_unchecked_mut(*array_index as usize) as *mut $type;
                let other_value = $other_array.get_unchecked(other_index as usize);
                *self_value = *other_value as $type;
            }
        }
    };
}

pub(crate) use slice_array_inner;

macro_rules! slice_array {
    ($self:expr, $slice:expr, $rhs:expr, $type:ty) => {
        let mut array = $self.array.write().map_err(cannot_write)?;
        let indices = $self.indices.0.read().map_err(cannot_read)?;
        if $crate::arrays::setitem::same_array(&$self.array, &$rhs.0.array) {
            $crate::arrays::setitem::slice_array_inner!(
                array, indices, $slice, array, indices, $type
            );
        } else {
            let other_array = $rhs.0.array.read().map_err(cannot_read)?;
            let other_indices = $rhs.0.indices.0.read().map_err(cannot_read)?;
            $crate::arrays::setitem::slice_array_inner!(
                array,
                indices,
                $slice,
                other_array,
                other_indices,
                $type
            );
        }
    };
}

pub(crate) use slice_array;

macro_rules! indices_array_inner {
    ($array:expr, $indices:expr, $array_indices:expr, $other_array:expr, $other_indices:expr, $type:ty) => {
        for (&index, &other_index) in $array_indices.iter().zip($other_indices.iter()) {
            unsafe {
                let array_index = $indices.get(index as usize).ok_or_else(bad_index)?;
                let self_value = $array.get_unchecked_mut(*array_index as usize) as *mut $type;
                let other_value = $other_array.get_unchecked(other_index as usize);
                *self_value = *other_value as $type;
            }
        }
    };
}

pub(crate) use indices_array_inner;

macro_rules! indices_array {
    ($self:expr, $array_indices:expr, $rhs:expr, $type:ty) => {
        let mut array = $self.array.write().map_err(cannot_write)?;
        let indices = $self.indices.0.read().map_err(cannot_read)?;
        if $crate::arrays::setitem::same_array(&$self.array, &$rhs.0.array) {
            $crate::arrays::setitem::indices_array_inner!(
                array,
                indices,
                $array_indices,
                array,
                indices,
                $type
            );
        } else {
            let other_array = $rhs.0.array.read().map_err(cannot_read)?;
            let other_indices = $rhs.0.indices.0.read().map_err(cannot_read)?;
            $crate::arrays::setitem::indices_array_inner!(
                array,
                indices,
                $array_indices,
                other_array,
                other_indices,
                $type
            );
        }
    };
}

pub(crate) use indices_array;

macro_rules! mask_array {
    ($self:expr, $mask:expr, $rhs:expr, $type:ty) => {
        let mut array = $self.array.write().map_err(cannot_write)?;
        let indices = $self.indices.0.read().map_err(cannot_read)?;
        let other_array = $rhs.0.array.read().map_err(cannot_read)?;
        let other_indices = $rhs.0.indices.0.read().map_err(cannot_read)?;
        let mut other_indices = other_indices.iter();
        for (&write, &self_index) in $mask.iter().zip(indices.iter()) {
            if write {
                let self_value = unsafe { array.get_unchecked_mut(self_index as usize) };
                let other_value =
                    unsafe { other_array.get_unchecked(*other_indices.next().unwrap() as usize) };
                *self_value = *other_value as $type;
            }
        }
    };
}

pub(crate) use mask_array;

macro_rules! slice_py_array {
    ($self:expr, $slice:expr, $rhs:expr, $type:ty) => {
        let mut array = $self.array.write().map_err(cannot_write)?;
        let indices = $self.indices.0.read().map_err(cannot_read)?;
        let slice_indices = $slice.indices(indices.len() as i64)?;
        for (index, &other_value) in (slice_indices.start..slice_indices.stop)
            .step_by(slice_indices.step as usize)
            .zip($rhs.readonly().as_array().iter())
        {
            let array_index = unsafe { indices.get_unchecked(index as usize) };
            let self_value = unsafe { array.get_unchecked_mut(*array_index as usize) };
            *self_value = other_value as $type;
        }
    };
}

pub(crate) use slice_py_array;

macro_rules! indices_py_array {
    ($self:expr, $array_indices:expr, $rhs:expr, $type:ty) => {
        let mut array = $self.array.write().map_err(cannot_write)?;
        let indices = $self.indices.0.read().map_err(cannot_read)?;
        for (&index, &other_value) in $array_indices.iter().zip($rhs.readonly().as_array().iter()) {
            let array_index = indices.get(index as usize).ok_or_else(bad_index)?;
            let self_value = unsafe { array.get_unchecked_mut(*array_index as usize) };
            *self_value = other_value as $type;
        }
    };
}

pub(crate) use indices_py_array;

macro_rules! mask_py_array {
    ($self:expr, $mask:expr, $rhs:expr, $type:ty) => {
        let mut array = $self.array.write().map_err(cannot_write)?;
        let indices = $self.indices.0.read().map_err(cannot_read)?;
        let other_values = $rhs.readonly();
        let other_values = other_values.as_array();
        let mut other_values = other_values.iter();
        for (&write, &self_index) in $mask.iter().zip(indices.iter()) {
            if write {
                let self_value = unsafe { array.get_unchecked_mut(self_index as usize) };
                let other_value = other_values.next().unwrap();
                *self_value = *other_value as $type;
            }
        }
    };
}

pub(crate) use mask_py_array;

macro_rules! float {
    ($self:expr, $key:expr, $value:expr, $type:ty) => {
        match ($key, $value) {
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::I64(rhs),
            ) => {
                $crate::arrays::setitem::slice_value!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::I64(rhs),
            ) => {
                $crate::arrays::setitem::indices_value!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::I64(rhs),
            ) => {
                $crate::arrays::setitem::mask_value!($self, mask.readonly().as_array(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::I64(rhs),
            ) => {
                $crate::arrays::setitem::indices_value!($self, vector_indices.iter(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::I64(rhs),
            ) => {
                $crate::arrays::setitem::mask_value!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::F64(rhs),
            ) => {
                $crate::arrays::setitem::slice_value!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::F64(rhs),
            ) => {
                $crate::arrays::setitem::indices_value!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::F64(rhs),
            ) => {
                $crate::arrays::setitem::mask_value!($self, mask.readonly().as_array(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::F64(rhs),
            ) => {
                $crate::arrays::setitem::indices_value!($self, vector_indices.iter(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::F64(rhs),
            ) => {
                $crate::arrays::setitem::mask_value!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::Float32(rhs),
            ) => {
                $crate::arrays::setitem::slice_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::Float32(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::Float32(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask.readonly().as_array(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::Float32(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::Float32(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::Float64(rhs),
            ) => {
                $crate::arrays::setitem::slice_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::Float64(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::Float64(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask.readonly().as_array(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::Float64(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::Float64(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::Int8(rhs),
            ) => {
                $crate::arrays::setitem::slice_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::Int8(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::Int8(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask.readonly().as_array(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::Int8(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::Int8(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::Int16(rhs),
            ) => {
                $crate::arrays::setitem::slice_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::Int16(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::Int16(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask.readonly().as_array(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::Int16(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::Int16(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::Int32(rhs),
            ) => {
                $crate::arrays::setitem::slice_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::Int32(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::Int32(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask.readonly().as_array(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::Int32(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::Int32(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::Int64(rhs),
            ) => {
                $crate::arrays::setitem::slice_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::Int64(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::Int64(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask.readonly().as_array(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::Int64(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::Int64(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::UInt8(rhs),
            ) => {
                $crate::arrays::setitem::slice_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::UInt8(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::UInt8(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask.readonly().as_array(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::UInt8(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::UInt8(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::UInt16(rhs),
            ) => {
                $crate::arrays::setitem::slice_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::UInt16(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::UInt16(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask.readonly().as_array(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::UInt16(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::UInt16(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::UInt32(rhs),
            ) => {
                $crate::arrays::setitem::slice_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::UInt32(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::UInt32(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask.readonly().as_array(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::UInt32(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::UInt32(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::UInt64(rhs),
            ) => {
                $crate::arrays::setitem::slice_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::UInt64(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::UInt64(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask.readonly().as_array(), rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::UInt64(rhs),
            ) => {
                $crate::arrays::setitem::indices_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::UInt64(rhs),
            ) => {
                $crate::arrays::setitem::mask_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::PyArrayF32(rhs),
            ) => {
                $crate::arrays::setitem::slice_py_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayF32(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayF32(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!(
                    $self,
                    mask.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayF32(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayF32(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::PyArrayF64(rhs),
            ) => {
                $crate::arrays::setitem::slice_py_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayF64(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayF64(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!(
                    $self,
                    mask.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayF64(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayF64(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI8(rhs),
            ) => {
                $crate::arrays::setitem::slice_py_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI8(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI8(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!(
                    $self,
                    mask.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI8(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI8(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI16(rhs),
            ) => {
                $crate::arrays::setitem::slice_py_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI16(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI16(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!(
                    $self,
                    mask.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI16(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI16(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI32(rhs),
            ) => {
                $crate::arrays::setitem::slice_py_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI32(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI32(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!(
                    $self,
                    mask.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI32(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI32(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI64(rhs),
            ) => {
                $crate::arrays::setitem::slice_py_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI64(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI64(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!(
                    $self,
                    mask.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI64(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayI64(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU8(rhs),
            ) => {
                $crate::arrays::setitem::slice_py_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU8(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU8(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!(
                    $self,
                    mask.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU8(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU8(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU16(rhs),
            ) => {
                $crate::arrays::setitem::slice_py_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU16(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU16(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!(
                    $self,
                    mask.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU16(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU16(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU32(rhs),
            ) => {
                $crate::arrays::setitem::slice_py_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU32(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU32(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!(
                    $self,
                    mask.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU32(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU32(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!($self, mask, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::Slice(slice),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU64(rhs),
            ) => {
                $crate::arrays::setitem::slice_py_array!($self, slice, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayIndices(array_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU64(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!(
                    $self,
                    array_indices.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::PyArrayMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU64(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!(
                    $self,
                    mask.readonly().as_array(),
                    rhs,
                    $type
                );
            }
            (
                $crate::getitem_key::GetItemKey::VectorIndices(vector_indices),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU64(rhs),
            ) => {
                $crate::arrays::setitem::indices_py_array!($self, vector_indices, rhs, $type);
            }
            (
                $crate::getitem_key::GetItemKey::VectorMask(mask),
                $crate::arrays::float_rhs::FloatRhs::PyArrayU64(rhs),
            ) => {
                $crate::arrays::setitem::mask_py_array!($self, mask, rhs, $type);
            }
        }
    };
}

pub(super) use float;
