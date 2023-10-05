from typing import TypeVar

import numpy as np
import pytest
import xecs as xx

T = TypeVar("T", bound=xx.Component | xx.Struct)


def get(component: T, indices: list[int]) -> T:
    mask = np.zeros(len(component), dtype=np.bool_)
    mask[indices] = True
    x = component[mask]
    assert isinstance(x, type(component))
    return x


class StructA(xx.Struct):
    a: xx.Float32


class StructB(xx.Struct):
    b: xx.Float32
    c: StructA
    h: xx.PyField[str] = xx.py_field(default="hello")


class MyComponent(xx.Component):
    d: xx.Float32
    e: StructA
    f: StructB
    g: xx.PyField[str] = xx.py_field(default="world")


@pytest.fixture
def app() -> xx.RealTimeApp:
    app = xx.RealTimeApp(num_entities=20)
    app.add_pool(MyComponent, 10)
    app.add_pool(ComponentWithDefaults, 10)
    app.add_startup_system(spawn_entities)
    return app


def spawn_entities(commands: xx.Commands) -> None:
    commands.spawn((MyComponent,), 10)
    commands.spawn((ComponentWithDefaults,), 10)


def test_struct_getitem_creates_shared_view(app: xx.RealTimeApp) -> None:
    app.add_system(struct_getitem_creates_shared_view)
    app.update()


def struct_getitem_creates_shared_view(
    component_query: xx.Query[MyComponent],
) -> None:
    component = component_query.result()
    struct = component.f
    assert len(struct) == 10
    assert len(struct.b) == 10
    assert len(struct.c) == 10
    assert len(struct.c.a) == 10
    sub_view = get(struct, list(range(5)))
    assert len(sub_view) == 5
    assert len(sub_view.b) == 5
    assert len(sub_view.c) == 5
    assert len(sub_view.c.a) == 5

    assert np.sum(struct.c.a.numpy()) == 0
    assert np.sum(sub_view.c.a.numpy()) == 0
    all_mask = np.ones(len(sub_view), dtype=np.bool_)
    sub_view.c.a[all_mask] = 1
    assert np.sum(struct.c.a.numpy()) == 5
    assert np.sum(sub_view.c.a.numpy()) == 5


def test_py_field_default_value_is_used(app: xx.RealTimeApp) -> None:
    app.add_system(py_field_default_value_is_used)
    app.update()


def py_field_default_value_is_used(
    query: xx.Query[MyComponent],
) -> None:
    component = query.result()
    assert component.f.h.get(6) == "hello"
    assert component.g.get(6) == "world"


class StructWithDefaults(xx.Struct):
    a: xx.Float = xx.float_(default=1.0)
    b: xx.Float32 = xx.float32(default=2.0)
    c: xx.Int = xx.int_(default=3)
    d: xx.Int32 = xx.int32(default=4)
    e: xx.Bool = xx.bool_(default=True)


class ComponentWithDefaults(xx.Component):
    a: xx.Float = xx.float_(default=1.0)
    b: xx.Float32 = xx.float32(default=2.0)
    c: xx.Int = xx.int_(default=3)
    d: xx.Int32 = xx.int32(default=4)
    e: xx.Bool = xx.bool_(default=True)
    f: StructWithDefaults


def test_default_values_get_used(app: xx.RealTimeApp) -> None:
    app.add_system(default_values_get_used)
    app.update()


def default_values_get_used(query: xx.Query[ComponentWithDefaults]) -> None:
    component = query.result()
    assert np.all(component.a == 1.0)
    assert np.all(component.b == 2.0)
    assert np.all(component.c == 3)
    assert np.all(component.d == 4)
    assert np.all(component.e == True)  # noqa: E712
    assert np.all(component.f.a == 1.0)
    assert np.all(component.f.b == 2.0)
    assert np.all(component.f.c == 3)
    assert np.all(component.f.d == 4)
    assert np.all(component.f.e == True)  # noqa: E712
