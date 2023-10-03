from typing import cast

from xecs.xecs import Int32


def int32(*, default: int) -> Int32:
    """
    Provide additional data about a component field.

    Parameters:
        default: The default value for the field.
    """
    return cast(Int32, default)
