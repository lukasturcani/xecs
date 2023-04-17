import typing

import numpy as np
import numpy.typing as npt

from ecstasy._internal.rust_type_aliases import ComponentId, Key, QueryId

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
    def __getitem__(self, key: Key) -> ArrayViewIndices: ...

_NumpyT = typing.TypeVar("_NumpyT", np.float32, np.float64)

class _Array(typing.Generic[_NumpyT]):
    def numpy(self) -> npt.NDArray[_NumpyT]: ...
    def p_new_view_with_indices(
        self,
        indices: ArrayViewIndices,
    ) -> typing.Self: ...
    def __getitem__(self, key: Key) -> typing.Self: ...
    def __setitem__(
        self,
        key: Key,
        value: float | typing.Self | npt.NDArray[_NumpyT],
    ) -> None: ...
    def __len__(self) -> int: ...
    def __iadd__(
        self,
        other: typing.Self | npt.NDArray[_NumpyT] | float,
    ) -> typing.Self: ...
    def __isub__(
        self,
        other: typing.Self | npt.NDArray[_NumpyT] | float,
    ) -> typing.Self: ...
    def __imul__(
        self,
        other: typing.Self | npt.NDArray[_NumpyT] | float,
    ) -> typing.Self: ...
    def __itruediv__(
        self,
        other: typing.Self | npt.NDArray[_NumpyT] | float,
    ) -> typing.Self: ...
    def __ifloordiv__(
        self,
        other: typing.Self | npt.NDArray[_NumpyT] | float,
    ) -> typing.Self: ...
    def __imod__(
        self,
        other: typing.Self | npt.NDArray[_NumpyT] | float,
    ) -> typing.Self: ...
    def __ipow__(
        self,
        other: typing.Self | npt.NDArray[_NumpyT] | float,
    ) -> typing.Self: ...

class Float32(_Array[np.float32]):
    @staticmethod
    def from_numpy(array: npt.NDArray[np.float32]) -> Float32: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> Float32: ...

class Float64(_Array[np.float64]):
    @staticmethod
    def from_numpy(array: npt.NDArray[np.float64]) -> Float64: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> Float64: ...
