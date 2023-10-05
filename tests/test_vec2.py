import operator
import typing

import numpy as np
import numpy.typing as npt
import pytest
import xecs as xx


class VecContainer(xx.Component):
    vec1: xx.Vec2
    vec2: xx.Vec2


class Generator(xx.Resource):
    value: np.random.Generator


@pytest.fixture
def app() -> xx.RealTimeApp:
    app = xx.RealTimeApp(num_entities=5)
    app.add_pool(VecContainer, 5)
    app.add_resource(Generator(np.random.default_rng(11)))
    app.add_startup_system(spawn_entities)
    return app


def spawn_entities(
    generator: Generator, commands: xx.Commands, world: xx.World
) -> None:
    (containeri,) = commands.spawn((VecContainer,), 5)
    container = world.get_view(VecContainer, containeri)
    container.vec1.fill(generator.value.random((2, 5), dtype=np.float32))
    container.vec2.fill(generator.value.random((2, 5), dtype=np.float32))


def test_numpy(app: xx.RealTimeApp) -> None:
    app.add_system(numpy)
    app.update()


def numpy(q: xx.Query[VecContainer]) -> None:
    vec = q.result().vec1
    generator = np.random.default_rng(54)
    expected = generator.random((2, 5), dtype=np.float32)
    vec.fill(expected)
    assert np.all(np.equal(vec.numpy(), expected))


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
    app: xx.RealTimeApp,
    math_ioperator: typing.Any,
) -> None:
    def ioperators_vec2(
        q: xx.Query[VecContainer],
    ) -> None:
        container = q.result()
        expected = math_ioperator(
            container.vec1.numpy(), container.vec2.numpy()
        )
        result = math_ioperator(container.vec1, container.vec2)
        assert np.allclose(result.numpy(), expected)

    app.add_system(ioperators_vec2)
    app.update()


def test_ioperators_float(
    app: xx.RealTimeApp,
    math_ioperator: typing.Any,
) -> None:
    def ioperators_float(
        q: xx.Query[VecContainer],
    ) -> None:
        container = q.result()
        expected = math_ioperator(container.vec1.numpy(), 2)
        result = math_ioperator(container.vec1, 2)
        assert np.all(np.equal(result.numpy(), expected))

    app.add_system(ioperators_float)
    app.update()


def test_ioperators_array(
    app: xx.RealTimeApp,
    array: npt.NDArray[np.float32],
    math_ioperator: typing.Any,
) -> None:
    def ioperators_array(
        q: xx.Query[VecContainer],
    ) -> None:
        container = q.result()
        expected = math_ioperator(container.vec1.numpy(), array)
        result = math_ioperator(container.vec1, array)
        assert np.allclose(result.numpy(), expected)

    app.add_system(ioperators_array)
    app.update()


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
    app: xx.RealTimeApp,
    math_operator: typing.Any,
) -> None:
    def operators_vec2(
        q: xx.Query[VecContainer],
    ) -> None:
        container = q.result()
        expected = math_operator(
            container.vec1.numpy(), container.vec2.numpy()
        )
        result = math_operator(container.vec1, container.vec2)
        assert np.all(np.equal(result, expected))

    app.add_system(operators_vec2)
    app.update()


def test_operators_array(
    app: xx.RealTimeApp,
    array: npt.NDArray[np.float32],
    math_operator: typing.Any,
) -> None:
    def operators_array(
        q: xx.Query[VecContainer],
    ) -> None:
        container = q.result()
        expected = math_operator(container.vec1.numpy(), array)
        result = container.vec1 * array
        result = math_operator(container.vec1, array)
        assert np.all(np.equal(result, expected))

    app.add_system(operators_array)
    app.update()


def test_angle_between_xy(app: xx.RealTimeApp) -> None:
    app.add_system(angle_between_xy)
    app.update()


def angle_between_xy(q: xx.Query[VecContainer]) -> None:
    container = q.result()
    container.vec1.x.fill(0)
    container.vec1.y.fill(1)
    assert np.allclose(container.vec1.angle_between_xy(1, 0), -np.pi / 2)


def test_clamp_length(
    app: xx.RealTimeApp,
) -> None:
    app.add_system(clamp_length)
    app.update()


def clamp_length(q: xx.Query[VecContainer]) -> None:
    v1 = q.result().vec1
    v1.fill(
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


@pytest.fixture(
    params=(
        5,
        (2, 5),
    ),
)
def array(request: pytest.FixtureRequest) -> npt.NDArray[np.float32]:
    generator = np.random.default_rng(57)
    return generator.random(request.param, dtype=np.float32)
