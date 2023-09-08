import numpy as np
import pytest
import xecs as xx


class Thing(xx.Component):
    position: xx.Float32
    num_neighbors: xx.Float32


def test_getting_neighbors_one_component(app: xx.RealTimeApp) -> None:
    app.add_system(get_neighbors_one_component)
    app.update()


def get_neighbors_one_component(query: xx.Query[tuple[Thing]]) -> None:
    (thing1,), (thing2,) = query.product_2()
    displacement = thing1.position - thing2.position
    distance = abs(displacement)
    is_neighbor = distance <= 1.1
    thing1[is_neighbor].num_neighbors += 1
    (thing,) = query.result()
    assert np.all(
        np.equal(
            sorted(thing.num_neighbors.numpy()),
            [1, 1, 1, 1, 2],
        ),
    )


class First(xx.Component):
    position: xx.Float32


class Second(xx.Component):
    num_neighbors: xx.Float32


def test_getting_neighbors_two_components(app: xx.RealTimeApp) -> None:
    app.add_system(get_neighbors_two_components)
    app.update()


def get_neighbors_two_components(
    query: xx.Query[tuple[First, Second]],
) -> None:
    (first1, second1), (first2, second2) = query.product_2()
    displacement = first1.position - first2.position
    distance = abs(displacement)
    is_neighbor = distance <= 1.1
    second1[is_neighbor].num_neighbors += 1
    (first, second) = query.result()
    assert np.all(
        np.equal(
            sorted(second.num_neighbors.numpy()),
            [1, 1, 1, 1, 2],
        ),
    )


def spawn_entities(world: xx.World, commands: xx.Commands) -> None:
    (thingi,) = commands.spawn((Thing,), 5)
    world.get_view(Thing, thingi).position.fill([1, 2, 8, 9, 10])

    (firsti, _) = commands.spawn((First, Second), 5)
    first = world.get_view(First, firsti)
    first.position.fill([1, 2, 8, 9, 10])


@pytest.fixture
def app() -> xx.RealTimeApp:
    app = xx.RealTimeApp()
    app.add_pool(Thing.create_pool(10))
    app.add_pool(First.create_pool(10))
    app.add_pool(Second.create_pool(10))
    app.add_startup_system(spawn_entities)
    return app
