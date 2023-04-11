import typing

import ecstacy as ecs
import numpy as np
import numpy.typing as npt
import pytest


@pytest.mark.benchmark(group="numpy-getitem-indices")
def benchmark_numpy_getitem_indices(
    benchmark: typing.Any,
    array: npt.NDArray[np.float64],
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = np.array(
        np.where(generator.random(len(array)) < key_size)[0],
        dtype=np.uint32,
    )
    benchmark(numpy_getitem, array, key)


@pytest.mark.benchmark(group="ecstacy-getitem-indices")
def benchmark_ecstacy_getitem_indices(
    benchmark: typing.Any,
    view: ecs.ArrayViewF64,
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = np.array(
        np.where(generator.random(len(view)) < key_size)[0],
        dtype=np.uint32,
    )
    benchmark(ecstacy_getitem, view, key)


@pytest.mark.benchmark(group="numpy-getitem-mask")
def benchmark_numpy_getitem_mask(
    benchmark: typing.Any,
    array: npt.NDArray[np.float64],
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = generator.random(len(array)) < key_size
    benchmark(numpy_getitem, array, key)


@pytest.mark.benchmark(group="ecstacy-getitem-mask")
def benchmark_ecstacy_getitem_mask(
    benchmark: typing.Any,
    view: ecs.ArrayViewF64,
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = generator.random(len(view)) < key_size
    benchmark(ecstacy_getitem, view, key)


def numpy_getitem(
    array: npt.NDArray[np.float64],
    key: npt.NDArray[np.uint32 | np.bool_],
) -> None:
    array[key]


def ecstacy_getitem(
    view: ecs.ArrayViewF64,
    key: npt.NDArray[np.uint32 | np.bool_],
) -> None:
    view[key]


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
