import typing

import numpy as np

from tests.types import FloatArray

if typing.TYPE_CHECKING:
    from ecstasy.ecstasy import FloatRhs, GetItemKey


def test_setitem_self(array: FloatArray, key: "GetItemKey") -> None:
    array[key] = array[key]


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


def test_setitem_non_integer_value(
    array: FloatArray,
    key: "GetItemKey",
    non_integer_value: "FloatRhs",
) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    array[key] = non_integer_value
    assert np.all(np.equal(array.numpy(), [3.125, 3.125, 2, 3, 4]))


def test_setitem_on_subview(array: FloatArray, key: "GetItemKey") -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    subview = array[key]
    subview[[1]] = 100.0
    assert np.all(np.equal(array.numpy(), [0, 100, 2, 3, 4]))
