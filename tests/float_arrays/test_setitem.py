import typing

import numpy as np

from tests.types import FloatArray

if typing.TYPE_CHECKING:
    from ecstasy.ecstasy import FloatRhs, GetItemKey


def test_setitem_single_value(
    array: FloatArray,
    key: "GetItemKey",
    single_value_float_rhs: "FloatRhs",
) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    array[key] = single_value_float_rhs
    assert np.all(np.equal(array.numpy(), [100, 100, 2, 3, 4]))


def test_setitem_multiple_values(
    array: FloatArray,
    key: "GetItemKey",
    multiple_value_float_rhs: "FloatRhs",
) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    array[key] = multiple_value_float_rhs
    assert np.all(np.equal(array.numpy(), [50, 100, 2, 3, 4]))
