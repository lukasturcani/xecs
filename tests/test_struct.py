import pytest
import xecs as xx


class MyStruct(xx.Struct):
    x: xx.Int


class MyComponent(xx.Component):
    s: MyStruct


def test_to_str(app: xx.RealTimeApp) -> None:
    app.add_system(to_str)
    app.update()


def to_str(q: xx.Query[MyComponent]) -> None:
    component = q.result()
    assert (
        repr(component)
        == str(component)
        == (
            "<MyComponent(\n    "
            "s=<MyStruct(\n        x=<xecs.Int32 [0, 0]>,\n    )>,\n)>"
        )
    )


@pytest.fixture
def app() -> xx.RealTimeApp:
    app = xx.RealTimeApp(num_entities=2)
    app.add_pool(MyComponent, 2)
    app.add_startup_system(spawn_entities)
    return app


def spawn_entities(commands: xx.Commands) -> None:
    commands.spawn((MyComponent,), 2)
