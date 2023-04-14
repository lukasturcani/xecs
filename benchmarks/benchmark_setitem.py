import typing

import ecstasy as ecs
import numpy as np
import numpy.typing as npt
import pytest


@pytest.mark.benchmark(group="numpy-setitem-indices-one")
def benchmark_numpy_setitem_indices_one(
    benchmark: typing.Any,
    numpy_array: npt.NDArray[np.float64],
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = np.array(
        np.where(generator.random(len(numpy_array)) < key_size)[0],
        dtype=np.uint32,
    )
    benchmark(setitem, numpy_array, key, 123.0)


@pytest.mark.benchmark(group="ecstasy-setitem-indices-one")
def benchmark_ecstasy_setitem_indices_one(
    benchmark: typing.Any,
    ecs_array: ecs.Float64,
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = np.array(
        np.where(generator.random(len(ecs_array)) < key_size)[0],
        dtype=np.uint32,
    )
    benchmark(setitem, ecs_array, key, 123.0)


@pytest.mark.benchmark(group="numpy-setitem-indices-many")
def benchmark_numpy_setitem_indices_many(
    benchmark: typing.Any,
    numpy_array: npt.NDArray[np.float64],
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = np.array(
        np.where(generator.random(len(numpy_array)) < key_size)[0],
        dtype=np.uint32,
    )
    value = generator.random(len(key), dtype=np.float64)
    benchmark(setitem, numpy_array, key, value)


@pytest.mark.benchmark(group="ecstasy-setitem-indices-many")
def benchmark_ecstasy_setitem_indices_many(
    benchmark: typing.Any,
    ecs_array: ecs.Float64,
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = np.array(
        np.where(generator.random(len(ecs_array)) < key_size)[0],
        dtype=np.uint32,
    )
    value = generator.random(len(key), dtype=np.float64)
    benchmark(setitem, ecs_array, key, value)


def setitem(
    array: ecs.Float64 | npt.NDArray[np.float64],
    key: npt.NDArray[np.uint32],
    value: float | npt.NDArray[np.float64],
) -> None:
    array[key] = value


@pytest.mark.benchmark(group="numpy-setitem-mask-many")
def benchmark_numpy_setitem_mask_many(
    benchmark: typing.Any,
    numpy_array: npt.NDArray[np.float64],
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = generator.random(len(numpy_array)) < key_size
    value = generator.random(np.count_nonzero(key), dtype=np.float64)
    benchmark(setitem, numpy_array, key, value)


@pytest.mark.benchmark(group="ecstasy-setitem-mask-many")
def benchmark_ecstasy_setitem_mask_many(
    benchmark: typing.Any,
    ecs_array: ecs.Float64,
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = generator.random(len(ecs_array)) < key_size
    value = generator.random(np.count_nonzero(key), dtype=np.float64)
    benchmark(setitem, ecs_array, key, value)


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
