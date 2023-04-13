import typing

import ecstasy as ecs
import numpy as np
import numpy.typing as npt
import pytest


@pytest.mark.benchmark(group="numpy-getitem-indices")
def benchmark_numpy_getitem_indices(
    benchmark: typing.Any,
    numpy_array: npt.NDArray[np.float64],
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = np.array(
        np.where(generator.random(len(numpy_array)) < key_size)[0],
        dtype=np.uint32,
    )
    benchmark(getitem, numpy_array, key)


@pytest.mark.benchmark(group="ecstasy-getitem-indices")
def benchmark_ecstasy_getitem_indices(
    benchmark: typing.Any,
    ecs_array: ecs.Float64,
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = np.array(
        np.where(generator.random(len(ecs_array)) < key_size)[0],
        dtype=np.uint32,
    )
    benchmark(getitem, ecs_array, key)


@pytest.mark.benchmark(group="numpy-getitem-mask")
def benchmark_numpy_getitem_mask(
    benchmark: typing.Any,
    numpy_array: npt.NDArray[np.float64],
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = generator.random(len(numpy_array)) < key_size
    benchmark(getitem, numpy_array, key)


@pytest.mark.benchmark(group="ecstasy-getitem-mask")
def benchmark_ecstasy_getitem_mask(
    benchmark: typing.Any,
    ecs_array: ecs.Float64,
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = generator.random(len(ecs_array)) < key_size
    benchmark(getitem, ecs_array, key)


def getitem(
    array: ecs.Float64 | npt.NDArray[np.float64],
    key: npt.NDArray[np.uint32 | np.bool_],
) -> None:
    array[key]


@pytest.fixture(
    params=(
        np.arange(10, dtype=np.float64),
        np.arange(100, dtype=np.float64),
        np.arange(1_000, dtype=np.float64),
        np.arange(10_000, dtype=np.float64),
        np.arange(100_000, dtype=np.float64),
        np.arange(1_000_000, dtype=np.float64),
    ),
)
def numpy_array(request: typing.Any) -> npt.NDArray[np.float64]:
    return request.param


@pytest.fixture
def ecs_array(numpy_array: npt.NDArray[np.float64]) -> ecs.Float64:
    return ecs.Float64.from_numpy(numpy_array)


@pytest.fixture(
    params=(0.1, 0.5, 0.9),
)
def key_size(request: typing.Any) -> float:
    return request.param
