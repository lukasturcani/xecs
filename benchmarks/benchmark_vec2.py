import typing

import pytest
import xecs as xx


@pytest.mark.benchmark(group="Vec2.numpy")
def benchmark_numpy(benchmark: typing.Any) -> None:
    vec = xx.Vec2.from_xy(10, 20, 10_000)
    benchmark(vec.numpy)
