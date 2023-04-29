import ecstasy as ecs
import numpy as np
import pytest

from tests.types import IntArray


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
def array(request: pytest.FixtureRequest) -> IntArray:
    return request.param()
