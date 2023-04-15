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

class Float32:
    @staticmethod
    def from_numpy(array: npt.NDArray[np.float32]) -> Float32: ...
    def numpy(self) -> npt.NDArray[np.float32]: ...
    def p_new_view_with_indices(
        self,
        indices: ArrayViewIndices,
    ) -> Float32: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> Float32: ...
    def __getitem__(self, key: Key) -> Float32: ...
    def __setitem__(
        self,
        key: Key,
        value: float | Float32 | npt.NDArray[np.float32],
    ) -> None: ...
    def __len__(self) -> int: ...
    def __iadd__(
        self,
        other: Float32 | npt.NDArray[np.float32] | float,
    ) -> Float32: ...

class Float64:
    @staticmethod
    def from_numpy(array: npt.NDArray[np.float64]) -> Float64: ...
    def numpy(self) -> npt.NDArray[np.float64]: ...
    def p_new_view_with_indices(
        self,
        indices: ArrayViewIndices,
    ) -> Float64: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> Float64: ...
    def __getitem__(self, key: Key) -> Float64: ...
    def __setitem__(
        self,
        key: Key,
        value: float | Float64 | npt.NDArray[np.float64],
    ) -> None: ...
    def __len__(self) -> int: ...
    def __iadd__(
        self,
        other: Float64 | npt.NDArray[np.float64] | float,
    ) -> Float64: ...
