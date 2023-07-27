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


def test_spawning_increases_length() -> None:
    indices = xx.ArrayViewIndices.with_capacity(10)
    array = xx.Float32.p_with_indices(indices)
    assert len(array) == 0
    indices.spawn(2)
    assert len(array) == 2
    indices.spawn(5)
    assert len(array) == 7


def test_view_indices_are_shared_between_arrays() -> None:
    indices = xx.ArrayViewIndices.with_capacity(10)
    array_1 = xx.Float32.p_with_indices(indices)
    array_2 = xx.Float32.p_with_indices(indices)
    assert len(array_1) == len(array_2) == 0
    indices.spawn(5)
    assert len(array_1) == len(array_2) == 5


def test_spawning_to_a_full_array_causes_error() -> None:
    indices = xx.ArrayViewIndices.with_capacity(10)
    array = xx.Float32.p_with_indices(indices)
    indices.spawn(6)
    indices.spawn(4)
    with pytest.raises(
        RuntimeError,
        match="cannot spawn more entities because pool is full",
    ):
        indices.spawn(1)
    # Prove that writing does not cause a segfault.
    all_mask = np.ones(len(array), dtype=np.bool_)
    array[all_mask] = 1.0


def test_new_view_uses_same_array() -> None:
    array_1 = xx.Float32.p_from_numpy(np.zeros(10, dtype=np.float32))
    array_indices = xx.ArrayViewIndices.with_capacity(10)
    array_2 = array_1.p_new_view_with_indices(array_indices)
    array_indices.spawn(5)

    assert len(array_1) == 10
    assert len(array_2) == 5
    assert array_1.numpy()[2] == array_2.numpy()[2] == 0
    assert array_1.numpy()[4] == array_2.numpy()[4] == 0

    array_1[indices(10, [2])] = 1
    assert array_1.numpy()[2] == array_2.numpy()[2] == 1

    array_2[indices(5, [4])] = 2
    assert array_1.numpy()[4] == array_2.numpy()[4] == 2
