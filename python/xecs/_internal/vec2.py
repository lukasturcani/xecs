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
        Vec2._init(Float32.p_from_value(x, num), Float32.p_from_value(y, num))
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

    def __add__(self, other: Self | Rhs) -> Self:
        obj = type(self).__new__(type(self))
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            obj._init(self.x + other, self.y + other)
            return obj
        obj._init(self.x + other.x, self.y + other.y)
        return obj

    def __iadd__(self, other: Self | Rhs) -> Self:
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            self.x += other
            self.y += other
        else:
            self.x += other.x
            self.y += other.y
        return self

    def __sub__(self, other: Self | Rhs) -> Self:
        obj = type(self).__new__(type(self))
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            obj._init(self.x - other, self.y - other)
            return obj
        obj._init(self.x - other.x, self.y - other.y)
        return obj

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

    def __mul__(self, other: Self | Rhs) -> Self:
        obj = type(self).__new__(type(self))
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            obj._init(self.x * other, self.y * other)
            return obj
        obj._init(self.x * other.x, self.y * other.y)
        return obj

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

    def __truediv__(self, other: Self | Rhs) -> Self:
        obj = type(self).__new__(type(self))
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            obj._init(self.x / other, self.y / other)
            return obj
        obj._init(self.x / other.x, self.y / other.y)
        return obj

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

    def __floordiv__(self, other: Self | Rhs) -> Self:
        obj = type(self).__new__(type(self))
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            obj._init(self.x // other, self.y // other)
            return obj
        obj._init(self.x // other.x, self.y // other.y)
        return obj

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

    def __mod__(self, other: Self | Rhs) -> Self:
        obj = type(self).__new__(type(self))
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            obj._init(self.x % other, self.y % other)
            return obj
        obj._init(self.x % other.x, self.y % other.y)
        return obj

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

    def __pow__(self, other: Self | Rhs) -> Self:
        obj = type(self).__new__(type(self))
        if isinstance(
            other, int | float | np.ndarray | Float32 | list | tuple
        ):
            obj._init(self.x**other, self.y**other)
            return obj
        obj._init(self.x**other.x, self.y**other.y)
        return obj

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
