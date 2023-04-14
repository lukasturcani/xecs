import typing

from ecstasy._internal.struct import Struct
from ecstasy.ecstasy import Float32


class Vec2(Struct):
    x: Float32
    y: Float32

    def __iadd__(self, other: typing.Self) -> None:
        self.x += other.x
        self.y += other.y
