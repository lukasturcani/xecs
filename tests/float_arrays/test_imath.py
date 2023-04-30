import operator
import typing

import ecstasy as ecs
import numpy as np
import numpy.typing as npt
import pytest

from tests.types import FloatArray, IntArray

if typing.TYPE_CHECKING:
    from ecstasy.ecstasy import NumpyFloat, NumpyInt


def test_ioperator_value(
    array: FloatArray,
    other_value: float,
    iop: typing.Any,
) -> None:
    expected = array.numpy()
    iop(array, other_value)
    iop(expected, other_value)
    assert np.all(np.equal(array.numpy(), expected))


def test_ioperator_numpy(
    array: FloatArray,
    other_numpy: "npt.NDArray[NumpyFloat | NumpyInt]",
    iop: typing.Any,
) -> None:
    expected = array.numpy()
    iop(array, other_numpy)
    iop(expected, other_numpy)
    assert np.all(np.equal(array.numpy(), expected))


def test_ioperator_array(
    array: FloatArray,
    other_array: FloatArray | IntArray,
    iop: typing.Any,
) -> None:
    expected = array.numpy()
    iop(array, other_array)
    iop(expected, other_array.numpy())
    assert np.all(np.equal(array.numpy(), expected))


def test_ioperator_on_subview_value(
    array: FloatArray,
    other_value: float,
    iop: typing.Any,
) -> None:
    expected = array.numpy()
    sub_view = array[0:2]
    iop(sub_view, other_value)
    iop(expected[0:2], other_value)
    assert np.all(np.equal(array.numpy(), expected))


def test_ioperator_on_subview_numpy(
    array: FloatArray,
    other_numpy: "npt.NDArray[NumpyFloat | NumpyInt]",
    iop: typing.Any,
) -> None:
    expected = array.numpy()
    sub_view = array[0:2]
    iop(sub_view, other_numpy[0:2])
    iop(expected[0:2], other_numpy[0:2])
    assert np.all(np.equal(array.numpy(), expected))


def test_ioperator_on_subview_array(
    array: FloatArray,
    other_array: FloatArray | IntArray,
    iop: typing.Any,
) -> None:
    expected = array.numpy()
    sub_view = array[0:2]
    iop(sub_view, other_array[0:2])
    iop(expected[0:2], other_array[0:2].numpy())
    assert np.all(np.equal(array.numpy(), expected))


def test_self() -> None:
    array = ecs.Float32.p_from_numpy(np.arange(5, dtype=np.float32))
    array += array
    assert np.all(np.equal(array.numpy(), [0, 2, 4, 6, 8]))


def test_self_slice() -> None:
    array = ecs.Float32.p_from_numpy(np.arange(5, dtype=np.float32))
    array += array[:]
    assert np.all(np.equal(array.numpy(), [0, 2, 4, 6, 8]))


def test_self_slice_both_sides() -> None:
    array = ecs.Float32.p_from_numpy(np.arange(5, dtype=np.float32))
    array[:] += array[:]
    assert np.all(np.equal(array.numpy(), [0, 2, 4, 6, 8]))


def test_self_mask() -> None:
    array = ecs.Float32.p_from_numpy(np.arange(5, dtype=np.float32))
    mask = [True, False, False, True, False]
    array[mask] += array[mask]
    assert np.all(np.equal(array.numpy(), [0, 2, 4, 6, 8]))


def test_works_with_complex_indices() -> None:
    array = ecs.Float32.p_from_numpy(np.arange(5, dtype=np.float32))
    array[[0, 3]] += np.array([10, 20])
    assert np.all(np.equal(array.numpy(), [10, 1, 2, 23, 4]))
    array[[0, 3]] += array[[0, 3]]
    assert np.all(np.equal(array.numpy(), [20, 1, 2, 46, 4]))


def test_works_with_mask() -> None:
    array = ecs.Float32.p_from_numpy(np.arange(5, dtype=np.float32))
    array[[True, False, False, True, False]] += np.array([10, 20])
    assert np.all(np.equal(array.numpy(), [10, 1, 2, 23, 4]))


@pytest.fixture(
    params=(
        operator.iadd,
        operator.isub,
        operator.imul,
        operator.itruediv,
        operator.ifloordiv,
        operator.imod,
        operator.ipow,
    ),
)
def iop(request: pytest.FixtureRequest) -> typing.Any:
    return request.param


@pytest.fixture(
    params=(
        lambda: 10,
        lambda: 10.5,
    ),
    ids=(
        "int",
        "float",
    ),
)
def other_value(request: pytest.FixtureRequest) -> float:
    return request.param()


@pytest.fixture(
    params=(
        lambda: np.array([10.2, 20.5, 30.5, 40.5, 50.5], dtype=np.float32),
        lambda: np.array([10.2, 20.5, 30.5, 40.5, 50.5], dtype=np.float64),
        lambda: np.array([10, 20, 30, 40, 50], dtype=np.int8),
        lambda: np.array([10, 20, 30, 40, 50], dtype=np.int16),
        lambda: np.array([10, 20, 30, 40, 50], dtype=np.int32),
        lambda: np.array([10, 20, 30, 40, 50], dtype=np.int64),
        lambda: np.array([10, 20, 30, 40, 50], dtype=np.uint8),
        lambda: np.array([10, 20, 30, 40, 50], dtype=np.uint16),
        lambda: np.array([10, 20, 30, 40, 50], dtype=np.uint32),
        lambda: np.array([10, 20, 30, 40, 50], dtype=np.uint64),
    ),
    ids=(
        "numpy_f32",
        "numpy_f64",
        "numpy_i8",
        "numpy_i16",
        "numpy_i32",
        "numpy_i64",
        "numpy_u8",
        "numpy_u16",
        "numpy_u32",
        "numpy_u64",
    ),
)
def other_numpy(
    request: pytest.FixtureRequest,
) -> "float | npt.NDArray[NumpyFloat | NumpyInt]":
    return request.param()


@pytest.fixture(
    params=(
        lambda: ecs.Float32.p_from_numpy(
            np.array([10.2, 20.5, 30.5, 40.5, 50.5], dtype=np.float32)
        ),
        lambda: ecs.Float64.p_from_numpy(
            np.array([10.2, 20.5, 30.5, 40.5, 50.5], dtype=np.float64)
        ),
        lambda: ecs.Int8.p_from_numpy(
            np.array([10, 20, 30, 40, 50], dtype=np.int8)
        ),
        lambda: ecs.Int16.p_from_numpy(
            np.array([10, 20, 30, 40, 50], dtype=np.int16)
        ),
        lambda: ecs.Int32.p_from_numpy(
            np.array([10, 20, 30, 40, 50], dtype=np.int32)
        ),
        lambda: ecs.Int64.p_from_numpy(
            np.array([10, 20, 30, 40, 50], dtype=np.int64)
        ),
        lambda: ecs.UInt8.p_from_numpy(
            np.array([10, 20, 30, 40, 50], dtype=np.uint8)
        ),
        lambda: ecs.UInt16.p_from_numpy(
            np.array([10, 20, 30, 40, 50], dtype=np.uint16)
        ),
        lambda: ecs.UInt32.p_from_numpy(
            np.array([10, 20, 30, 40, 50], dtype=np.uint32)
        ),
        lambda: ecs.UInt64.p_from_numpy(
            np.array([10, 20, 30, 40, 50], dtype=np.uint64)
        ),
    ),
    ids=(
        "Float32",
        "Float64",
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
def other_array(
    request: pytest.FixtureRequest,
) -> FloatArray | IntArray:
    return request.param()
