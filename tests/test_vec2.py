import ecstasy as ecs
import numpy as np
import pytest


class VecContainer(ecs.Component):
    vec: ecs.Vec2


def test_numpy(vec1: ecs.Vec2) -> None:
    assert np.all(
        np.equal(
            vec1.numpy(),
            np.array([np.arange(10), np.arange(10)], dtype=np.float32),
        ),
    )


def test_iadd(vec1: ecs.Vec2, vec2: ecs.Vec2) -> None:
    vec1 += vec2
    assert np.all(np.equal(vec1.numpy(), vec2.numpy() * 2))


@pytest.fixture
def vec1() -> ecs.Vec2:
    pool = VecContainer.create_pool(10)
    pool.p_spawn(10)
    pool.p_component.vec.x[:] = np.arange(10, dtype=np.float32)
    pool.p_component.vec.y[:] = np.arange(10, dtype=np.float32)
    return pool.p_component.vec


@pytest.fixture
def vec2() -> ecs.Vec2:
    pool = VecContainer.create_pool(10)
    pool.p_spawn(10)
    pool.p_component.vec.x[:] = np.arange(10, dtype=np.float32)
    pool.p_component.vec.y[:] = np.arange(10, dtype=np.float32)
    return pool.p_component.vec
