import ecstasy as ecs
import numpy as np
import pytest

from tests.types import FloatArray


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
