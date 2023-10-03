from typing import cast

from xecs.xecs import Int32


def int32(*, default: int) -> Int32:
    return cast(Int32, default)
