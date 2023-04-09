import necs as ecs
import numpy as np


def test_indexing_with_list_of_indices_does_not_return_a_copy() -> None:
    numpy_array = np.zeros(100, dtype=np.float64)
    assert np.sum(numpy_array) == 0
    array = ecs.Array(numpy_array)
    sub_array = array[[0, 10, 50]]
    sub_array[:] = 1.0
    assert np.sum(numpy_array) == 3
    assert np.sum(numpy_array[[0, 10, 50]]) == 3


def test_assigning_with_list_of_indices_does_not_return_a_copy() -> None:
    numpy_array = np.zeros(100, dtype=np.float64)
    assert np.sum(numpy_array) == 0
    array = ecs.Array(numpy_array)
    array[[0, 10, 50]] = 1.0
    assert np.sum(numpy_array) == 3
    assert np.sum(numpy_array[[0, 10, 50]]) == 3


def test_indexing_with_boolean_mask_does_not_return_a_copy() -> None:
    numpy_array = np.zeros(5, dtype=np.float64)
    assert np.sum(numpy_array) == 0
    array = ecs.Array(numpy_array)
    sub_array = array[[True, False, True, False, True]]
    sub_array[:] = 1.0
    assert np.sum(numpy_array) == 3
    assert np.sum(numpy_array[[0, 2, 4]]) == 3


def test_assigning_with_boolean_mask_does_not_return_a_copy() -> None:
    numpy_array = np.zeros(5, dtype=np.float64)
    assert np.sum(numpy_array) == 0
    array = ecs.Array(numpy_array)
    array[[True, False, True, False, True]] = 1.0
    assert np.sum(numpy_array) == 3
    assert np.sum(numpy_array[[0, 2, 4]]) == 3

    array[:] = np.arange(len(numpy_array), dtype=np.float64)
    assert np.sum(numpy_array) == 1 + 2 + 3 + 4
    array[numpy_array < 3] = 100.0
    assert np.sum(numpy_array) == 300 + 3 + 4


def test_indexing_with_slice_does_not_return_a_copy() -> None:
    numpy_array = np.zeros(100, dtype=np.float64)
    assert np.sum(numpy_array) == 0
    array = ecs.Array(numpy_array)
    sub_array = array[5:8]
    sub_array[:] = 1.0
    assert np.sum(numpy_array) == 3
    assert np.sum(numpy_array[5:8]) == 3


def test_assigning_with_slice_does_not_return_a_copy() -> None:
    numpy_array = np.zeros(100, dtype=np.float64)
    assert np.sum(numpy_array) == 0
    array = ecs.Array(numpy_array)
    array[5:8] = 1.0
    assert np.sum(numpy_array) == 3
    assert np.sum(numpy_array[5:8]) == 3
    array[5:8] = np.array([1.0, 2.0, 3.0])
    assert np.sum(numpy_array) == 6
    assert np.sum(numpy_array[5:8]) == 6
    array[5:8] = [10.0, 20.0, 30.0]
    assert np.sum(numpy_array) == 60
    assert np.sum(numpy_array[5:8]) == 60
    array[5:8] = (100.0, 200.0, 300.0)
    assert np.sum(numpy_array) == 600
    assert np.sum(numpy_array[5:8]) == 600


def test_assigning_with_number_does_not_return_a_copy() -> None:
    numpy_array = np.zeros(100, dtype=np.float64)
    assert np.sum(numpy_array) == 0
    array = ecs.Array(numpy_array)
    array[5] = 1.0
    assert np.sum(numpy_array) == 1
    assert numpy_array[5] == 1


def test_mulitple_complex_indices_reach_correct_elements() -> None:
    numpy_array = np.zeros(10, dtype=np.float64)
    array = ecs.Array(numpy_array)
    xs = array[[7, 8, 9]]
    xs = xs[[1, 2]]
    xs[:] = 1.0
    assert np.sum(numpy_array) == 2.0
    assert np.sum(numpy_array[[8, 9]]) == 2.0
