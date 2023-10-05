import numpy as np
import numpy.typing as npt
import pytest
import xecs as xx


def mask(xs: list[bool]) -> npt.NDArray[np.bool_]:
    return np.array(xs, dtype=np.bool_)


def indices(length: int, indices: list[int]) -> npt.NDArray[np.bool_]:
    mask = np.zeros(length, dtype=np.bool_)
    mask[indices] = True
    return mask


def test_indexing_with_boolean_mask_does_not_return_a_copy() -> None:
    array = xx.Float32.p_from_numpy(np.zeros(5, dtype=np.float32))
    assert np.sum(array.numpy()) == 0
    sub_array = array[mask([True, False, True, False, True])]
    all_mask = np.ones(len(sub_array), dtype=np.bool_)
    sub_array[all_mask] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[[0, 2, 4]]) == 3


def test_assigning_with_boolean_mask_does_not_return_a_copy() -> None:
    array = xx.Float32.p_from_numpy(np.zeros(5, dtype=np.float32))
    assert np.sum(array.numpy()) == 0
    array[mask([True, False, True, False, True])] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[[0, 2, 4]]) == 3

    all_mask = np.ones(len(array), dtype=np.bool_)
    array[all_mask] = np.arange(len(array), dtype=np.float32)
    assert np.sum(array.numpy()) == 1 + 2 + 3 + 4
    array[array.numpy() < 3] = 100.0
    assert np.sum(array.numpy()) == 300 + 3 + 4


def test_mulitple_complex_indices_reach_correct_elements() -> None:
    array = xx.Float32.p_from_numpy(np.zeros(10, dtype=np.float32))
    sub_array = array[indices(10, [7, 8, 9])]
    sub_array = sub_array[indices(3, [1, 2])]
    all_mask = np.ones(len(sub_array), dtype=np.bool_)
    sub_array[all_mask] = 1.0
    assert np.sum(sub_array.numpy()) == 2.0
    assert np.sum(array.numpy()) == 2.0
    assert np.sum(array.numpy()[[8, 9]]) == 2.0


def test_length_of_sub_array_is_accurate() -> None:
    array = xx.Float32.p_from_numpy(np.zeros(10, dtype=np.float32))
    assert len(array) == 10
    sub_array = array[indices(10, [5, 8, 9])]
    assert len(sub_array) == 3
    assert len(sub_array[indices(3, [1])]) == 1
