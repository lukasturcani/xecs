import typing

import numpy as np
import numpy.typing as npt

QueryId: typing.TypeAlias = int
ComponentId: typing.TypeAlias = int

class MultipleArrayInidices:
    def next(self) -> ArrayViewIndices | None: ...

class RustApp:
    def __init__(self, num_pools: int, num_queries: int) -> None: ...
    def spawn(self, components: list[ComponentId], num: int) -> None: ...
    def add_pool(
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
    def spawn(self, num: int) -> ArrayViewIndices: ...
    def __len__(self) -> int: ...
    def __getitem__(self, key: npt.NDArray[np.bool_]) -> ArrayViewIndices: ...

Float32Rhs: typing.TypeAlias = (
    float | Float32 | npt.NDArray[np.float32] | list[float] | tuple[float, ...]
)

class Float32:
    @staticmethod
    def p_from_numpy(array: npt.NDArray[np.float32]) -> Float32: ...
    @staticmethod
    def p_with_indices(indices: ArrayViewIndices) -> Float32: ...
    def numpy(self) -> npt.NDArray[np.float32]: ...
    def fill(self, values: Float32Rhs) -> None: ...
    def p_new_view_with_indices(
        self,
        indices: ArrayViewIndices,
    ) -> typing.Self: ...
    def __getitem__(self, key: npt.NDArray[np.bool_]) -> Float32: ...
    def __setitem__(
        self,
        key: npt.NDArray[np.bool_],
        value: Float32Rhs,
    ) -> None: ...
    def __len__(self) -> int: ...
    def __add__(self, other: Float32Rhs) -> Float32: ...
    def __iadd__(self, other: Float32Rhs) -> Float32: ...
    def __sub__(self, other: Float32Rhs) -> Float32: ...
    def __isub__(self, other: Float32Rhs) -> Float32: ...
    def __mul__(self, other: Float32Rhs) -> Float32: ...
    def __imul__(self, other: Float32Rhs) -> Float32: ...
    def __itruediv__(self, other: Float32Rhs) -> Float32: ...
    def __ifloordiv__(self, other: Float32Rhs) -> Float32: ...
    def __imod__(self, other: Float32Rhs) -> Float32: ...
    def __ipow__(self, other: Float32Rhs) -> Float32: ...
    def __lt__(self, other: Float32Rhs) -> npt.NDArray[np.bool_]: ...
    def __le__(self, other: Float32Rhs) -> npt.NDArray[np.bool_]: ...
    def __gt__(self, other: Float32Rhs) -> npt.NDArray[np.bool_]: ...
    def __ge__(self, other: Float32Rhs) -> npt.NDArray[np.bool_]: ...
    def __eq__(self, other: Float32Rhs) -> npt.NDArray[np.bool_]: ...  # type: ignore
    def __ne__(self, other: Float32Rhs) -> npt.NDArray[np.bool_]: ...  # type: ignore

class Duration:
    @staticmethod
    def new(secs: int, nanos: int) -> Duration: ...
    @staticmethod
    def from_millis(millis: int) -> Duration: ...
    @staticmethod
    def from_micros(micros: int) -> Duration: ...
    @staticmethod
    def from_nanos(nanos: int) -> Duration: ...
    def is_zero(self) -> bool: ...
    def as_secs(self) -> int: ...
    def subsec_micros(self) -> int: ...
    def subsec_nanos(self) -> int: ...
    def as_millis(self) -> int: ...
    def as_micros(self) -> int: ...
    def as_nanos(self) -> int: ...
    def checked_add(self, rhs: Duration) -> None: ...
    def __add__(self, rhs: Duration) -> Duration: ...
    def __iadd__(self, rhs: Duration) -> Duration: ...
    def checked_sub(self, rhs: Duration) -> None: ...
    def __sub__(self, rhs: Duration) -> Duration: ...
    def __isub__(self, rhs: Duration) -> Duration: ...
    def checked_mul(self, rhs: int) -> None: ...
    def checked_div(self, rhs: int) -> None: ...
    def __lt__(self, other: Duration) -> bool: ...
    def __le__(self, other: Duration) -> bool: ...
    def __gt__(self, other: Duration) -> bool: ...
    def __ge__(self, other: Duration) -> bool: ...
    def __eq__(self, other: Duration) -> bool: ...  # type: ignore
    def __ne__(self, other: Duration) -> bool: ...  # type: ignore

class Instant:
    @staticmethod
    def now() -> Instant: ...
    def checked_duration_since(self, earlier: Instant) -> Duration: ...
    def elapsed(self) -> Duration: ...
    def checked_add(self, duration: Duration) -> Instant: ...
    def checked_sub(self, duration: Duration) -> Instant: ...

class Time:
    @staticmethod
    def default() -> Time: ...
    def delta(self) -> Duration: ...
    def elapsed(self) -> Duration: ...
    def update(self) -> None: ...
