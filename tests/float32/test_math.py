import operator
import typing

import numpy as np
import numpy.typing as npt
import pytest
import xecs as xx


def test_operator_value(
    array: xx.Float32,
    other_value: float,
    op: typing.Any,
) -> None:
    expected = op(array.numpy(), other_value)
    result = op(array, other_value)
    assert np.all(np.equal(result, expected))


def test_operator_numpy(
    array: xx.Float32,
    other_numpy: npt.NDArray[np.float32],
    op: typing.Any,
) -> None:
    expected = op(array.numpy(), other_numpy)
    result = op(array, other_numpy)
    assert np.all(np.equal(result, expected))


def test_operator_array(
    array: xx.Float32,
    other_array: xx.Float32,
    op: typing.Any,
) -> None:
    expected = op(array.numpy(), other_array.numpy())
    result = op(array, other_array)
    assert np.all(np.equal(result, expected))


def test_operator_list(
    array: xx.Float32,
    other_list: list[float],
    op: typing.Any,
) -> None:
    expected = op(array.numpy(), other_list)
    result = op(array, other_list)
    assert np.allclose(result, expected)


def test_self(array: xx.Float32, op: typing.Any) -> None:
    expected = op(array.numpy(), array.numpy())
    result = op(array, array)
    assert np.all(np.equal(result, expected))


@pytest.fixture(
    params=(
        operator.add,
        operator.sub,
        operator.mul,
        operator.truediv,
        operator.floordiv,
        operator.mod,
        operator.pow,
    ),
)
def op(request: pytest.FixtureRequest) -> typing.Any:
    return request.param


@pytest.fixture
def array() -> xx.Float32:
    return xx.Float32.p_from_numpy(np.arange(1, 6, dtype=np.float32))


@pytest.fixture
def other_array() -> xx.Float32:
    return xx.Float32.p_from_numpy(np.arange(6, 11, dtype=np.float32))


@pytest.fixture
def other_numpy() -> npt.NDArray[np.float32]:
    return np.arange(5, 10, dtype=np.float32)


@pytest.fixture
def other_list() -> list[float]:
    return list(range(5, 10))


@pytest.fixture(
    params=(12, 12.5),
    ids=("int", "float"),
)
def other_value(request: pytest.FixtureRequest) -> float:
    return request.param
