import typing

import numpy as np
import numpy.typing as npt
import pytest
import xecs as xx


def setitem(
    array: xx.Float32 | npt.NDArray[np.float32],
    key: npt.NDArray[np.bool_],
    value: npt.NDArray[np.float32],
) -> None:
    array[key] = value


@pytest.mark.benchmark(group="numpy-setitem-mask-many")
def benchmark_numpy_setitem_mask_many(
    benchmark: typing.Any,
    numpy_array: npt.NDArray[np.float32],
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = generator.random(len(numpy_array)) < key_size
    value = generator.random(np.count_nonzero(key), dtype=np.float32)
    benchmark(setitem, numpy_array, key, value)


@pytest.mark.benchmark(group="xecs-setitem-mask-many")
def benchmark_xecs_setitem_mask_many(
    benchmark: typing.Any,
    ecs_array: xx.Float32,
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = generator.random(len(ecs_array)) < key_size
    value = generator.random(np.count_nonzero(key), dtype=np.float32)
    benchmark(setitem, ecs_array, key, value)


@pytest.fixture(
    params=(
        np.arange(15, dtype=np.float32),
        np.arange(100, dtype=np.float32),
        np.arange(1_000, dtype=np.float32),
        np.arange(1_000_000, dtype=np.float32),
    ),
    ids=(
        "15",
        "100",
        "1_000",
        "1_000_000",
    ),
)
def numpy_array(request: pytest.FixtureRequest) -> npt.NDArray[np.float32]:
    return request.param


@pytest.fixture
def ecs_array(numpy_array: npt.NDArray[np.float32]) -> xx.Float32:
    return xx.Float32.p_from_numpy(numpy_array)


@pytest.fixture(
    params=(0.1, 0.5, 0.9),
)
def key_size(request: pytest.FixtureRequest) -> float:
    return request.param
