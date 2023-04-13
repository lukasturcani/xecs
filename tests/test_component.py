import ecstasy as ecs
import numpy as np
import numpy.typing as npt


def indices(xs: list[int]) -> npt.NDArray[np.uint32]:
    return np.array(xs, dtype=np.uint32)


class StructA(ecs.Struct):
    a: ecs.Float64


class StructB(ecs.Struct):
    b: ecs.Float64
    c: StructA


class MyComponent(ecs.Component):
    d: ecs.Float64
    e: StructA
    f: StructB


def test_spawning_entities_updates_views_of_children() -> None:
    pool = MyComponent.create_pool(100)
    pool.p_spawn(50)

    component = pool.p_component
    assert len(component) == 50
    assert len(component.d) == 50
    assert len(component.e) == 50
    assert len(component.f) == 50
    assert len(component.e.a) == 50
    assert len(component.f.b) == 50
    assert len(component.f.c) == 50
    assert len(component.f.c.a) == 50

    sub_view = component[indices([0, 10, 32])]
    assert len(sub_view) == 3
    assert len(sub_view.d) == 3
    assert len(sub_view.e) == 3
    assert len(sub_view.f) == 3
    assert len(sub_view.e.a) == 3
    assert len(sub_view.f.b) == 3
    assert len(sub_view.f.c) == 3
    assert len(sub_view.f.c.a) == 3

    pool.p_spawn(50)
    assert len(component) == 100
    assert len(component.d) == 100
    assert len(component.e) == 100
    assert len(component.f) == 100
    assert len(component.e.a) == 100
    assert len(component.f.b) == 100
    assert len(component.f.c) == 100
    assert len(component.f.c.a) == 100

    assert len(sub_view) == 3
    assert len(sub_view.d) == 3
    assert len(sub_view.e) == 3
    assert len(sub_view.f) == 3
    assert len(sub_view.e.a) == 3
    assert len(sub_view.f.b) == 3
    assert len(sub_view.f.c) == 3
    assert len(sub_view.f.c.a) == 3


def test_struct_getitem_creates_shared_view() -> None:
    assert False
