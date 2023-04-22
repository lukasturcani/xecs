import typing

import ecstasy as ecs
import numpy as np
import numpy.typing as npt
import pytest

from tests.types import Array, FloatArray

NumpyFloat: typing.TypeAlias = np.float32 | np.float64
NumpyInt: typing.TypeAlias = (
    np.int8
    | np.int16
    | np.int32
    | np.int64
    | np.uint8
    | np.uint16
    | np.uint32
    | np.uint64
)


def test_float_array_setitem(
    float_array: FloatArray,
    float_setitem_rhs: float | Array | npt.NDArray[NumpyFloat | NumpyInt],
) -> None:
    assert np.all(np.equal(float_array.numpy(), [0, 1, 2, 3, 4]))
    array[5:8] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[5:8]) == 3
    array[5:8] = np.array([1.0, 2.0, 3.0])
    assert np.sum(array.numpy()) == 6
    assert np.sum(array.numpy()[5:8]) == 6


def test_multiple_masks_reach_correct_elements(array: Array) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    sub_array = array[array < 3]
    sub_array = sub_array[sub_array > 1]
    sub_array[:] = 100
    assert np.all(np.equal(array.numpy(), [0, 1, 100, 3, 4]))


def test_length_of_sub_array_is_accurate(array: Array) -> None:
    assert len(array) == 10
    sub_array = array[indices([5, 8, 9])]
    assert len(sub_array) == 3
    assert len(sub_array[indices([1])]) == 1


def test_spawning_increases_length() -> None:
    indices = ecs.ecstasy.ArrayViewIndices.with_capacity(10)
    array = ecs.Float64.p_with_indices(indices)
    assert len(array) == 0
    indices.spawn(2)
    assert len(array) == 2
    indices.spawn(5)
    assert len(array) == 7


def test_view_indices_are_shared_between_arrays() -> None:
    indices = ecs.ecstasy.ArrayViewIndices.with_capacity(10)
    array_1 = ecs.Float64.p_with_indices(indices)
    array_2 = ecs.Float64.p_with_indices(indices)
    assert len(array_1) == len(array_2) == 0
    indices.spawn(5)
    assert len(array_1) == len(array_2) == 5


def test_spawning_to_a_full_array_causes_error() -> None:
    indices = ecs.ecstasy.ArrayViewIndices.with_capacity(10)
    array = ecs.Float64.p_with_indices(indices)
    indices.spawn(6)
    indices.spawn(4)
    with pytest.raises(
        RuntimeError,
        match="cannot spawn more entities because pool is full",
    ):
        indices.spawn(1)
    # Prove that writing does not cause a segfault.
    array[:] = 1.0


def test_new_view_uses_same_array() -> None:
    array_1 = ecs.Float64.from_numpy(np.zeros(10, dtype=np.float64))
    indices = ecs.ecstasy.ArrayViewIndices.with_capacity(10)
    array_2 = array_1.p_new_view_with_indices(indices)
    indices.spawn(5)

    assert len(array_1) == 10
    assert len(array_2) == 5
    assert array_1.numpy()[2] == array_2.numpy()[2] == 0
    assert array_1.numpy()[4] == array_2.numpy()[4] == 0

    array_1[2:3] = 1
    assert array_1.numpy()[2] == array_2.numpy()[2] == 1

    array_2[4:5] = 2
    assert array_1.numpy()[4] == array_2.numpy()[4] == 2


def test_float_array_type_checking() -> None:
    pass


def test_int_array_type_checking() -> None:
    pass
