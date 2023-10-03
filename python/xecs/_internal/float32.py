from typing import cast

from xecs.xecs import Float32


def float32(*, default: float) -> Float32:
    return cast(Float32, default)
