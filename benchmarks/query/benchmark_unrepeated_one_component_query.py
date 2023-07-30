import typing

import pytest
import xecs as xx


class One(xx.Component):
    x: xx.Float32


@pytest.mark.benchmark(group="unrepeated-one-component-query")
def benchmark_query(benchmark: typing.Any, app: xx.App) -> None:
    app.add_system(system)
    benchmark(app.update)


def system(query: xx.Query[tuple[One]]) -> None:
    pass


@pytest.fixture(
    params=(10, 100, 1_000, 1_000_000),
    ids=("10", "100", "1_000", "1_000_000"),
)
def app(request: pytest.FixtureRequest) -> xx.App:
    def startup_system(commands: xx.Commands) -> None:
        commands.spawn(components=(One,), num=request.param)

    app = xx.App()
    app.add_startup_system(startup_system)
    app.add_pool(One.create_pool(request.param))
    app.update()
    return app
