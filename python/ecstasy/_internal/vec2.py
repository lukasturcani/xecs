import typing

import numpy as np
import numpy.typing as npt

from ecstasy._internal.struct import Struct
from ecstasy.ecstasy import Float32


class Vec2(Struct):
    x: Float32
    y: Float32

    def __iadd__(
        self,
        other: typing.Self | Float32 | npt.NDArray[np.float32] | float,
    ) -> typing.Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x += other
            self.y += other
        else:
            self.x += other.x
            self.y += other.y
        return self

    def __isub__(
        self,
        other: typing.Self | Float32 | npt.NDArray[np.float32] | float,
    ) -> typing.Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x -= other
            self.y -= other
        else:
            self.x -= other.x
            self.y -= other.y
        return self

    def __imul__(
        self,
        other: typing.Self | Float32 | npt.NDArray[np.float32] | float,
    ) -> typing.Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x *= other
            self.y *= other
        else:
            self.x *= other.x
            self.y *= other.y
        return self

    def __itruediv__(
        self,
        other: typing.Self | Float32 | npt.NDArray[np.float32] | float,
    ) -> typing.Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x /= other
            self.y /= other
        else:
            self.x /= other.x
            self.y /= other.y
        return self

    def __ifloordiv__(
        self,
        other: typing.Self | Float32 | npt.NDArray[np.float32] | float,
    ) -> typing.Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x //= other
            self.y //= other
        else:
            self.x //= other.x
            self.y //= other.y
        return self

    def __imod__(
        self,
        other: typing.Self | Float32 | npt.NDArray[np.float32] | float,
    ) -> typing.Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x %= other
            self.y %= other
        else:
            self.x %= other.x
            self.y %= other.y
        return self

    def __ipow__(
        self,
        other: typing.Self | Float32 | npt.NDArray[np.float32] | float,
    ) -> typing.Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x **= other
            self.y **= other
        else:
            self.x **= other.x
            self.y **= other.y
        return self

    def numpy(self) -> npt.NDArray[np.float32]:
        return np.array([self.x.numpy(), self.y.numpy()], dtype=np.float32)
