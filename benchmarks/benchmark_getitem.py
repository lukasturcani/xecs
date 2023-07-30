import operator
import typing

import numpy as np
import numpy.typing as npt
import pytest
import xecs as xx


@pytest.mark.benchmark(group="numpy-getitem-mask")
def benchmark_numpy_getitem_mask(
    benchmark: typing.Any,
    numpy_array: npt.NDArray[np.float32],
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = generator.random(len(numpy_array)) < key_size
    benchmark(operator.getitem, numpy_array, key)


@pytest.mark.benchmark(group="xecs-getitem-mask")
def benchmark_xecs_getitem_mask(
    benchmark: typing.Any,
    ecs_array: xx.Float32,
    key_size: float,
) -> None:
    generator = np.random.default_rng(55)
    key = generator.random(len(ecs_array)) < key_size
    benchmark(operator.getitem, ecs_array, key)


@pytest.fixture(
    params=(
        np.arange(10, dtype=np.float32),
        np.arange(100, dtype=np.float32),
        np.arange(1_000, dtype=np.float32),
        np.arange(1_000_000, dtype=np.float32),
    ),
    ids=(
        "10",
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
