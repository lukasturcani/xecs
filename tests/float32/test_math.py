import operator
import typing

import ecstasy as ecs
import numpy as np
import numpy.typing as npt
import pytest


def test_operator_value(
    array: ecs.Float32,
    other_value: float,
    op: typing.Any,
) -> None:
    expected = op(array.numpy(), other_value)
    result = op(array, other_value)
    assert np.all(np.equal(result.numpy(), expected))


def test_operator_numpy(
    array: ecs.Float32,
    other_numpy: npt.NDArray[np.float32],
    op: typing.Any,
) -> None:
    expected = op(array.numpy(), other_numpy)
    result = op(array, other_numpy)
    assert np.all(np.equal(result.numpy(), expected))


def test_operator_array(
    array: ecs.Float32,
    other_array: ecs.Float32,
    op: typing.Any,
) -> None:
    expected = op(array.numpy(), other_array.numpy())
    result = op(array, other_array)
    assert np.all(np.equal(result.numpy(), expected))


def test_operator_list(
    array: ecs.Float32,
    other_list: list[float],
    op: typing.Any,
) -> None:
    expected = op(array.numpy(), other_list)
    result = op(array, other_list)
    assert np.allclose(result.numpy(), expected)


def test_self(array: ecs.Float32, op: typing.Any) -> None:
    with pytest.raises(TypeError):
        op(array, array)


@pytest.fixture(
    params=(
        operator.add,
        operator.sub,
        operator.mul,
        operator.truediv,
        operator.floordiv,
        # operator.mod,
        # operator.pow,
    ),
)
def op(request: pytest.FixtureRequest) -> typing.Any:
    return request.param


@pytest.fixture
def array() -> ecs.Float32:
    return ecs.Float32.p_from_numpy(np.arange(1, 6, dtype=np.float32))


@pytest.fixture
def other_array() -> ecs.Float32:
    return ecs.Float32.p_from_numpy(np.arange(6, 11, dtype=np.float32))


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
