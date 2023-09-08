from typing import Generic, TypeVar

import numpy as np
import numpy.typing as npt

from xecs import xecs

T = TypeVar("T")


class PyField(Generic[T]):
    _inner: "xecs.PyField[T]"

    def __init__(self, inner: "xecs.PyField[T]") -> None:
        self._inner = inner

    @staticmethod
    def p_with_indices(indices: xecs.ArrayViewIndices) -> "PyField[T]":
        return PyField(xecs.PyField.p_with_indices(indices))

    def p_new_view_with_indices(
        self,
        indices: xecs.ArrayViewIndices,
    ) -> "PyField[T]":
        return PyField(self._inner.p_new_view_with_indices(indices))

    def fill(self, value: T) -> None:
        self._inner.fill(value)

    def get(self, index: int) -> T:
        return self._inner.get(index)

    def __getitem__(self, key: npt.NDArray[np.bool_]) -> "PyField[T]":
        return PyField(self._inner[key])
