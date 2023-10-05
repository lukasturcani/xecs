import numpy as np
import pytest
import xecs as xx


class One(xx.Component):
    x: xx.Float32


def test_despawn(app: xx.RealTimeApp) -> None:
    app.update()


def spawn_entities(commands: xx.Commands, world: xx.World) -> None:
    (onei,) = commands.spawn((One,), 5)
    one = world.get_view(One, onei)
    one.x.fill(np.arange(5, dtype=np.float32))


def system1(
    commands: xx.Commands,
    one_query: xx.Query[tuple[xx.EntityId, One]],
) -> None:
    entity_id, one = one_query.result()
    assert set(entity_id.value.numpy()) == {0, 1, 2, 3, 4}
    commands.despawn(entity_id[one.x < 3])


def system2(
    commands: xx.Commands,
    one_query: xx.Query[tuple[xx.EntityId, One]],
) -> None:
    entity_id, _ = one_query.result()
    assert set(entity_id.value.numpy()) == {3, 4}
    commands.spawn((One,), 3)


def system3(
    one_query: xx.Query[tuple[xx.EntityId, One]],
) -> None:
    entity_id, _ = one_query.result()
    assert set(entity_id.value.numpy()) == {0, 1, 2, 3, 4}


@pytest.fixture
def app() -> xx.RealTimeApp:
    app = xx.RealTimeApp(num_entities=5)
    app.add_pool(One.create_pool(5))
    app.add_startup_system(spawn_entities)
    app.add_system(system1)
    app.add_system(system2)
    app.add_system(system3)
    return app
