import typing

import numpy as np
import numpy.typing as npt

from ecstasy._internal.rust_type_aliases import ComponentId, QueryId

class MultipleArrayInidices:
    def next(self) -> ArrayViewIndices | None: ...

class RustApp:
    def __init__(self, num_pools: int, num_queries: int) -> None: ...
    def spawn(self, components: list[ComponentId], num: int) -> None: ...
    def add_component_pool(
        self,
        component_id: ComponentId,
        capacity: int,
    ) -> None: ...
    def add_query(
        self,
        first_component: ComponentId,
        other_components: list[ComponentId],
    ) -> QueryId: ...
    def run_query(self, query_id: QueryId) -> MultipleArrayInidices: ...

class ArrayViewIndices:
    @staticmethod
    def with_capacity(capacity: int) -> ArrayViewIndices: ...
    def spawn(self, num: int) -> None: ...
    def __len__(self) -> int: ...

_NumpyFloatT = typing.TypeVar("_NumpyFloatT", np.float32, np.float64)
_NumpyIntT = typing.TypeVar(
    "_NumpyIntT",
    np.int8,
    np.int16,
    np.int32,
    np.int64,
    np.uint8,
    np.uint16,
    np.uint32,
    np.uint64,
)

_NumpyFloat: typing.TypeAlias = np.float32 | np.float64
_NumpyInt: typing.TypeAlias = (
    np.int8
    | np.int16
    | np.int32
    | np.int64
    | np.uint8
    | np.uint16
    | np.uint32
    | np.uint64
)

_Array: typing.TypeAlias = _FloatArray | _IntArray

class _FloatArray(typing.Generic[_NumpyFloatT]):
    def numpy(self) -> npt.NDArray[_NumpyFloatT]: ...
    def __getitem__(self, key: ArrayViewIndices) -> typing.Self: ...
    def __setitem__(
        self,
        key: ArrayViewIndices,
        value: float | _Array | npt.NDArray[_NumpyFloat | _NumpyInt],
    ) -> None: ...
    def __len__(self) -> int: ...
    def __iadd__(
        self,
        other: float | _Array | npt.NDArray[_NumpyFloat | _NumpyInt],
    ) -> typing.Self: ...
    def __isub__(
        self,
        other: float | _Array | npt.NDArray[_NumpyFloat | _NumpyInt],
    ) -> typing.Self: ...
    def __imul__(
        self,
        other: float | _Array | npt.NDArray[_NumpyFloat | _NumpyInt],
    ) -> typing.Self: ...
    def __itruediv__(
        self,
        other: float | _Array | npt.NDArray[_NumpyFloat | _NumpyInt],
    ) -> typing.Self: ...
    def __ifloordiv__(
        self,
        other: float | _Array | npt.NDArray[_NumpyFloat | _NumpyInt],
    ) -> typing.Self: ...
    def __imod__(
        self,
        other: float | _Array | npt.NDArray[_NumpyFloat | _NumpyInt],
    ) -> typing.Self: ...
    def __ipow__(
        self,
        other: float | _Array | npt.NDArray[_NumpyFloat | _NumpyInt],
    ) -> typing.Self: ...

class _IntArray(typing.Generic[_NumpyIntT]):
    def numpy(self) -> npt.NDArray[_NumpyFloatT]: ...
    def __getitem__(self, key: ArrayViewIndices) -> typing.Self: ...
    def __setitem__(
        self,
        key: ArrayViewIndices,
        value: float | _Array | npt.NDArray[_NumpyFloat | _NumpyInt],
    ) -> None: ...
    def __len__(self) -> int: ...
    def __iadd__(
        self,
        other: float | _IntArray | npt.NDArray[_NumpyInt],
    ) -> typing.Self: ...
    def __isub__(
        self,
        other: float | _IntArray | npt.NDArray[_NumpyInt],
    ) -> typing.Self: ...
    def __imul__(
        self,
        other: float | _IntArray | npt.NDArray[_NumpyInt],
    ) -> typing.Self: ...
    def __itruediv__(
        self,
        other: float | _IntArray | npt.NDArray[_NumpyInt],
    ) -> typing.Self: ...
    def __ifloordiv__(
        self,
        other: float | _IntArray | npt.NDArray[_NumpyInt],
    ) -> typing.Self: ...
    def __imod__(
        self,
        other: float | _IntArray | npt.NDArray[_NumpyInt],
    ) -> typing.Self: ...
    def __ipow__(
        self,
        other: float | _IntArray | npt.NDArray[_NumpyInt],
    ) -> typing.Self: ...

class Float32(_FloatArray[np.float32]):
    @staticmethod
    def p_from_numpy(array: npt.NDArray[np.float32]) -> Float32: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> Float32: ...

class Float64(_FloatArray[np.float64]):
    @staticmethod
    def p_from_numpy(array: npt.NDArray[np.float64]) -> Float64: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> Float64: ...

class Int8(_IntArray[np.int8]):
    @staticmethod
    def p_from_numpy(array: npt.NDArray[np.int8]) -> Int8: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> Int8: ...

class Int16(_IntArray[np.int16]):
    @staticmethod
    def p_from_numpy(array: npt.NDArray[np.int16]) -> Int16: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> Int16: ...

class Int32(_IntArray[np.int32]):
    @staticmethod
    def p_from_numpy(array: npt.NDArray[np.int32]) -> Int32: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> Int32: ...

class Int64(_IntArray[np.int64]):
    @staticmethod
    def p_from_numpy(array: npt.NDArray[np.int64]) -> Int64: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> Int64: ...

class UInt8(_IntArray[np.uint8]):
    @staticmethod
    def p_from_numpy(array: npt.NDArray[np.uint8]) -> UInt8: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> UInt8: ...

class UInt16(_IntArray[np.uint16]):
    @staticmethod
    def p_from_numpy(array: npt.NDArray[np.uint16]) -> UInt16: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> UInt16: ...

class UInt32(_IntArray[np.uint32]):
    @staticmethod
    def p_from_numpy(array: npt.NDArray[np.uint32]) -> UInt32: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> UInt32: ...

class UInt64(_IntArray[np.uint64]):
    @staticmethod
    def p_from_numpy(array: npt.NDArray[np.uint64]) -> UInt64: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> UInt64: ...
