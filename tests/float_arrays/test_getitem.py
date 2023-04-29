import typing

import numpy as np

from tests.types import FloatArray

if typing.TYPE_CHECKING:
    from ecstasy.ecstasy import GetItemKey


def test_getitem_does_not_return_a_copy(
    array: FloatArray,
    key: "GetItemKey",
) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    sub_array = array[key]
    sub_array[:] = 100
    assert np.all(np.equal(array.numpy(), [100, 100, 2, 3, 4]))
