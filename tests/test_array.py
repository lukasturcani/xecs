import necs as ecs
import numpy as np
import numpy.typing as npt


def indices(xs: list[int]) -> npt.NDArray[np.uint32]:
    return np.array(xs, dtype=np.uint32)


def mask(xs: list[int]) -> npt.NDArray[np.bool_]:
    return np.array(xs, dtype=np.bool_)


def test_indexing_with_array_of_indices_does_not_return_a_copy() -> None:
    array = ecs.ArrayF64.from_numpy(np.zeros(100, dtype=np.float64))
    assert np.sum(array.numpy()) == 0
    view = array.view()

    sub_view = view[indices([0, 10, 50])]
    sub_view[:] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[[0, 10, 50]]) == 3


def test_assigning_with_array_of_indices_does_not_return_a_copy() -> None:
    array = ecs.ArrayF64.from_numpy(np.zeros(100, dtype=np.float64))
    assert np.sum(array.numpy()) == 0
    view = array.view()
    view[indices([0, 10, 50])] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[[0, 10, 50]]) == 3


def test_indexing_with_boolean_mask_does_not_return_a_copy() -> None:
    array = ecs.ArrayF64.from_numpy(np.zeros(5, dtype=np.float64))
    assert np.sum(array.numpy()) == 0
    view = array.view()
    sub_view = view[mask([True, False, True, False, True])]
    sub_view[:] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[[0, 2, 4]]) == 3


def test_assigning_with_boolean_mask_does_not_return_a_copy() -> None:
    array = ecs.ArrayF64.from_numpy(np.zeros(5, dtype=np.float64))
    assert np.sum(array.numpy()) == 0
    view = array.view()
    view[mask([True, False, True, False, True])] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[[0, 2, 4]]) == 3

    view[:] = np.arange(len(array.numpy()), dtype=np.float64)
    assert np.sum(array.numpy()) == 1 + 2 + 3 + 4
    view[array.numpy() < 3] = 100.0
    assert np.sum(array.numpy()) == 300 + 3 + 4


def test_indexing_with_slice_does_not_return_a_copy() -> None:
    array = ecs.ArrayF64.from_numpy(np.zeros(100, dtype=np.float64))
    assert np.sum(array.numpy()) == 0
    view = array.view()
    sub_view = view[5:8]
    sub_view[:] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[5:8]) == 3


def test_assigning_with_slice_does_not_return_a_copy() -> None:
    array = ecs.ArrayF64.from_numpy(np.zeros(100, dtype=np.float64))
    assert np.sum(array.numpy()) == 0
    view = array.view()
    view[5:8] = 1.0
    assert np.sum(array.numpy()) == 3
    assert np.sum(array.numpy()[5:8]) == 3
    view[5:8] = np.array([1.0, 2.0, 3.0])
    assert np.sum(array.numpy()) == 6
    assert np.sum(array.numpy()[5:8]) == 6


def test_mulitple_complex_indices_reach_correct_elements() -> None:
    array = ecs.ArrayF64.from_numpy(np.zeros(10, dtype=np.float64))
    view = array.view()
    view = view[indices([7, 8, 9])]
    view = view[indices([1, 2])]
    view[:] = 1.0
    assert np.sum(array.numpy()) == 2.0
    assert np.sum(array.numpy()[[8, 9]]) == 2.0
