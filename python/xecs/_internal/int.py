import typing

from xecs.xecs import Int32

Int: typing.TypeAlias = Int32


def int_(*, default: int) -> Int:
    """
    Provide additional data about a component field.

    Parameters:
        default: The default value for the field.
    """
    return typing.cast(Int, default)
