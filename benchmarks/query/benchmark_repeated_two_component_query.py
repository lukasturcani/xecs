"""
Benchmarks for a query of two components.

The purpose of these benchmarks is to show how the runtime of a
query with more than one component changes when it is executed
multiple times.

"""
import typing

import ecstasy as ecs
import pytest


class One(ecs.Component):
    x: ecs.Float32


class Two(ecs.Component):
    x: ecs.Float32


@pytest.mark.benchmark(group="repeated-two-component-query")
def benchmark_query(
    benchmark: typing.Any,
    app: ecs.App,
) -> None:
    benchmark(app.p_run_systems)


def system(
    query1: ecs.Query[tuple[One, Two]],
    query2: ecs.Query[tuple[One, Two]],
) -> None:
    pass


@pytest.fixture(
    params=(10, 100, 1_000, 1_000_000),
    ids=("10", "100", "1_000", "1_000_000"),
)
def app(request: pytest.FixtureRequest) -> ecs.App:
    def startup_system(commands: ecs.Commands) -> None:
        commands.spawn(components=(One,), num=5)
        commands.spawn(components=(Two,), num=5)
        commands.spawn(components=(One, Two), num=request.param - 10)

    app = ecs.App.new()
    app.add_startup_system(startup_system)
    app.add_system(system)
    app.add_component_pool(One.create_pool(request.param))
    app.add_component_pool(Two.create_pool(request.param))
    app.p_run_startup_systems()
    return app
