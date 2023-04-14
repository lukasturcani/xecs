import typing

import ecstasy as ecs
import pytest


class One(ecs.Component):
    x: ecs.Float64


class Two(ecs.Component):
    x: ecs.Float64


class Three(ecs.Component):
    x: ecs.Float64


class Four(ecs.Component):
    x: ecs.Float64


class Five(ecs.Component):
    x: ecs.Float64


@pytest.mark.benchmark(group="unrepeated-five-component-query")
def benchmark_query(
    benchmark: typing.Any,
    app: ecs.App,
) -> None:
    benchmark(app.p_run_systems)


@pytest.fixture(
    params=(10, 100, 1_000, 1_000_000),
    ids=("10", "100", "1_000", "1_000_000"),
)
def app(request: pytest.FixtureRequest) -> ecs.App:
    def startup_system(commands: ecs.Commands) -> None:
        commands.spawn(
            components=(One, Two, Three, Four, Five),
            num=request.param,
        )

    def system(query: ecs.Query[tuple[One, Two, Three, Four, Five]]) -> None:
        result = query.result()
        assert len(result[0]) == request.param

    app = ecs.App.new()
    app.add_startup_system(startup_system)
    app.add_system(system)
    app.add_component_pool(One.create_pool(request.param))
    app.add_component_pool(Two.create_pool(request.param))
    app.add_component_pool(Three.create_pool(request.param))
    app.add_component_pool(Four.create_pool(request.param))
    app.add_component_pool(Five.create_pool(request.param))
    app.p_run_startup_systems()
    return app
