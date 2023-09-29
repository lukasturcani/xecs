from typing import Generic, TypeVar, cast

import numpy as np
import numpy.typing as npt

from xecs import xecs

T = TypeVar("T")


class PyFieldError(Exception):
    pass


class PyField(Generic[T]):
    """
    An array of Python component values.
    """

    _inner: "xecs.PyField[T]"

    @staticmethod
    def p_new(inner: "xecs.PyField[T]") -> "PyField[T]":
        py_field: PyField[T] = PyField()
        py_field._inner = inner
        return py_field

    @staticmethod
    def p_with_indices(
        indices: xecs.ArrayViewIndices, default: T
    ) -> "PyField[T]":
        return PyField.p_new(xecs.PyField.p_with_indices(indices, default))

    def p_new_view_with_indices(
        self,
        indices: xecs.ArrayViewIndices,
    ) -> "PyField[T]":
        return PyField.p_new(self._inner.p_new_view_with_indices(indices))

    def fill(self, value: T) -> None:
        """
        Set the values of the array.

        Parameters:
            value: The value to use.
        """
        self._inner.fill(value)

    def get(self, index: int) -> T:
        """
        Get the value at a specific index.

        Parameters:
            index: The index where the value is located.
        Returns:
            The value at `index`.
        """
        return self._inner.get(index)

    def __getitem__(self, key: npt.NDArray[np.bool_]) -> "PyField[T]":
        return PyField.p_new(self._inner[key])

    def __len__(self) -> int:
        return len(self._inner)


def py_field(*, default: T) -> PyField[T]:
    return cast(PyField[T], default)
