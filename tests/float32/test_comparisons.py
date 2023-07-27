import operator
import typing

import numpy as np
import numpy.typing as npt
import pytest
import xecs as xx


def test_ioperator_value(
    array: xx.Float32,
    other_value: float,
    op: typing.Any,
) -> None:
    result = op(array, other_value)
    expected = op(array.numpy(), other_value)
    assert np.all(np.equal(result, expected))


def test_ioperator_numpy(
    array: xx.Float32,
    other_numpy: npt.NDArray[np.float32],
    op: typing.Any,
) -> None:
    result = op(array, other_numpy)
    expected = op(array.numpy(), other_numpy)
    assert np.all(np.equal(result, expected))


def test_ioperator_array(
    array: xx.Float32,
    other_array: xx.Float32,
    op: typing.Any,
) -> None:
    result = op(array, other_array)
    expected = op(array.numpy(), other_array.numpy())
    assert np.all(np.equal(result, expected))


def test_ioperator_list(
    array: xx.Float32,
    other_list: list[float],
    op: typing.Any,
) -> None:
    result = op(array, other_list)
    expected = op(array.numpy(), other_list)
    assert np.all(np.equal(result, expected))


def test_self(array: xx.Float32, op: typing.Any) -> None:
    result = op(array, array)
    expected = op(array.numpy(), array.numpy())
    assert np.all(np.equal(result, expected))


def test_self_mask(array: xx.Float32, op: typing.Any) -> None:
    all_mask = np.ones(len(array), dtype=np.bool_)
    result = op(array, array[all_mask])
    expected = op(array.numpy(), array.numpy())
    assert np.all(np.equal(result, expected))


def test_self_slice_both_sides(array: xx.Float32, op: typing.Any) -> None:
    all_mask = np.ones(len(array), dtype=np.bool_)
    result = op(array[all_mask], array[all_mask])
    expected = op(array.numpy(), array.numpy())
    assert np.all(np.equal(result, expected))


@pytest.fixture(
    params=(
        operator.lt,
        operator.le,
        operator.gt,
        operator.ge,
        operator.eq,
        operator.ne,
    ),
)
def op(request: pytest.FixtureRequest) -> typing.Any:
    return request.param


@pytest.fixture
def array() -> xx.Float32:
    return xx.Float32.p_from_numpy(np.array([0, 10, 3, 5], dtype=np.float32))


@pytest.fixture
def other_array() -> xx.Float32:
    return xx.Float32.p_from_numpy(np.array([1, 5, 3, 11], dtype=np.float32))


@pytest.fixture
def other_numpy() -> npt.NDArray[np.float32]:
    return np.array([1, 5, 3, 11], dtype=np.float32)


@pytest.fixture
def other_list() -> list[float]:
    return [1, 5, 3, 11]


@pytest.fixture(
    params=(8, 8.5),
    ids=("int", "float"),
)
def other_value(request: pytest.FixtureRequest) -> float:
    return request.param
