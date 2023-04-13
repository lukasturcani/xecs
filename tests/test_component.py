import ecstasy as ecs
import numpy as np
import numpy.typing as npt
import pytest


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


@pytest.fixture
def component() -> MyComponent:
    pool = MyComponent.create_pool(100)
    return pool.p_inner


def test_spawning_entities_updates_views_of_children(
    component: MyComponent,
) -> None:
    assert len(component) == 100
    sub_view = component[indices([0, 10, 32])]
    assert len(component) == 100
    assert len(sub_view) == 3
    assert len(sub_view.d) == 3
    assert len(sub_view.e) == 3
    assert len(sub_view.f) == 3
    assert len(sub_view.e.a) == 3
    assert len(sub_view.f.b) == 3
    assert len(sub_view.f.c) == 3
    assert len(sub_view.f.c.a) == 3
