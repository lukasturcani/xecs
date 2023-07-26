"""
Benchmarks for a query of two components.

The purpose of of these benchmarks is to show how the runtime of a
two component query changes with regard to the number of components.
There are mutiple scenarios which need to be analyzed:

1. The number of entities which match the query grows.
2. The number of entities which match the query does not grow but the
   size of one of the component pools does grow.
3. The number of entities which match the query does not grow but the
   size of both of the component pools does grow.

"""

import typing

import ecstasy as ecs
import pytest


class One(ecs.Component):
    x: ecs.Float32


class Two(ecs.Component):
    x: ecs.Float32


@pytest.mark.benchmark(
    group="unrepeated-two-component-query-fixed-overlap-one-grows",
)
def benchmark_one_component_grows_but_overlap_constant(
    benchmark: typing.Any,
    fixed_overlap_app_one_grows: ecs.App,
) -> None:
    benchmark(fixed_overlap_app_one_grows.p_run_systems)


@pytest.mark.benchmark(
    group="unrepeated-two-component-query-fixed-overlap-both-grow",
)
def benchmark_both_components_grow_but_overlap_constant(
    benchmark: typing.Any,
    fixed_overlap_app_both_grow: ecs.App,
) -> None:
    benchmark(fixed_overlap_app_both_grow.p_run_systems)


@pytest.mark.benchmark(
    group="unrepeated-two-component-query-increasing-overlap",
)
def benchmark_overlap_increases(
    benchmark: typing.Any,
    increasing_overlap_app: ecs.App,
) -> None:
    benchmark(increasing_overlap_app.p_run_systems)


def system(query: ecs.Query[tuple[One, Two]]) -> None:
    pass


@pytest.fixture(
    params=(10, 100, 1_000, 1_000_000, 100_000_000),
    ids=("10", "100", "1_000", "1_000_000", "100_000_000"),
)
def fixed_overlap_app_one_grows(request: pytest.FixtureRequest) -> ecs.App:
    def startup_system(commands: ecs.Commands) -> None:
        commands.spawn(components=(One,), num=request.param - 5)
        commands.spawn(components=(Two,), num=5)
        commands.spawn(components=(One, Two), num=5)

    app = ecs.App()
    app.add_startup_system(startup_system)
    app.add_system(system)
    app.add_component_pool(One.create_pool(request.param))
    app.add_component_pool(Two.create_pool(request.param))
    app.p_run_startup_systems()
    return app


@pytest.fixture(
    params=(10, 100, 1_000, 1_000_000),
    ids=("10", "100", "1_000", "1_000_000"),
)
def fixed_overlap_app_both_grow(request: pytest.FixtureRequest) -> ecs.App:
    def startup_system(commands: ecs.Commands) -> None:
        commands.spawn(components=(One,), num=request.param - 5)
        commands.spawn(components=(Two,), num=request.param - 5)
        commands.spawn(components=(One, Two), num=5)

    app = ecs.App()
    app.add_startup_system(startup_system)
    app.add_system(system)
    app.add_component_pool(One.create_pool(request.param))
    app.add_component_pool(Two.create_pool(request.param))
    app.p_run_startup_systems()
    return app


@pytest.fixture(
    params=(10, 100, 1_000, 1_000_000),
    ids=("10", "100", "1_000", "1_000_000"),
)
def increasing_overlap_app(request: pytest.FixtureRequest) -> ecs.App:
    def startup_system(commands: ecs.Commands) -> None:
        commands.spawn(components=(One,), num=5)
        commands.spawn(components=(Two,), num=5)
        commands.spawn(components=(One, Two), num=request.param - 10)

    app = ecs.App()
    app.add_startup_system(startup_system)
    app.add_system(system)
    app.add_component_pool(One.create_pool(request.param))
    app.add_component_pool(Two.create_pool(request.param))
    app.p_run_startup_systems()
    return app
