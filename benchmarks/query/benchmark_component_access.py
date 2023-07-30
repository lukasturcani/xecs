"""
Benchmark how fast component access is.

The purpose of this benchmark is to check that
components matching a query are access in a performant way.
For example, if a query was to produce components which are
accessed in a random way, we may get bad cache use. However
in order to access components in order, we may need to sort
indices we're accessing. These benchmarks exist to check
if we're making good trade-offs.
"""


import typing

import pytest
import xecs as xx


class One(xx.Component):
    x: xx.Float32


@pytest.mark.benchmark(group="component-access")
def benchmark_component_access(benchmark: typing.Any, app: xx.App) -> None:
    app.add_system(add)
    app.update()
    query = app.world.get_resource(xx.Systems).systems[0].query_args["query"]
    benchmark(add, query)


@pytest.fixture
def app() -> xx.App:
    app = xx.App()
    app.add_startup_system(spawn)
    app.add_pool(One.create_pool(10_000))
    app.update()
    return app


def spawn(world: xx.World, commands: xx.Commands) -> None:
    (onei,) = commands.spawn((One,), 10_000)
    world.get_view(One, onei).x.fill(list(range(10_000)))


def add(query: xx.Query[tuple[One]]) -> None:
    (one,) = query.result()
    one.x += 12
    one.x -= 12
