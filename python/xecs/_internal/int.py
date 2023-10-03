import builtins
import typing

from xecs.xecs import Int32

Int: typing.TypeAlias = Int32


def int(*, default: builtins.int) -> Int:
    return typing.cast(Int, default)
