import typing

import numpy as np
import numpy.typing as npt

from necs._internal.element_type import ElementType

_ints: typing.TypeAlias = (
    np.int8
    | np.int16
    | np.int32
    | np.int64
    | np.uint8
    | np.uint16
    | np.uint32
    | np.uint64
)


class Array(typing.Generic[ElementType]):
    __slots__ = "_array", "_indices", "_assign_value_at_indices"

    _array: npt.NDArray[ElementType]
    _indices: npt.NDArray[np.uint64] | None
    _assign_value_at_indices: typing.Any

    def __init__(
        self,
        array: npt.NDArray[ElementType],
        indices: npt.NDArray[np.uint64] | None = None,
    ) -> None:
        self._array = array
        self._indices = indices

    @typing.overload
    def __getitem__(
        self,
        key: list[int] | tuple[int, ...] | npt.NDArray[_ints],
    ) -> "Array[ElementType]":
        ...

    @typing.overload
    def __getitem__(self, key: slice) -> "Array[ElementType]":
        ...

    @typing.overload
    def __getitem__(self, key: int | _ints) -> ElementType:
        ...

    def __getitem__(self, key: typing.Any) -> typing.Any:
        if isinstance(key, list | tuple | np.ndarray):
            if self._indices is None:
                return Array(
                    self._array,
                    np.arange(len(self._array), dtype=np.uint64)[key],
                )
            else:
                return Array(self._array, self._indices[key])
        elif isinstance(key, slice):
            if self._indices is None:
                return Array(self._array[key])
            else:
                return Array(self._array, self._indices[key])
        else:
            if self._indices is None:
                return self._array[key]
            else:
                return self._array[self._indices[key]]

    @typing.overload
    def __setitem__(
        self,
        key: list[int | bool]
        | tuple[int | bool, ...]
        | npt.NDArray[_ints | np.bool_]
        | slice,
        value: npt.NDArray[ElementType]
        | list[ElementType | float | int | bool]
        | tuple[ElementType | float | int | bool, ...]
        | ElementType
        | float
        | int
        | bool,
    ) -> None:
        ...

    @typing.overload
    def __setitem__(
        self,
        key: int | _ints,
        value: ElementType | float | int | bool,
    ) -> None:
        ...

    def __setitem__(self, key: typing.Any, value: typing.Any) -> None:
        if isinstance(key, list | tuple | np.ndarray):
            if self._indices is None:
                self._assign_value_at_indices(
                    self._array,
                    np.arange(len(self._array), dtype=np.uint64)[key],
                    value,
                )
            else:
                self._assign_value_at_indices(
                    self._array, self._indices[key], value
                )
        elif isinstance(key, slice):
            if self._indices is None:
                self._array[key] = value
            else:
                self._assign_value_at_indices(
                    self._array, self._indices[key], value
                )
        else:
            if self._indices is None:
                self._array[key] = value
            else:
                self._array[self._indices[key]] = value
