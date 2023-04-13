import typing

import numpy as np
import numpy.typing as npt

Key: typing.TypeAlias = npt.NDArray[np.uint32 | np.bool_] | slice

class ArrayViewIndices:
    pass

class Float64:
    @staticmethod
    def from_numpy(array: npt.NDArray[np.float64]) -> "Float64": ...
    def numpy(self) -> npt.NDArray[np.float64]: ...
    def p_spawn(self, num: int) -> None: ...
    def p_new_view_with_indices(
        self,
        indices: ArrayViewIndices,
    ) -> "Float64": ...
    @staticmethod
    def p_create_pool(size: int, indices: ArrayViewIndices) -> "Float64": ...
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
