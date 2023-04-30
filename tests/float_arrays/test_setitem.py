import typing

import ecstasy as ecs
import numpy as np
import pytest

from tests.types import FloatArray

if typing.TYPE_CHECKING:
    from ecstasy.ecstasy import FloatRhs, GetItemKey


def test_setitem_same_array_different_indices(
    array: FloatArray,
    key: "GetItemKey",
) -> None:
    before = array.numpy()
    array[key] = array[key]
    after = array.numpy()
    assert np.all(np.equal(before, after))


def test_setitem_same_array_different_indices_detached(
    array: FloatArray,
    detached_key: "GetItemKey",
) -> None:
    before = array.numpy()
    array[detached_key] = array[detached_key]
    after = array.numpy()
    assert np.all(np.equal(before, after))


def test_setitem_same_array_same_indices(
    array: FloatArray,
    key: "GetItemKey",
) -> None:
    with pytest.raises(TypeError):
        array[key] = array


def test_setitem_same_array_same_indices_detached(
    array: FloatArray,
    detached_key: "GetItemKey",
) -> None:
    with pytest.raises(TypeError):
        array[detached_key] = array


def test_setitem_subview(array: FloatArray, key: "GetItemKey") -> None:
    before = array.numpy()
    sub_view = array[key]
    array[key] = sub_view
    assert np.all(np.equal(array.numpy(), before))


def test_setitem_subview_detached(
    array: FloatArray, detached_key: "GetItemKey"
) -> None:
    before = array.numpy()
    sub_view = array[detached_key]
    array[detached_key] = sub_view
    assert np.all(np.equal(array.numpy(), before))


def test_setitem_single_value(
    array: FloatArray,
    key: "GetItemKey",
    single_value_float_rhs: "FloatRhs",
) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    array[key] = single_value_float_rhs
    assert np.all(np.equal(array.numpy(), [100, 100, 2, 3, 4]))


def test_setitem_single_value_detached(
    array: FloatArray,
    detached_key: "GetItemKey",
    single_value_float_rhs: "FloatRhs",
) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    array[detached_key] = single_value_float_rhs
    assert np.all(np.equal(array.numpy(), [100, 1, 2, 100, 4]))


def test_setitem_multiple_values(
    array: FloatArray,
    key: "GetItemKey",
    multiple_value_float_rhs: "FloatRhs",
) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    array[key] = multiple_value_float_rhs
    assert np.all(np.equal(array.numpy(), [50, 100, 2, 3, 4]))


def test_setitem_multiple_values_detached(
    array: FloatArray,
    detached_key: "GetItemKey",
    multiple_value_float_rhs: "FloatRhs",
) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    array[detached_key] = multiple_value_float_rhs
    assert np.all(np.equal(array.numpy(), [50, 1, 2, 100, 4]))


def test_setitem_non_integer_value(
    array: FloatArray,
    key: "GetItemKey",
    non_integer_value: "FloatRhs",
) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    array[key] = non_integer_value
    assert np.all(np.equal(array.numpy(), [3.125, 3.125, 2, 3, 4]))


def test_setitem_non_integer_value_detached(
    array: FloatArray,
    detached_key: "GetItemKey",
    non_integer_value: "FloatRhs",
) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    array[detached_key] = non_integer_value
    assert np.all(np.equal(array.numpy(), [3.125, 1, 2, 3.125, 4]))


def test_setitem_on_subview(array: FloatArray, key: "GetItemKey") -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    subview = array[key]
    subview[[1]] = 100.0
    assert np.all(np.equal(array.numpy(), [0, 100, 2, 3, 4]))


def test_setitem_on_subview_detached(
    array: FloatArray, detached_key: "GetItemKey"
) -> None:
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 3, 4]))
    subview = array[detached_key]
    subview[[1]] = 100.0
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 100, 4]))
