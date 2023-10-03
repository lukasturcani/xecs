from typing import cast

from xecs.xecs import Float32


def float32(*, default: float) -> Float32:
    """
    Provide additional data about a component field.

    Parameters:
        default: The default value for the field.
    """
    return cast(Float32, default)
