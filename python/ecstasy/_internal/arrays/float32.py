import typing

import numpy as np
import numpy.typing as npt

from ecstasy.ecstasy import ArrayViewIndices, iadd_float32


class Float32:
    _array: npt.NDArray[np.float32]
    _indices: ArrayViewIndices

    @staticmethod
    def p_from_numpy(array: npt.NDArray[np.float32]) -> "Float32":
        float32 = Float32()
        float32._array = array
        float32._indices = ArrayViewIndices.with_capacity(len(array))
        float32._indices.spawn(len(array))
        return float32

    def __iadd__(self, other: "Float32") -> typing.Self:
        iadd_float32(self._array, self._indices, other._array, other._indices)
        return self

    def __len__(self) -> int:
        return len(self._array)

    def __getitem__(
        self,
        key: npt.NDArray[np.bool_],
    ) -> typing.Self:
        float32 = self.__class__()
        float32._array = self._array
        float32._indices = self._indices[key]
        return float32
