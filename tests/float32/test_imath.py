import operator
import typing

import numpy as np
import numpy.typing as npt
import pytest
import xecs as xx


def test_ioperator_value(
    array: xx.Float32,
    other_value: float,
    iop: typing.Any,
) -> None:
    expected = iop(array.numpy(), other_value)
    iop(array, other_value)
    assert np.all(np.equal(array.numpy(), expected))


def test_ioperator_numpy(
    array: xx.Float32,
    other_numpy: npt.NDArray[np.float32],
    iop: typing.Any,
) -> None:
    expected = iop(array.numpy(), other_numpy)
    iop(array, other_numpy)
    assert np.all(np.equal(array.numpy(), expected))


def test_ioperator_array(
    array: xx.Float32,
    other_array: xx.Float32,
    iop: typing.Any,
) -> None:
    expected = iop(array.numpy(), other_array.numpy())
    iop(array, other_array)
    assert np.all(np.equal(array.numpy(), expected))


def test_ioperator_list(
    array: xx.Float32,
    other_list: list[float],
    iop: typing.Any,
) -> None:
    expected = iop(array.numpy(), other_list)
    iop(array, other_list)
    assert np.all(np.equal(array.numpy(), expected))


def test_self(array: xx.Float32, iop: typing.Any) -> None:
    expected = iop(array.numpy(), array.numpy())
    result = iop(array, array)
    assert np.all(np.equal(result, expected))


def test_self_mask(array: xx.Float32, iop: typing.Any) -> None:
    expected = iop(array.numpy(), array.numpy())
    all_mask = np.ones(len(array), dtype=np.bool_)
    iop(array, array[all_mask])
    assert np.all(np.equal(array.numpy(), expected))


def test_self_slice_both_sides(array: xx.Float32, iop: typing.Any) -> None:
    expected = iop(array.numpy(), array.numpy())
    all_mask = np.ones(len(array), dtype=np.bool_)
    iop(array[all_mask], array[all_mask])
    assert np.all(np.equal(array.numpy(), expected))


def test_self_key() -> None:
    array = xx.Float32.p_from_numpy(np.arange(5, dtype=np.float32))
    mask = np.array([True, False, False, True, False])
    array[mask] += array[mask]
    assert np.all(np.equal(array.numpy(), [0, 1, 2, 6, 4]))


def test_works_with_mask() -> None:
    array = xx.Float32.p_from_numpy(np.arange(5, dtype=np.float32))
    mask = np.array([True, False, False, True, False])
    array[mask] += np.array([10, 20], dtype=np.float32)
    assert np.all(np.equal(array.numpy(), [10, 1, 2, 23, 4]))


@pytest.fixture(
    params=(
        operator.iadd,
        operator.isub,
        operator.imul,
        operator.itruediv,
        operator.ifloordiv,
        operator.imod,
        operator.ipow,
    ),
)
def iop(request: pytest.FixtureRequest) -> typing.Any:
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
