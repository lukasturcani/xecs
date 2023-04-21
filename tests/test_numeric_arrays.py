import typing

import ecstasy as ecs
import numpy as np
import numpy.typing as npt
import pytest
from pytest_lazyfixture import lazy_fixture

FloatArray: typing.TypeAlias = ecs.Float32 | ecs.Float64

IntArray: typing.TypeAlias = (
    ecs.Int8
    | ecs.Int16
    | ecs.Int32
    | ecs.Int64
    | ecs.UInt8
    | ecs.UInt16
    | ecs.UInt32
    | ecs.UInt64
)

Array: typing.TypeAlias = FloatArray | IntArray

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


def test_getitem_does_not_return_a_copy(array: Array) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))

    mask = array < 3
    sub_array = array[mask]
    assert len(sub_array) == 3
    sub_array[sub_array < 2] = 100
    assert np.all(np.equal(array.numpy(), [100, 100, 2, 3, 4]))


def test_float_array_setitem(
    float_array: FloatArray,
    float_setitem_rhs: float | Array | npt.NDArray[NumpyFloat | NumpyInt],
) -> None:
    assert np.sum(array.numpy()) == 0
    array[5:8] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[5:8]) == 3
    array[5:8] = np.array([1.0, 2.0, 3.0])
    assert np.sum(array.numpy()) == 6
    assert np.sum(array.numpy()[5:8]) == 6


def test_mulitple_masks_reach_correct_elements(array: Array) -> None:
    array = array[indices([7, 8, 9])]
    array = array[indices([1, 2])]
    array[:] = 1.0
    assert np.sum(array.numpy()) == 2.0
    assert np.sum(array.numpy()[[8, 9]]) == 2.0


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


def test_float_array_type_checking() -> None:
    pass


def test_int_array_type_checking() -> None:
    pass


@pytest.fixture(
    params=(
        lambda: ecs.Float32.p_from_numpy(np.arange(5, dtype=np.float32)),
        lambda: ecs.Float64.p_from_numpy(np.arange(5, dtype=np.float64)),
    ),
    ids=(
        "Float32",
        "Float64",
    ),
)
def float_array(request: pytest.FixtureRequest) -> FloatArray:
    return request.param()


@pytest.fixture(
    params=(
        lambda: ecs.Int8.p_from_numpy(np.arange(5, dtype=np.int8)),
        lambda: ecs.Int16.p_from_numpy(np.arange(5, dtype=np.int16)),
        lambda: ecs.Int32.p_from_numpy(np.arange(5, dtype=np.int32)),
        lambda: ecs.Int64.p_from_numpy(np.arange(5, dtype=np.int64)),
        lambda: ecs.UInt8.p_from_numpy(np.arange(5, dtype=np.uint8)),
        lambda: ecs.UInt16.p_from_numpy(np.arange(5, dtype=np.uint16)),
        lambda: ecs.UInt32.p_from_numpy(np.arange(5, dtype=np.uint32)),
        lambda: ecs.UInt64.p_from_numpy(np.arange(5, dtype=np.uint64)),
    ),
    ids=(
        "Int8",
        "Int16",
        "Int32",
        "Int64",
        "UInt8",
        "UInt16",
        "UInt32",
        "UInt64",
    ),
)
def int_array(request: pytest.FixtureRequest) -> IntArray:
    return request.param()


@pytest.fixture(
    params=(
        lazy_fixture("float_array"),
        lazy_fixture("int_array"),
    ),
)
def array(request: pytest.FixtureRequest) -> Array:
    return request.param
