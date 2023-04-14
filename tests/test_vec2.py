import ecstasy as ecs
import numpy as np


def test_iadd() -> None:
    first = ecs.Float32.from_numpy(np.arange(10, dtype=np.float32))
    second = ecs.Float32.from_numpy(np.arange(10, dtype=np.float32))
    first += second
    assert np.all(np.equal(first.numpy(), second.numpy() * 2))
