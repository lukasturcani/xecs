import builtins
import typing

from xecs.xecs import Float32

Float: typing.TypeAlias = Float32


def float(*, default: builtins.float) -> Float:
    """
    Provide additional data about a component field.

    Parameters:
        default: The default value for the field.
    """
    return typing.cast(Float, default)
