import ecstasy as ecs
import numpy as np
import numpy.typing as npt
import pytest


def indices(xs: list[int]) -> npt.NDArray[np.uint32]:
    return np.array(xs, dtype=np.uint32)


def mask(xs: list[int]) -> npt.NDArray[np.bool_]:
    return np.array(xs, dtype=np.bool_)


def test_indexing_with_array_of_indices_does_not_return_a_copy() -> None:
    array = ecs.Float64.from_numpy(np.zeros(100, dtype=np.float64))
    assert np.sum(array.numpy()) == 0

    sub_array = array[indices([0, 10, 50])]
    sub_array[:] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[[0, 10, 50]]) == 3


def test_assigning_with_array_of_indices_does_not_return_a_copy() -> None:
    array = ecs.Float64.from_numpy(np.zeros(100, dtype=np.float64))
    assert np.sum(array.numpy()) == 0
    array[indices([0, 10, 50])] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[[0, 10, 50]]) == 3


def test_indexing_with_boolean_mask_does_not_return_a_copy() -> None:
    array = ecs.Float64.from_numpy(np.zeros(5, dtype=np.float64))
    assert np.sum(array.numpy()) == 0
    sub_array = array[mask([True, False, True, False, True])]
    sub_array[:] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[[0, 2, 4]]) == 3


def test_assigning_with_boolean_mask_does_not_return_a_copy() -> None:
    array = ecs.Float64.from_numpy(np.zeros(5, dtype=np.float64))
    assert np.sum(array.numpy()) == 0
    array[mask([True, False, True, False, True])] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[[0, 2, 4]]) == 3

    array[:] = np.arange(len(array.numpy()), dtype=np.float64)
    assert np.sum(array.numpy()) == 1 + 2 + 3 + 4
    array[array.numpy() < 3] = 100.0
    assert np.sum(array.numpy()) == 300 + 3 + 4


def test_indexing_with_slice_does_not_return_a_copy() -> None:
    array = ecs.Float64.from_numpy(np.zeros(100, dtype=np.float64))
    assert np.sum(array.numpy()) == 0
    sub_array = array[5:8]
    sub_array[:] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[5:8]) == 3


def test_assigning_with_slice_does_not_return_a_copy() -> None:
    array = ecs.Float64.from_numpy(np.zeros(100, dtype=np.float64))
    assert np.sum(array.numpy()) == 0
    array[5:8] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[5:8]) == 3
    array[5:8] = np.array([1.0, 2.0, 3.0])
    assert np.sum(array.numpy()) == 6
    assert np.sum(array.numpy()[5:8]) == 6


def test_mulitple_complex_indices_reach_correct_elements() -> None:
    array = ecs.Float64.from_numpy(np.zeros(10, dtype=np.float64))
    array = array[indices([7, 8, 9])]
    array = array[indices([1, 2])]
    array[:] = 1.0
    assert np.sum(array.numpy()) == 2.0
    assert np.sum(array.numpy()[[8, 9]]) == 2.0


def test_length_of_sub_array_is_accurate() -> None:
    array = ecs.Float64.from_numpy(np.zeros(10, dtype=np.float64))
    assert len(array) == 10
    sub_array = array[indices([5, 8, 9])]
    assert len(sub_array) == 3
    assert len(sub_array[indices([1])]) == 1


def test_spawning_increases_length() -> None:
    indices = ecs.ecstasy.ArrayViewIndices.with_capacity(10)
    array = ecs.Float64.p_with_capacity(
        capacity=10,
        indices=indices,
    )
    assert len(array) == 0
    indices.spawn(2)
    assert len(array) == 2
    indices.spawn(5)
    assert len(array) == 7


def test_view_indices_are_shared_between_arrays() -> None:
    indices = ecs.ecstasy.ArrayViewIndices.with_capacity(10)
    array_1 = ecs.Float64.p_with_capacity(
        capacity=10,
        indices=indices,
    )
    array_2 = ecs.Float64.p_with_capacity(
        capacity=10,
        indices=indices,
    )
    assert len(array_1) == len(array_2) == 0
    indices.spawn(5)
    assert len(array_1) == len(array_2) == 5


def test_spawning_to_a_full_array_causes_error() -> None:
    indices = ecs.ecstasy.ArrayViewIndices.with_capacity(10)
    array = ecs.Float64.p_with_capacity(
        capacity=10,
        indices=indices,
    )
    indices.spawn(6)
    indices.spawn(4)
    with pytest.raises(
        RuntimeError,
        match="cannot spawn more entities because pool is full",
    ):
        indices.spawn(1)
    array[:] = 1.0
