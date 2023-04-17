import operator
import typing

import ecstasy as ecs
import numpy as np
import numpy.typing as npt
import pytest


class VecContainer(ecs.Component):
    vec: ecs.Vec2


def test_numpy(vec1: ecs.Vec2) -> None:
    generator = np.random.default_rng(54)
    xs = generator.random(10, dtype=np.float32)
    ys = generator.random(10, dtype=np.float32)
    vec1.x[:] = xs
    vec1.y[:] = ys
    assert np.all(
        np.equal(
            vec1.numpy(),
            np.array([xs, ys], dtype=np.float32),
        ),
    )


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
def math_operator(request: pytest.FixtureRequest) -> typing.Any:
    return request.param


def test_operators_vec2(
    vec1: ecs.Vec2,
    vec2: ecs.Vec2,
    math_operator: typing.Any,
) -> None:
    array1 = vec1.numpy()
    array2 = vec2.numpy()
    expected = math_operator(array1, array2)
    result = math_operator(vec1, vec2)
    assert np.all(np.equal(result.numpy(), expected))


def test_operators_float(
    vec1: ecs.Vec2,
    math_operator: typing.Any,
) -> None:
    array1 = vec1.numpy()
    expected = math_operator(array1, 3)
    result = math_operator(vec1, 3)
    assert np.all(np.equal(result.numpy(), expected))


def test_operators_array(
    vec1: ecs.Vec2,
    array: npt.NDArray[np.float32],
    math_operator: typing.Any,
) -> None:
    array1 = vec1.numpy()
    expected = math_operator(array1, array)
    result = math_operator(vec1, array)
    assert np.all(np.equal(result.numpy(), expected))


@pytest.fixture
def vec1() -> ecs.Vec2:
    generator = np.random.default_rng(55)
    pool = VecContainer.create_pool(10)
    pool.p_spawn(10)
    pool.p_component.vec.x[:] = generator.random(10, dtype=np.float32)
    pool.p_component.vec.y[:] = generator.random(10, dtype=np.float32)
    return pool.p_component.vec


@pytest.fixture
def vec2() -> ecs.Vec2:
    generator = np.random.default_rng(56)
    pool = VecContainer.create_pool(10)
    pool.p_spawn(10)
    pool.p_component.vec.x[:] = generator.random(10, dtype=np.float32)
    pool.p_component.vec.y[:] = generator.random(10, dtype=np.float32)
    return pool.p_component.vec


@pytest.fixture
def array() -> npt.NDArray[np.float32]:
    generator = np.random.default_rng(57)
    return generator.random(10, dtype=np.float32)
