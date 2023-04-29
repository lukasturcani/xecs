import operator
import typing

import ecstasy as ecs
import numpy as np
import numpy.typing as npt
import pytest

from tests.types import FloatArray, IntArray

if typing.TYPE_CHECKING:
    from ecstasy.ecstasy import NumpyFloat, NumpyInt


def test_ioperator_numpy(
    array: FloatArray,
    other_numpy: "float | npt.NDArray[NumpyFloat | NumpyInt]",
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


def test_ioperator_on_subview(array: FloatArray, iop: typing.Any) -> None:
    pass


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
        "int",
        "float",
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
