import ecstasy as ecs
import pytest


class One(ecs.Component):
    x: ecs.Float32


class Two(ecs.Component):
    y: ecs.Float32


class Params(ecs.Resource):
    z: str


def test_query_with_one_component(app: ecs.App) -> None:
    app.add_system(query_with_one_component)
    app.update()


def test_query_with_two_components(app: ecs.App) -> None:
    app.add_system(query_with_two_components)
    app.update()


def test_system_with_resource(app: ecs.App) -> None:
    app.add_system(system_with_resource)
    app.add_resource(Params("hi"))
    app.update()


def query_with_one_component(
    query_one: ecs.Query[tuple[One]],
    query_two: ecs.Query[tuple[Two]],
) -> None:
    (one,) = query_one.result()
    assert isinstance(one, One)
    assert len(one) == 10

    (two,) = query_two.result()
    assert isinstance(two, Two)
    assert len(two) == 5


def query_with_two_components(query: ecs.Query[tuple[One, Two]]) -> None:
    one, two = query.result()
    assert len(one) == len(two) == 5
    assert isinstance(one, One)
    assert isinstance(two, Two)


def system_with_resource(params: Params, query: ecs.Query[tuple[One]]) -> None:
    (one,) = query.result()
    assert isinstance(one, One)
    assert len(one) == 10
    assert params.z == "hi"


def spawn_entities(commands: ecs.Commands) -> None:
    commands.spawn((One,), 5)
    commands.spawn((One, Two), 5)


@pytest.fixture
def app() -> ecs.App:
    app = ecs.App.new()
    app.add_component_pool(One.create_pool(10))
    app.add_component_pool(Two.create_pool(5))
    app.add_startup_system(spawn_entities)
    return app