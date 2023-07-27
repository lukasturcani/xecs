import typing

import numpy as np
import numpy.typing as npt
import pytest
import xecs as xx


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


@pytest.mark.benchmark(group="numpy-iadd-mask")
def benchmark_iadd_numpy(
    benchmark: typing.Any, size: int, key_size: float
) -> None:
    generator = np.random.default_rng(55)
    first = generator.random(size, dtype=np.float32)
    first_key = generator.random(len(first)) < key_size
    second = generator.random(size, dtype=np.float32)
    benchmark(iadd_numpy, first, first_key, second)


@pytest.mark.benchmark(group="xecs-iadd-mask")
def benchmark_iadd_xecs(
    benchmark: typing.Any,
    size: int,
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    first = xx.Float32.p_from_numpy(generator.random(size, dtype=np.float32))
    first_key = generator.random(len(first)) < key_size
    second = xx.Float32.p_from_numpy(generator.random(size, dtype=np.float32))
    benchmark(iadd_xecs, first, first_key, second[first_key])


def iadd_numpy(
    first: npt.NDArray[np.float32],
    first_key: npt.NDArray[np.bool_],
    second: npt.NDArray[np.float32],
) -> None:
    np.add(first, second, where=first_key, out=first)


def iadd_xecs(
    first: xx.Float32,
    first_key: npt.NDArray[np.bool_],
    second: xx.Float32,
) -> None:
    first_subview = first[first_key]
    first_subview += second


@pytest.fixture(
    params=(0.1, 0.5, 0.9),
)
def key_size(request: pytest.FixtureRequest) -> float:
    return request.param
