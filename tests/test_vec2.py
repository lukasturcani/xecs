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
    xs = generator.random(10, dtype=np.float32)
    ys = generator.random(10, dtype=np.float32)
    all_mask = np.ones(10, dtype=np.bool_)
    vec1.x[all_mask] = xs
    vec1.y[all_mask] = ys
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
def math_ioperator(request: pytest.FixtureRequest) -> typing.Any:
    return request.param


def test_ioperators_vec2(
    vec1: xx.Vec2,
    vec2: xx.Vec2,
    math_ioperator: typing.Any,
) -> None:
    expected = math_ioperator(vec1.numpy(), vec2.numpy())
    result = math_ioperator(vec1, vec2)
    assert np.all(np.equal(result.numpy(), expected))


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
    assert np.all(np.equal(result.numpy(), expected))


# @pytest.fixture(
#     params=(
#         operator.add,
#         # operator.sub,
#         # operator.mul,
#         # operator.truediv,
#         # operator.floordiv,
#         # operator.mod,
#         # operator.pow,
#     )
# )
# def math_operator(request: pytest.FixtureRequest) -> typing.Any:
#     return request.param


# def test_operators_array(
#     vec1: xx.Vec2,
#     array: npt.NDArray[np.float32],
#     math_operator: typing.Any,
# ) -> None:
#     expected = math_operator(vec1.numpy(), array)
#     result = math_operator(vec1, array)
#     assert np.all(np.equal(result, expected))


@pytest.fixture
def vec1() -> xx.Vec2:
    generator = np.random.default_rng(55)
    pool = VecContainer.create_pool(10)
    pool.p_spawn(10)
    all_mask = np.ones(10, dtype=np.bool_)
    pool.p_component.vec.x[all_mask] = generator.random(10, dtype=np.float32)
    pool.p_component.vec.y[all_mask] = generator.random(10, dtype=np.float32)
    return pool.p_component.vec


@pytest.fixture
def vec2() -> xx.Vec2:
    generator = np.random.default_rng(56)
    pool = VecContainer.create_pool(10)
    pool.p_spawn(10)
    all_mask = np.ones(10, dtype=np.bool_)
    pool.p_component.vec.x[all_mask] = generator.random(10, dtype=np.float32)
    pool.p_component.vec.y[all_mask] = generator.random(10, dtype=np.float32)
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
