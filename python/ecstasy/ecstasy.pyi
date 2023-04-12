import typing

import numpy as np
import numpy.typing as npt

Key: typing.TypeAlias = npt.NDArray[np.uint32 | np.bool_] | slice

class Float64:
    @classmethod
    def from_numpy(cls, array: npt.NDArray[np.float64]) -> typing.Self: ...
    def numpy(self) -> npt.NDArray[np.float64]: ...
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
