import operator
import typing

import ecstasy as ecs
import numpy as np
import numpy.typing as npt
import pytest


def test_ioperator_value(
    array: ecs.Float32,
    other_value: float,
    iop: typing.Any,
) -> None:
    expected = array.numpy()
    iop(array, other_value)
    iop(expected, other_value)
    assert np.all(np.equal(array.numpy(), expected))


def test_ioperator_numpy(
    array: ecs.Float32,
    other_numpy: npt.NDArray[np.float32],
    iop: typing.Any,
) -> None:
    expected = array.numpy()
    iop(array, other_numpy)
    iop(expected, other_numpy)
    assert np.all(np.equal(array.numpy(), expected))


def test_ioperator_array(
    array: ecs.Float32,
    other_array: ecs.Float32,
    iop: typing.Any,
) -> None:
    expected = array.numpy()
    iop(array, other_array)
    iop(expected, other_array.numpy())
    assert np.all(np.equal(array.numpy(), expected))


def test_self(array: ecs.Float32, iop: typing.Any) -> None:
    with pytest.raises(TypeError):
        iop(array, array)


def test_self_slice(array: ecs.Float32, iop: typing.Any) -> None:
    expected = array.numpy()
    iop(array, array[:])
    iop(expected, expected)
    assert np.all(np.equal(array.numpy(), expected))


def test_self_slice_both_sides(array: ecs.Float32, iop: typing.Any) -> None:
    expected = array.numpy()
    iop(array[:], array[:])
    iop(expected, expected)
    assert np.all(np.equal(array.numpy(), expected))


def test_self_key() -> None:
    array = ecs.Float32.p_from_numpy(np.arange(5, dtype=np.float32))
    mask = np.array([True, False, False, True, False])
    array[mask] += array[mask]
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 6, 4]))


def test_works_with_mask() -> None:
    array = ecs.Float32.p_from_numpy(np.arange(5, dtype=np.float32))
    mask = np.array([True, False, False, True, False])
    array[mask] += np.array([10, 20], dtype=np.float32)
    assert np.all(np.equal(array.numpy(), [10, 1, 2, 23, 4]))


@pytest.fixture(
    params=(
        operator.iadd,
        # operator.isub,
        # operator.imul,
        # operator.itruediv,
        # operator.ifloordiv,
        # operator.imod,
        # operator.ipow,
    ),
)
def iop(request: pytest.FixtureRequest) -> typing.Any:
    return request.param


@pytest.fixture
def array() -> ecs.Float32:
    return ecs.Float32.p_from_numpy(np.arange(5, dtype=np.float32))


@pytest.fixture
def other_array() -> ecs.Float32:
    return ecs.Float32.p_from_numpy(np.arange(5, 10, dtype=np.float32))


@pytest.fixture
def other_numpy() -> npt.NDArray[np.float32]:
    return np.arange(5, 10, dtype=np.float32)


@pytest.fixture(
    params=(12, 12.5),
    ids=("int", "float"),
)
def other_value(request: pytest.FixtureRequest) -> float:
    return request.param