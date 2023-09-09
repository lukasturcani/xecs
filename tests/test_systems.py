import numpy as np
import pytest
import xecs as xx


class One(xx.Component):
    x: xx.Float32


class Two(xx.Component):
    y: xx.Float32


class Params(xx.Resource):
    z: str


def test_query_with_one_component(app: xx.RealTimeApp) -> None:
    app.add_system(query_with_one_component)
    app.update()


def test_query_with_two_components(app: xx.RealTimeApp) -> None:
    app.add_system(query_with_two_components)
    app.update()


def test_system_with_resource(app: xx.RealTimeApp) -> None:
    app.add_system(system_with_resource)
    app.add_resource(Params("hi"))
    app.update()


def test_spawning(app: xx.RealTimeApp) -> None:
    app.add_system(spawning_sytem)
    app.update()


def query_with_one_component(
    query_one: xx.Query[tuple[One]],
    query_two: xx.Query[Two],
) -> None:
    (one,) = query_one.result()
    assert isinstance(one, One)
    assert len(one) == 10

    two = query_two.result()
    assert isinstance(two, Two)
    assert len(two) == 5


def query_with_two_components(query: xx.Query[tuple[One, Two]]) -> None:
    one, two = query.result()
    assert len(one) == len(two) == 5
    assert isinstance(one, One)
    assert isinstance(two, Two)


def system_with_resource(params: Params, query: xx.Query[tuple[One]]) -> None:
    (one,) = query.result()
    assert isinstance(one, One)
    assert len(one) == 10
    assert params.z == "hi"


def spawning_sytem(world: xx.World, commands: xx.Commands) -> None:
    (one_indices, two_indices) = commands.spawn((One, Two), 2)
    one = world.get_view(One, one_indices)
    two = world.get_view(Two, two_indices)
    one.x.fill([10, 20])
    two.y.fill([30, 40])

    all_ones = world.get_view(One)
    expected_ones = np.zeros(12, dtype=np.float32)
    expected_ones[[10, 11]] = [10, 20]
    assert np.all(all_ones.x == expected_ones)

    all_twos = world.get_view(Two)
    expected_twos = np.zeros(7, dtype=np.float32)
    expected_twos[[5, 6]] = [30, 40]
    assert np.all(all_twos.y == expected_twos)


def spawn_entities(commands: xx.Commands) -> None:
    commands.spawn((One,), 5)
    commands.spawn((One, Two), 5)


@pytest.fixture
def app() -> xx.RealTimeApp:
    app = xx.RealTimeApp()
    app.add_pool(One.create_pool(20))
    app.add_pool(Two.create_pool(10))
    app.add_startup_system(spawn_entities)
    return app
