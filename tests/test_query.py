import ecstasy as ecs
import pytest


class One(ecs.Component):
    x: ecs.Float64


class Two(ecs.Component):
    y: ecs.Float64


def test_query_with_one_component(app: ecs.App) -> None:
    app.add_system(query_with_one_component)
    app.run()


def test_query_with_two_components(app: ecs.App) -> None:
    app.add_system(query_with_two_components)
    app.run()


def query_with_one_component(
    query_one: ecs.Query[tuple[One]],
    query_two: ecs.Query[tuple[Two]],
) -> None:
    (one,) = query_one.result()
    assert len(one) == 10

    (two,) = query_two.result()
    assert len(two) == 5


def query_with_two_components(query: ecs.Query[tuple[One, Two]]) -> None:
    one, two = query.result()
    assert len(one) == 5
    assert len(one) == len(two)


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
