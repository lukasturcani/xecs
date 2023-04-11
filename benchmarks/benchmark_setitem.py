import typing

import ecstacy as ecs
import numpy as np
import numpy.typing as npt
import pytest


@pytest.mark.benchmark(group="numpy-setitem-indices-one")
def benchmark_numpy_setitem_indices_one(
    benchmark: typing.Any,
    array: npt.NDArray[np.float64],
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = np.array(
        np.where(generator.random(len(array)) < key_size)[0],
        dtype=np.uint32,
    )
    benchmark(numpy_setitem, array, key, 123.0)


@pytest.mark.benchmark(group="ecstacy-setitem-indices-one")
def benchmark_ecstacy_setitem_indices_one(
    benchmark: typing.Any,
    view: ecs.ArrayViewF64,
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = np.array(
        np.where(generator.random(len(view)) < key_size)[0],
        dtype=np.uint32,
    )
    benchmark(ecstacy_setitem, view, key, 123.0)


@pytest.mark.benchmark(group="numpy-setitem-indices-many")
def benchmark_numpy_setitem_indices_many(
    benchmark: typing.Any,
    array: npt.NDArray[np.float64],
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = np.array(
        np.where(generator.random(len(array)) < key_size)[0],
        dtype=np.uint32,
    )
    value = generator.random(len(key), dtype=np.float64)
    benchmark(numpy_setitem, array, key, value)


@pytest.mark.benchmark(group="ecstacy-setitem-indices-many")
def benchmark_ecstacy_setitem_indices_many(
    benchmark: typing.Any,
    view: ecs.ArrayViewF64,
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = np.array(
        np.where(generator.random(len(view)) < key_size)[0],
        dtype=np.uint32,
    )
    value = generator.random(len(key), dtype=np.float64)
    benchmark(ecstacy_setitem, view, key, value)


def numpy_setitem(
    array: npt.NDArray[np.float64],
    key: npt.NDArray[np.uint32],
    value: float | npt.NDArray[np.float64],
) -> None:
    array[key] = value


@pytest.mark.benchmark(group="numpy-setitem-mask-many")
def benchmark_numpy_setitem_mask_many(
    benchmark: typing.Any,
    array: npt.NDArray[np.float64],
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = generator.random(len(array)) < key_size
    value = generator.random(np.count_nonzero(key), dtype=np.float64)
    benchmark(numpy_setitem, array, key, value)


@pytest.mark.benchmark(group="ecstacy-setitem-mask-many")
def benchmark_ecstacy_setitem_mask_many(
    benchmark: typing.Any,
    view: ecs.ArrayViewF64,
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = generator.random(len(view)) < key_size
    value = generator.random(np.count_nonzero(key), dtype=np.float64)
    benchmark(ecstacy_setitem, view, key, value)


def ecstacy_setitem(
    view: ecs.ArrayViewF64,
    key: npt.NDArray[np.uint32],
    value: float | npt.NDArray[np.float64],
) -> None:
    view[key] = value


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
def array(request: typing.Any) -> npt.NDArray[np.float64]:
    return request.param


@pytest.fixture
def view(array: npt.NDArray[np.float64]) -> ecs.ArrayViewF64:
    return ecs.ArrayF64.from_numpy(array).view()


@pytest.fixture(
    params=(0.1, 0.5, 0.9),
)
def key_size(request: typing.Any) -> float:
    return request.param
