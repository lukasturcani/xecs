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


def test_iadd_vec2(vec1: ecs.Vec2, vec2: ecs.Vec2) -> None:
    vec1 += vec2
    assert np.all(np.equal(vec1.numpy(), vec2.numpy() * 2))


def test_iadd_float(vec1: ecs.Vec2) -> None:
    vec1 += 1.0
    expected = np.array([np.arange(10), np.arange(10)], dtype=np.float32) + 1
    assert np.all(np.equal(vec1.numpy(), expected))


def test_iadd_numpy(vec1: ecs.Vec2) -> None:
    vec1 += np.arange(10, dtype=np.float32)
    expected = np.array([np.arange(10), np.arange(10)], dtype=np.float32) * 2
    assert np.all(np.equal(vec1.numpy(), expected))


def test_isub_vec2(vec1: ecs.Vec2, vec2: ecs.Vec2) -> None:
    vec1 -= vec2
    assert np.all(np.equal(vec1.numpy(), np.zeros((2, 10), dtype=np.float32)))


def test_isub_float(vec1: ecs.Vec2) -> None:
    vec1 -= 1.0
    expected = np.array([np.arange(10), np.arange(10)], dtype=np.float32) - 1
    assert np.all(np.equal(vec1.numpy(), expected))


def test_isub_numpy(vec1: ecs.Vec2) -> None:
    vec1 -= np.arange(10, dtype=np.float32)
    assert np.all(np.equal(vec1.numpy(), np.zeros((2, 10), dtype=np.float32)))


def test_imul_vec2(vec1: ecs.Vec2, vec2: ecs.Vec2) -> None:
    vec1 *= vec2
    assert np.all(np.equal(vec1.numpy(), vec2.numpy() ** 2))


def test_imul_float(vec1: ecs.Vec2) -> None:
    vec1 *= 2.0
    expected = np.array([np.arange(10), np.arange(10)], dtype=np.float32) * 2
    assert np.all(np.equal(vec1.numpy(), expected))


def test_imul_numpy(vec1: ecs.Vec2) -> None:
    vec1 *= np.arange(10, dtype=np.float32)
    expected = np.array([np.arange(10), np.arange(10)], dtype=np.float32) ** 2
    assert np.all(np.equal(vec1.numpy(), expected))


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
