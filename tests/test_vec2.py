import operator
import typing

import numpy as np
import numpy.typing as npt
import pytest
import xecs as xx


class VecContainer(xx.Component):
    vec: xx.Vec2


def test_numpy(vec1: xx.Vec2) -> None:
    generator = np.random.default_rng(54)
    expected = generator.random((2, 10), dtype=np.float32)
    vec1.fill(expected)
    assert np.all(np.equal(vec1.numpy(), expected))


@pytest.fixture(
    params=(
        operator.iadd,
        operator.isub,
        operator.imul,
        operator.itruediv,
        operator.ifloordiv,
        operator.imod,
        operator.ipow,
    )
)
def math_ioperator(request: pytest.FixtureRequest) -> typing.Any:
    return request.param


def test_ioperators_vec2(
    vec1: xx.Vec2,
    vec2: xx.Vec2,
    math_ioperator: typing.Any,
) -> None:
    expected = math_ioperator(vec1.numpy(), vec2.numpy())
    result = math_ioperator(vec1, vec2)
    assert np.allclose(result.numpy(), expected)


def test_ioperators_float(
    vec1: xx.Vec2,
    math_ioperator: typing.Any,
) -> None:
    expected = math_ioperator(vec1.numpy(), 2)
    result = math_ioperator(vec1, 2)
    assert np.all(np.equal(result.numpy(), expected))


def test_ioperators_array(
    vec1: xx.Vec2,
    array: npt.NDArray[np.float32],
    math_ioperator: typing.Any,
) -> None:
    expected = math_ioperator(vec1.numpy(), array)
    result = math_ioperator(vec1, array)
    assert np.allclose(result.numpy(), expected)


@pytest.fixture(
    params=(
        operator.add,
        operator.sub,
        operator.mul,
        operator.truediv,
        operator.floordiv,
        operator.mod,
        operator.pow,
    )
)
def math_operator(request: pytest.FixtureRequest) -> typing.Any:
    return request.param


def test_operators_vec2(
    vec1: xx.Vec2,
    vec2: xx.Vec2,
    math_operator: typing.Any,
) -> None:
    expected = math_operator(vec1.numpy(), vec2.numpy())
    result = math_operator(vec1, vec2)
    assert np.all(np.equal(result, expected))


def test_operators_array(
    vec1: xx.Vec2,
    array: npt.NDArray[np.float32],
    math_operator: typing.Any,
) -> None:
    expected = math_operator(vec1.numpy(), array)
    result = vec1 * array
    result = math_operator(vec1, array)
    assert np.all(np.equal(result, expected))


def test_angle_between_xy() -> None:
    assert np.allclose(
        xx.Vec2.from_xy(0, 1, 1).angle_between_xy(1, 0), [-np.pi / 2]
    )


def test_clamp_length() -> None:
    v1 = xx.Vec2.from_numpy(
        np.array(
            [
                [0.5, 6.0, 3.0, 5.0, 0.1],
                [0.0, 0.0, 0.0, 10.0, 0.05],
            ],
            dtype=np.float32,
        )
    )
    v1.clamp_length(1, 5)
    expected = [
        [1.0, 5.0, 3.0, 2.236068, 0.8944272],
        [0.0, 0.0, 0.0, 4.472136, 0.4472136],
    ]
    assert np.allclose(v1.numpy(), expected)


@pytest.fixture
def vec1() -> xx.Vec2:
    generator = np.random.default_rng(55)
    pool = VecContainer.create_pool(10)
    pool.p_spawn(10)
    pool.p_component.vec.fill(generator.random((2, 10), dtype=np.float32))
    return pool.p_component.vec


@pytest.fixture
def vec2() -> xx.Vec2:
    generator = np.random.default_rng(56)
    pool = VecContainer.create_pool(10)
    pool.p_spawn(10)
    pool.p_component.vec.fill(generator.random((2, 10), dtype=np.float32))
    return pool.p_component.vec


@pytest.fixture(
    params=(
        10,
        (2, 10),
    ),
)
def array(request: pytest.FixtureRequest) -> npt.NDArray[np.float32]:
    generator = np.random.default_rng(57)
    return generator.random(request.param, dtype=np.float32)
