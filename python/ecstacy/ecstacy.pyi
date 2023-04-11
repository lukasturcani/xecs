import typing

import numpy as np
import numpy.typing as npt

ElementType = typing.TypeVar(
    "ElementType",
    np.bool_,
    np.int8,
    np.int16,
    np.int32,
    np.int64,
    np.uint8,
    np.uint16,
    np.uint32,
    np.uint64,
    np.float32,
    np.float64,
)

ViewType = typing.TypeVar("ViewType")

class _Array(typing.Generic[ElementType, ViewType]):
    @classmethod
    def from_numpy(cls, array: npt.NDArray[ElementType]) -> typing.Self: ...
    def numpy(self) -> npt.NDArray[ElementType]: ...
    def view(self) -> ViewType: ...

Key: typing.TypeAlias = npt.NDArray[np.uint32 | np.bool_] | slice

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
    def __len__(self) -> int: ...

class ArrayF64(_Array[np.float64, ArrayViewF64]): ...
