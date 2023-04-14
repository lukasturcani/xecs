import typing

import ecstasy as ecs
import pytest


class One(ecs.Component):
    x: ecs.Float64


@pytest.mark.benchmark(group="unrepeated-one-component-query")
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
        commands.spawn(components=(One,), num=request.param)

    def system(query: ecs.Query[tuple[One]]) -> None:
        (one,) = query.result()
        assert len(one) == request.param

    app = ecs.App.new()
    app.add_startup_system(startup_system)
    app.add_system(system)
    app.add_component_pool(One.create_pool(request.param))
    app.p_run_startup_systems()
    return app
