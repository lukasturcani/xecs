import numpy as np
import xecs as xx


def test_repr() -> None:
    xs = xx.Float32.p_from_numpy(np.array([1, 2, 3], dtype=np.float32))
    assert (
        str(xs)
        == repr(xs)
        == "<xecs.Float32 [\n    1.0,\n    2.0,\n    3.0,\n]>"
    )


def test_to_str() -> None:
    xs = xx.Float32.p_from_numpy(np.array([1, 2, 3], dtype=np.float32))
    assert xs.to_str() == "<xecs.Float32 [1.0, 2.0, 3.0]>"
