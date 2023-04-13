import typing

import numpy as np
import numpy.typing as npt

from ecstasy._internal.component_id import ComponentId
from ecstasy._internal.getitem_key import Key

QueryId: typing.TypeAlias = int

class MultipleArrayInidices:
    pass

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
    def run_query(self, query_id: QueryId) -> MultipleArrayInidices:
        pass

class ArrayViewIndices:
    @staticmethod
    def with_capacity(capacity: int) -> ArrayViewIndices: ...
    def spawn(self, num: int) -> None: ...
    def __len__(self) -> int: ...
    def __getitem__(self, key: Key) -> typing.Self: ...

class Float64:
    @staticmethod
    def from_numpy(array: npt.NDArray[np.float64]) -> "Float64": ...
    def numpy(self) -> npt.NDArray[np.float64]: ...
    def p_new_view_with_indices(
        self,
        indices: ArrayViewIndices,
    ) -> "Float64": ...
    @staticmethod
    def p_with_indices(
        indices: ArrayViewIndices,
    ) -> "Float64": ...
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
