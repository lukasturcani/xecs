import itertools

import numpy as np
import pytest
import xecs as xx


class One(xx.Component):
    x: xx.Float32


class Two(xx.Component):
    y: xx.Float32


class Three(xx.Component):
    z: xx.Float32


def test_combinations_2_one_component(app: xx.App) -> None:
    app.add_system(get_combinations_one_component)
    app.update()


def test_combinations_2_two_components(app: xx.App) -> None:
    app.add_system(get_combinations_two_components)
    app.update()


def get_combinations_one_component(query: xx.Query[tuple[One]]) -> None:
    (one1,), (one2,) = query.combinations_2()
    result = one1.x + one2.x
    result.sort()
    assert np.all(np.equal(result, [3, 4, 5, 5, 6, 7]))


def get_combinations_two_components(
    query: xx.Query[tuple[Two, Three]],
) -> None:
    (two1, three1), (two2, three2) = query.combinations_2()
    sums1 = two1.y + two2.y
    sums1.sort()
    assert np.all(np.equal(sums1, [3, 4, 5, 5, 6, 7]))

    sums2 = three1.z + three2.z
    sums2.sort()
    assert np.all(np.equal(sums2, [30, 40, 50, 50, 60, 70]))


def spawn_entities(world: xx.World, commands: xx.Commands) -> None:
    (onei,) = commands.spawn((One,), 4)
    world.get_view(One, onei).x.fill([1, 2, 3, 4])
    (twoi, threei) = commands.spawn((Two, Three), 4)
    world.get_view(Two, twoi).y.fill([1, 2, 3, 4])
    world.get_view(Three, threei).z.fill([10, 20, 30, 40])


@pytest.fixture
def app() -> xx.App:
    app = xx.App()
    app.add_pool(One.create_pool(10))
    app.add_pool(Two.create_pool(10))
    app.add_pool(Three.create_pool(10))
    app.add_startup_system(spawn_entities)
    return app