import operator
import typing

import numpy as np
import pytest
import xecs as xx


@pytest.mark.benchmark(group="Vec2.__mul__")
def benchmark_mul(benchmark: typing.Any) -> None:
    generator = np.random.default_rng(55)
    first = xx.Vec2.from_numpy(
        generator.random((2, 1_000_000), dtype=np.float32)
    )
    second = xx.Vec2.from_numpy(
        generator.random((2, 1_000_000), dtype=np.float32)
    )
    benchmark(operator.mul, first, second)
