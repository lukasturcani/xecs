import numpy as np
import xecs as xx


def test_from_value() -> None:
    numbers = xx.Float32.p_from_value(10, 20)
    assert np.all(np.equal(numbers.numpy(), np.full(20, 10)))
