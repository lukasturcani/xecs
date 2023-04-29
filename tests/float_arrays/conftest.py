import ecstasy as ecs
import numpy as np
import pytest

from tests.types import FloatArray, GetItemKey


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
def key(request: pytest.FixtureRequest) -> GetItemKey:
    return request.param
