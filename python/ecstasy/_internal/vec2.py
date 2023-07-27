import typing
from typing import Self, TypeAlias

import numpy as np
import numpy.typing as npt

from ecstasy._internal.struct import Struct
from ecstasy.ecstasy import Float32

Rhs: TypeAlias = Float32 | npt.NDArray[np.float32] | float


class Vec2(Struct):
    x: Float32
    y: Float32

    def __iadd__(self, other: Self | Rhs) -> Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x += other
            self.y += other
        else:
            self.x += other.x
            self.y += other.y
        return self

    def __isub__(self, other: Self | Rhs) -> Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x -= other
            self.y -= other
        else:
            self.x -= other.x
            self.y -= other.y
        return self

    def __imul__(self, other: Self | Rhs) -> Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x *= other
            self.y *= other
        else:
            self.x *= other.x
            self.y *= other.y
        return self

    def __itruediv__(self, other: Self | Rhs) -> Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x /= other
            self.y /= other
        else:
            self.x /= other.x
            self.y /= other.y
        return self

    def __ifloordiv__(self, other: Self | Rhs) -> Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x //= other
            self.y //= other
        else:
            self.x //= other.x
            self.y //= other.y
        return self

    def __imod__(self, other: Self | Rhs) -> typing.Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x %= other
            self.y %= other
        else:
            self.x %= other.x
            self.y %= other.y
        return self

    def __ipow__(self, other: Self | Rhs) -> Self:
        if isinstance(other, int | float | np.ndarray | Float32):
            self.x **= other
            self.y **= other
        else:
            self.x **= other.x
            self.y **= other.y
        return self

    def numpy(self) -> npt.NDArray[np.float32]:
        return np.array([self.x.numpy(), self.y.numpy()], dtype=np.float32)
