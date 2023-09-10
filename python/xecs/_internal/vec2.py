import typing
from typing import Self, TypeAlias

import numpy as np
import numpy.typing as npt

from xecs._internal.struct import Struct
from xecs.xecs import ArrayViewIndices, Float32

if typing.TYPE_CHECKING:
    from xecs.xecs import Float32Rhs


class Vec2(Struct):
    """
    A set of 2D vector values.
    """

    x: Float32
    """The x values."""
    y: Float32
    """The y values."""

    @staticmethod
    def from_numpy(array: npt.NDArray[np.float32]) -> "Vec2":
        """
        Create the vectors from a NumPy array.

        Parameters:
            array: The NumPy array.
        Returns:
            The vectors.
        """
        obj = Vec2.__new__(Vec2)
        if isinstance(array, np.ndarray) and array.ndim == 2:
            indices = ArrayViewIndices.with_capacity(len(array[0]))
            indices.spawn(len(array[0]))
            obj.x = Float32.p_from_numpy(array[0]).p_new_view_with_indices(
                indices
            )
            obj.y = Float32.p_from_numpy(array[1]).p_new_view_with_indices(
                indices
            )
        else:
            indices = ArrayViewIndices.with_capacity(len(array))
            obj.x = Float32.p_from_numpy(array).p_new_view_with_indices(
                indices
            )
            obj.y = Float32.p_from_numpy(array).p_new_view_with_indices(
                indices
            )
        obj._indices = indices
        return obj

    @staticmethod
    def from_xy(x: float, y: float, num: int) -> "Vec2":
        """
        Create the vectors from x and y values.

        Parameters:
            x: The x value of the vectors.
            y: The y value of the vectors.
            num: The number of vectors to create.
        Returns:
            The vectors.
        """
        obj = Vec2.__new__(Vec2)
        obj._init(Float32.p_from_value(x, num), Float32.p_from_value(y, num))
        return obj

    def clamp_length(self, min: float, max: float) -> Self:
        """
        Ensure lengths are no less than `min` and no more than `max`.

        Parameters:
            min: The minimum length.
            max: The maximum length.
        Returns:
            Itself.
        """
        length_sq = self.length_squared()
        too_small = length_sq < (min * min)
        too_small_self = self[too_small]
        too_large = length_sq > (max * max)
        too_large_self = self[too_large]

        too_small_length = length_sq[too_small]
        np.sqrt(too_small_length, out=too_small_length)

        too_small_self.x /= too_small_length
        too_small_self.x *= min
        too_small_self.y /= too_small_length
        too_small_self.y *= min

        too_large_length = length_sq[too_large]
        np.sqrt(too_large_length, out=too_large_length)

        too_large_self.x /= too_large_length
        too_large_self.x *= max
        too_large_self.y /= too_large_length
        too_large_self.y *= max
        return self

    def dot_xy(self, x: float, y: float) -> npt.NDArray[np.float32]:
        """
        Get the dot products between the vectors and another vector.

        Parameters:
            x: The x value of the other vector.
            y: The y value of the other vector.
        Returns:
            The dot products.
        """
        tmp = self.numpy()
        np.multiply(tmp, [[x], [y]], out=tmp)
        return np.sum(tmp, axis=0, out=tmp[0])

    def dot_vec2(self, other: "Vec2") -> npt.NDArray[np.float32]:
        """
        Get the element wise dot products with some other vectors.

        Parameters:
            other: The other vectors.
        Returns:
            The dot products.
        """
        tmp = self.numpy()
        np.multiply(tmp, other.numpy(), out=tmp)
        return np.sum(tmp, axis=0, out=tmp[0])

    def perp_dot_xy(self, x: float, y: float) -> npt.NDArray[np.float32]:
        """
        Get the perpendicular dot products with the vectors and another.

        Also known as the 2D cross product.

        Parameters:
            x: The x value of the other vector.
            y: The y value of the other vector.
        Returns:
            The perpendicular dot products.
        """
        return np.cross(self.numpy(), [x, y], axisa=0, axisb=0)

    def perp_dot_vec2(self, other: "Vec2") -> npt.NDArray[np.float32]:
        """
        Get the perpendicular dot products with some other vectors.

        Also known as the 2D cross product.

        Parameters:
            other: The other vectors.
        Returns:
            The perpendicular dot products.
        """
        return np.cross(self.numpy(), other.numpy(), axisa=0, axisb=0)

    def angle_between_xy(self, x: float, y: float) -> npt.NDArray[np.float32]:
        """
        Return the angles between another vector.

        The returned angles are in radians and in the range ``[-pi, pi]``.

        Parameters:
            x: The x value of the other vector.
            y: The y value of the other vector.
        Returns:
            The angles.
        """
        tmp = self.length_squared()
        np.multiply(tmp, x * x + y * y, out=tmp)
        np.sqrt(tmp, out=tmp)
        tmp2 = self.dot_xy(x, y)
        tmp2 /= tmp
        np.arccos(tmp2, out=tmp2)
        perp_dot = self.perp_dot_xy(x, y)
        return np.multiply(tmp2, np.sign(perp_dot, out=perp_dot), out=tmp2)

    def angle_between_vec2(self, other: "Vec2") -> npt.NDArray[np.float32]:
        """
        Return the angles between some other vectors.

        The returned angles are in radians and in the range ``[-pi, pi]``.

        Parameters:
            other: The other vectors.
        Returns:
            The angles.
        """
        tmp = self.length_squared()
        np.multiply(tmp, other.length_squared(), out=tmp)
        np.sqrt(tmp, out=tmp)
        tmp2 = self.dot_vec2(other)
        tmp2 /= tmp
        np.arccos(tmp2, out=tmp2)
        perp_dot = self.perp_dot_vec2(other)
        return np.multiply(tmp2, np.sign(perp_dot, out=perp_dot), out=tmp2)

    def length_squared(self) -> npt.NDArray[np.float32]:
        """
        Get the squared lengths of the vectors.

        Returns:
            The squared lengths.
        """
        values = self.numpy()
        np.multiply(values, values, out=values)
        return np.sum(values, axis=0, out=values[0])

    def numpy(self) -> npt.NDArray[np.float32]:
        """
        Return a copy of the vectors as a NumPy array.

        Returns:
            The vectors.
        """
        return np.array([self.x.numpy(), self.y.numpy()], dtype=np.float32)

    def fill(self, value: "Float32Rhs") -> None:
        """
        Set the values of the vectors.

        Parameters:
            value (float | list[float]): The values to set the vectors to.
        """
        if isinstance(value, np.ndarray) and value.ndim == 2:
            self.x.fill(value[0])
            self.y.fill(value[1])
        else:
            self.x.fill(value)
            self.y.fill(value)

    def _init(self, x: Float32, y: Float32) -> None:
        self.x = x
        self.y = y

    def __add__(self, other: "Rhs") -> npt.NDArray[np.float32]:
        if isinstance(other, Vec2 | Float32):
            other = other.numpy()
        result = self.numpy()
        result += other
        return result

    def __iadd__(self, other: "Rhs") -> Self:
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

    def __sub__(self, other: "Rhs") -> npt.NDArray[np.float32]:
        if isinstance(other, Vec2 | Float32):
            other = other.numpy()
        result = self.numpy()
        result -= other
        return result

    def __isub__(self, other: "Rhs") -> Self:
        if isinstance(other, int | float | Float32 | list | tuple):
            self.x -= other
            self.y -= other
        elif isinstance(other, np.ndarray) and other.ndim == 2:
            self.x -= other[0]
            self.y -= other[1]
        elif isinstance(other, np.ndarray):
            self.x -= other
            self.y -= other
        else:
            self.x -= other.x
            self.y -= other.y
        return self

    def __mul__(self, other: "Rhs") -> npt.NDArray[np.float32]:
        if isinstance(other, Vec2 | Float32):
            other = other.numpy()
        result = self.numpy()
        result *= other
        return result

    def __imul__(self, other: "Rhs") -> Self:
        if isinstance(other, int | float | Float32 | list | tuple):
            self.x *= other
            self.y *= other
        elif isinstance(other, np.ndarray) and other.ndim == 2:
            self.x *= other[0]
            self.y *= other[1]
        elif isinstance(other, np.ndarray):
            self.x *= other
            self.y *= other
        else:
            self.x *= other.x
            self.y *= other.y
        return self

    def __truediv__(self, other: "Rhs") -> npt.NDArray[np.float32]:
        if isinstance(other, Vec2 | Float32):
            other = other.numpy()
        result = self.numpy()
        result /= other
        return result

    def __itruediv__(self, other: "Rhs") -> Self:
        if isinstance(other, int | float | Float32 | list | tuple):
            self.x /= other
            self.y /= other
        elif isinstance(other, np.ndarray) and other.ndim == 2:
            self.x /= other[0]
            self.y /= other[1]
        elif isinstance(other, np.ndarray):
            self.x /= other
            self.y /= other
        else:
            self.x /= other.x
            self.y /= other.y
        return self

    def __floordiv__(self, other: "Rhs") -> npt.NDArray[np.float32]:
        if isinstance(other, Vec2 | Float32):
            other = other.numpy()
        result = self.numpy()
        result //= other
        return result

    def __ifloordiv__(self, other: "Rhs") -> Self:
        if isinstance(other, int | float | Float32 | list | tuple):
            self.x //= other
            self.y //= other
        elif isinstance(other, np.ndarray) and other.ndim == 2:
            self.x //= other[0]
            self.y //= other[1]
        elif isinstance(other, np.ndarray):
            self.x //= other
            self.y //= other
        else:
            self.x //= other.x
            self.y //= other.y
        return self

    def __mod__(self, other: "Rhs") -> npt.NDArray[np.float32]:
        if isinstance(other, Vec2 | Float32):
            other = other.numpy()
        result = self.numpy()
        result %= other
        return result

    def __imod__(self, other: "Rhs") -> Self:
        if isinstance(other, int | float | Float32 | list | tuple):
            self.x %= other
            self.y %= other
        elif isinstance(other, np.ndarray) and other.ndim == 2:
            self.x %= other[0]
            self.y %= other[1]
        elif isinstance(other, np.ndarray):
            self.x %= other
            self.y %= other
        else:
            self.x %= other.x
            self.y %= other.y
        return self

    def __pow__(self, other: "Rhs") -> npt.NDArray[np.float32]:
        if isinstance(other, Vec2 | Float32):
            other = other.numpy()
        result = self.numpy()
        result **= other
        return result

    def __ipow__(self, other: "Rhs") -> Self:
        if isinstance(other, int | float | Float32 | list | tuple):
            self.x **= other
            self.y **= other
        elif isinstance(other, np.ndarray) and other.ndim == 2:
            self.x **= other[0]
            self.y **= other[1]
        elif isinstance(other, np.ndarray):
            self.x **= other
            self.y **= other
        else:
            self.x **= other.x
            self.y **= other.y
        return self


Rhs: TypeAlias = (
    Vec2
    | Float32
    | npt.NDArray[np.float32]
    | float
    | list[float]
    | tuple[float, ...]
)
