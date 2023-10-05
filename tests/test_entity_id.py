import pytest
import xecs as xx


class One(xx.Component):
    pass


class Two(xx.Component):
    pass


def test_entity_id(app: xx.RealTimeApp) -> None:
    app.update()


def system1(query: xx.Query[tuple[xx.EntityId, One]]) -> None:
    entity_id, _ = query.result()
    assert set(entity_id.value.numpy()) == {*range(5), *range(10, 15)}


def system2(query: xx.Query[tuple[xx.EntityId, Two]]) -> None:
    entity_id, _ = query.result()
    assert set(entity_id.value.numpy()) == {*range(5, 10), *range(10, 15)}


def system3(query: xx.Query[xx.EntityId]) -> None:
    entity_id = query.result()
    assert set(entity_id.value.numpy()) == set(range(15))


def spawn_entities(commands: xx.Commands) -> None:
    commands.spawn((One,), 5)
    commands.spawn((Two,), 5)
    commands.spawn((One, Two), 5)


@pytest.fixture
def app() -> xx.RealTimeApp:
    app = xx.RealTimeApp(num_entities=30)
    app.add_pool(One, 20)
    app.add_pool(Two, 10)
    app.add_startup_system(spawn_entities)
    app.add_system(system1)
    app.add_system(system2)
    app.add_system(system3)
    return app
