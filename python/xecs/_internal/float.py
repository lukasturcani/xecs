import builtins
import typing

from xecs.xecs import Float32

Float: typing.TypeAlias = Float32


def float(*, default: builtins.float) -> Float:
    return typing.cast(Float, default)
