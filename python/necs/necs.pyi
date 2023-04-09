import numpy as np
import numpy.typing as npt

from necs._internal.element_type import ElementType

def assign_value_at_indices_bool(
    array: npt.NDArray[np.bool_],
    indices: npt.NDArray[np.uint64],
    value: ElementType,
) -> None: ...
def assign_value_at_indices_i8(
    array: npt.NDArray[np.int8],
    indices: npt.NDArray[np.uint64],
    value: ElementType,
) -> None: ...
def assign_value_at_indices_i16(
    array: npt.NDArray[np.int16],
    indices: npt.NDArray[np.uint64],
    value: ElementType,
) -> None: ...
def assign_value_at_indices_i32(
    array: npt.NDArray[np.int32],
    indices: npt.NDArray[np.uint64],
    value: ElementType,
) -> None: ...
def assign_value_at_indices_i64(
    array: npt.NDArray[np.int64],
    indices: npt.NDArray[np.uint64],
    value: ElementType,
) -> None: ...
def assign_value_at_indices_u8(
    array: npt.NDArray[np.uint8],
    indices: npt.NDArray[np.uint64],
    value: ElementType,
) -> None: ...
def assign_value_at_indices_u16(
    array: npt.NDArray[np.uint16],
    indices: npt.NDArray[np.uint64],
    value: ElementType,
) -> None: ...
def assign_value_at_indices_u32(
    array: npt.NDArray[np.uint32],
    indices: npt.NDArray[np.uint64],
    value: ElementType,
) -> None: ...
def assign_value_at_indices_u64(
    array: npt.NDArray[np.uint64],
    indices: npt.NDArray[np.uint64],
    value: ElementType,
) -> None: ...
def assign_value_at_indices_f32(
    array: npt.NDArray[np.float32],
    indices: npt.NDArray[np.uint64],
    value: ElementType,
) -> None: ...
def assign_value_at_indices_f64(
    array: npt.NDArray[np.float64],
    indices: npt.NDArray[np.uint64],
    value: ElementType,
) -> None: ...
