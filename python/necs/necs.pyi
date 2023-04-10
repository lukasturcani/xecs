import typing

import numpy as np
import numpy.typing as npt

from necs._internal.element_type import ElementType

numpy_ints: typing.TypeAlias = (
    np.int8
    | np.int16
    | np.int32
    | np.int64
    | np.uint8
    | np.uint16
    | np.uint32
    | np.uint64
)

ViewType = typing.TypeVar("ViewType")

class _Array(typing.Generic[ElementType, ViewType]):
    @classmethod
    def from_numpy(cls, array: npt.NDArray[ElementType]) -> typing.Self: ...
    def numpy(self) -> npt.NDArray[ElementType]: ...
    def view(self) -> ViewType: ...

Key: typing.TypeAlias = (
    list[int] | tuple[int, ...] | npt.NDArray[numpy_ints | np.bool_] | slice
)

class ArrayViewF64:
    def __getitem__(self, key: Key) -> typing.Self:
        pass
    def __setitem__(
        self,
        key: Key,
        value: float
        | tuple[float, ...]
        | list[float]
        | npt.NDArray[np.float64],
    ) -> None:
        pass

class ArrayF64(_Array[np.float64, ArrayViewF64]): ...
