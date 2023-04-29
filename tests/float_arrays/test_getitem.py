import numpy as np
import pytest

from tests.types import FloatArray, GetItemKey


def test_getitem_does_not_return_a_copy(
    array: FloatArray,
    key: GetItemKey,
) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    sub_array = array[key]
    sub_array[:] = 100
    assert np.all(np.equal(array.numpy(), [100, 100, 2, 3, 4]))


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
