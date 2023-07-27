import numpy as np
import numpy.typing as npt
import xecs as xx


def indices(length: int, indices: list[int]) -> npt.NDArray[np.bool_]:
    mask = np.zeros(length, dtype=np.bool_)
    mask[indices] = True
    return mask


class StructA(xx.Struct):
    a: xx.Float32


class StructB(xx.Struct):
    b: xx.Float32
    c: StructA


class MyComponent(xx.Component):
    d: xx.Float32
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

    sub_view = component[indices(50, [0, 10, 32])]
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
    pool = MyComponent.create_pool(10)
    pool.p_spawn(10)

    struct = pool.p_component.f
    assert len(struct) == 10
    assert len(struct.b) == 10
    assert len(struct.c) == 10
    assert len(struct.c.a) == 10
    sub_view = struct[indices(10, list(range(5)))]
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
