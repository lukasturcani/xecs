import typing

import ecstasy as ecs
import numpy as np
import numpy.typing as npt
import pytest


@pytest.fixture(
    params=(
        10,
        100,
        1_000,
        1_000_000,
    ),
    ids=(
        "10",
        "100",
        "1_000",
        "1_000_000",
    ),
)
def size(request: pytest.FixtureRequest) -> int:
    return request.param


@pytest.mark.benchmark(group="numpy-iadd")
def benchmark_iadd_numpy(benchmark: typing.Any, size: int) -> None:
    generator = np.random.default_rng(55)
    first = generator.random(size, dtype=np.float32)
    second = generator.random(size, dtype=np.float32)
    benchmark(iadd, first, second)


@pytest.mark.benchmark(group="ecstasy-iadd")
def benchmark_iadd_ecstasy(benchmark: typing.Any, size: int) -> None:
    generator = np.random.default_rng(55)
    first = ecs.Float32.from_numpy(generator.random(size, dtype=np.float32))
    second = ecs.Float32.from_numpy(generator.random(size, dtype=np.float32))
    benchmark(iadd, first, second)


T = typing.TypeVar("T", ecs.Float32, npt.NDArray[np.float32])


def iadd(first: T, second: T) -> None:
    first += second
