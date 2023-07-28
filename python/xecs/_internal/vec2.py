from typing import Self, TypeAlias

import numpy as np
import numpy.typing as npt

from xecs._internal.struct import Struct
from xecs.xecs import Float32

Rhs: TypeAlias = (
    Float32 | npt.NDArray[np.float32] | float | list[float] | tuple[float, ...]
)


class Vec2(Struct):
    x: Float32
    y: Float32

    @staticmethod
    def from_xy(x: float, y: float, num: int) -> "Vec2":
        obj = Vec2.__new__(Vec2)
        obj._init(Float32.p_from_value(x, num), Float32.p_from_value(y, num))
        return obj

    def angle_between(self, other: "Vec2", out: Float32) -> None:
        pass

    def numpy(self) -> npt.NDArray[np.float32]:
        return np.array([self.x.numpy(), self.y.numpy()], dtype=np.float32)

    def fill(self, value: Rhs) -> None:
        self.x.fill(value)
        self.y.fill(value)

    def _init(self, x: Float32, y: Float32) -> None:
        self.x = x
        self.y = y

    def __iadd__(self, other: Self | Rhs) -> Self:
        if isinstance(other, int | float | Float32 | list | tuple):
            self.x += other
            self.y += other
        elif isinstance(other, np.ndarray) and other.ndim == 2:
            self.x += other[0]
            self.y += other[1]
        elif isinstance(other, np.ndarray):
            self.x += other
            self.y += other
        else:
            self.x += other.x
            self.y += other.y
        return self

    def __isub__(self, other: Self | Rhs) -> Self:
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            self.x -= other
            self.y -= other
        else:
            self.x -= other.x
            self.y -= other.y
        return self

    def __imul__(self, other: Self | Rhs) -> Self:
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            self.x *= other
            self.y *= other
        else:
            self.x *= other.x
            self.y *= other.y
        return self

    def __itruediv__(self, other: Self | Rhs) -> Self:
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            self.x /= other
            self.y /= other
        else:
            self.x /= other.x
            self.y /= other.y
        return self

    def __ifloordiv__(self, other: Self | Rhs) -> Self:
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            self.x //= other
            self.y //= other
        else:
            self.x //= other.x
            self.y //= other.y
        return self

    def __imod__(self, other: Self | Rhs) -> Self:
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            self.x %= other
            self.y %= other
        else:
            self.x %= other.x
            self.y %= other.y
        return self

    def __ipow__(self, other: Self | Rhs) -> Self:
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            self.x **= other
            self.y **= other
        else:
            self.x **= other.x
            self.y **= other.y
        return self
