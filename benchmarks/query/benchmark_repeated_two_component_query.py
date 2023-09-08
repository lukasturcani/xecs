"""
Benchmarks for a query of two components.

The purpose of these benchmarks is to show how the runtime of a
query with more than one component changes when it is executed
multiple times.
"""

import typing

import pytest
import xecs as xx


class One(xx.Component):
    x: xx.Float32


class Two(xx.Component):
    x: xx.Float32


@pytest.mark.benchmark(group="repeated-two-component-query")
def benchmark_query(benchmark: typing.Any, app: xx.RealTimeApp) -> None:
    app.add_system(system)
    benchmark(app.update)


def system(
    query1: xx.Query[tuple[One, Two]],
    query2: xx.Query[tuple[One, Two]],
) -> None:
    pass


@pytest.fixture(
    params=(10, 100, 1_000, 1_000_000),
    ids=("10", "100", "1_000", "1_000_000"),
)
def app(request: pytest.FixtureRequest) -> xx.RealTimeApp:
    def startup_system(commands: xx.Commands) -> None:
        commands.spawn(components=(One,), num=5)
        commands.spawn(components=(Two,), num=5)
        commands.spawn(components=(One, Two), num=request.param - 10)

    app = xx.RealTimeApp()
    app.add_startup_system(startup_system)
    app.add_pool(One.create_pool(request.param))
    app.add_pool(Two.create_pool(request.param))
    app.update()
    return app
