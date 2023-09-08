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

import pytest
import xecs as xx


class One(xx.Component):
    x: xx.Float32


class Two(xx.Component):
    x: xx.Float32


@pytest.mark.benchmark(
    group="unrepeated-two-component-query-fixed-overlap-one-grows",
)
def benchmark_one_component_grows_but_overlap_constant(
    benchmark: typing.Any,
    fixed_overlap_app_one_grows: xx.RealTimeApp,
) -> None:
    fixed_overlap_app_one_grows.add_system(system)
    benchmark(fixed_overlap_app_one_grows.update)


@pytest.mark.benchmark(
    group="unrepeated-two-component-query-fixed-overlap-both-grow",
)
def benchmark_both_components_grow_but_overlap_constant(
    benchmark: typing.Any,
    fixed_overlap_app_both_grow: xx.RealTimeApp,
) -> None:
    fixed_overlap_app_both_grow.add_system(system)
    benchmark(fixed_overlap_app_both_grow.update)


@pytest.mark.benchmark(
    group="unrepeated-two-component-query-increasing-overlap",
)
def benchmark_overlap_increases(
    benchmark: typing.Any,
    increasing_overlap_app: xx.RealTimeApp,
) -> None:
    increasing_overlap_app.add_system(system)
    benchmark(increasing_overlap_app.update)


def system(query: xx.Query[tuple[One, Two]]) -> None:
    pass


@pytest.fixture(
    params=(10, 100, 1_000, 1_000_000, 100_000_000),
    ids=("10", "100", "1_000", "1_000_000", "100_000_000"),
)
def fixed_overlap_app_one_grows(
    request: pytest.FixtureRequest,
) -> xx.RealTimeApp:
    def startup_system(commands: xx.Commands) -> None:
        commands.spawn(components=(One,), num=request.param - 5)
        commands.spawn(components=(Two,), num=5)
        commands.spawn(components=(One, Two), num=5)

    app = xx.RealTimeApp()
    app.add_startup_system(startup_system)
    app.add_pool(One.create_pool(request.param))
    app.add_pool(Two.create_pool(request.param))
    app.update()
    return app


@pytest.fixture(
    params=(10, 100, 1_000, 1_000_000),
    ids=("10", "100", "1_000", "1_000_000"),
)
def fixed_overlap_app_both_grow(
    request: pytest.FixtureRequest,
) -> xx.RealTimeApp:
    def startup_system(commands: xx.Commands) -> None:
        commands.spawn(components=(One,), num=request.param - 5)
        commands.spawn(components=(Two,), num=request.param - 5)
        commands.spawn(components=(One, Two), num=5)

    app = xx.RealTimeApp()
    app.add_startup_system(startup_system)
    app.add_pool(One.create_pool(request.param))
    app.add_pool(Two.create_pool(request.param))
    app.update()
    return app


@pytest.fixture(
    params=(10, 100, 1_000, 1_000_000),
    ids=("10", "100", "1_000", "1_000_000"),
)
def increasing_overlap_app(request: pytest.FixtureRequest) -> xx.RealTimeApp:
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
