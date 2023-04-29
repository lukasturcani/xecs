import typing

import ecstasy as ecs
import numpy as np
import pytest

from tests.types import FloatArray

if typing.TYPE_CHECKING:
    from ecstasy.ecstasy import FloatRhs, GetItemKey


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
def array(request: pytest.FixtureRequest) -> FloatArray:
    return request.param()


@pytest.fixture(
    params=(
        [0, 1],
        [True, True, False, False, False],
        np.array([0, 1], dtype=np.uint32),
        np.array([True, True, False, False, False], dtype=np.bool_),
        slice(0, 2),
    ),
    ids=(
        "list_indices",
        "list_mask",
        "numpy_indices",
        "numpy_mask",
        "slice",
    ),
)
def key(request: pytest.FixtureRequest) -> "GetItemKey":
    return request.param


@pytest.fixture(
    params=(
        lambda: 100,
        lambda: 100.0,
        lambda: np.array([100, 100], dtype=np.float32),
        lambda: np.array([100, 100], dtype=np.float64),
        lambda: np.array([100, 100], dtype=np.int8),
        lambda: np.array([100, 100], dtype=np.int16),
        lambda: np.array([100, 100], dtype=np.int32),
        lambda: np.array([100, 100], dtype=np.int64),
        lambda: np.array([100, 100], dtype=np.uint8),
        lambda: np.array([100, 100], dtype=np.uint16),
        lambda: np.array([100, 100], dtype=np.uint32),
        lambda: np.array([100, 100], dtype=np.uint64),
        lambda: ecs.Float32.p_from_numpy(
            np.array([100, 100], dtype=np.float32)
        ),
        lambda: ecs.Float64.p_from_numpy(
            np.array([100, 100], dtype=np.float64)
        ),
        lambda: ecs.Int8.p_from_numpy(np.array([100, 100], dtype=np.int8)),
        lambda: ecs.Int16.p_from_numpy(np.array([100, 100], dtype=np.int16)),
        lambda: ecs.Int32.p_from_numpy(np.array([100, 100], dtype=np.int32)),
        lambda: ecs.Int64.p_from_numpy(np.array([100, 100], dtype=np.int64)),
        lambda: ecs.UInt8.p_from_numpy(np.array([100, 100], dtype=np.uint8)),
        lambda: ecs.UInt16.p_from_numpy(np.array([100, 100], dtype=np.uint16)),
        lambda: ecs.UInt32.p_from_numpy(np.array([100, 100], dtype=np.uint32)),
        lambda: ecs.UInt64.p_from_numpy(np.array([100, 100], dtype=np.uint64)),
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
def single_value_float_rhs(request: pytest.FixtureRequest) -> "FloatRhs":
    return request.param()


@pytest.fixture(
    params=(
        lambda: np.array([50, 100], dtype=np.float32),
        lambda: np.array([50, 100], dtype=np.float64),
        lambda: np.array([50, 100], dtype=np.int8),
        lambda: np.array([50, 100], dtype=np.int16),
        lambda: np.array([50, 100], dtype=np.int32),
        lambda: np.array([50, 100], dtype=np.int64),
        lambda: np.array([50, 100], dtype=np.uint8),
        lambda: np.array([50, 100], dtype=np.uint16),
        lambda: np.array([50, 100], dtype=np.uint32),
        lambda: np.array([50, 100], dtype=np.uint64),
        lambda: ecs.Float32.p_from_numpy(
            np.array([50, 100], dtype=np.float32)
        ),
        lambda: ecs.Float64.p_from_numpy(
            np.array([50, 100], dtype=np.float64)
        ),
        lambda: ecs.Int8.p_from_numpy(np.array([50, 100], dtype=np.int8)),
        lambda: ecs.Int16.p_from_numpy(np.array([50, 100], dtype=np.int16)),
        lambda: ecs.Int32.p_from_numpy(np.array([50, 100], dtype=np.int32)),
        lambda: ecs.Int64.p_from_numpy(np.array([50, 100], dtype=np.int64)),
        lambda: ecs.UInt8.p_from_numpy(np.array([50, 100], dtype=np.uint8)),
        lambda: ecs.UInt16.p_from_numpy(np.array([50, 100], dtype=np.uint16)),
        lambda: ecs.UInt32.p_from_numpy(np.array([50, 100], dtype=np.uint32)),
        lambda: ecs.UInt64.p_from_numpy(np.array([50, 100], dtype=np.uint64)),
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
def multiple_value_float_rhs(request: pytest.FixtureRequest) -> "FloatRhs":
    return request.param()


@pytest.fixture(
    params=(
        lambda: 3.125,
        lambda: np.array([3.125, 3.125], dtype=np.float32),
        lambda: np.array([3.125, 3.125], dtype=np.float64),
        lambda: ecs.Float32.p_from_numpy(
            np.array([3.125, 3.125], dtype=np.float32)
        ),
        lambda: ecs.Float64.p_from_numpy(
            np.array([3.125, 3.125], dtype=np.float64)
        ),
    ),
    ids=(
        "float",
        "numpy_f32",
        "numpy_f64",
        "Float32",
        "Float64",
    ),
)
def non_integer_value(request: pytest.FixtureRequest) -> "FloatRhs":
    return request.param()
