import inspect
import typing

from ecstasy.ecstasy import ArrayViewIndices


class Struct:
    _indices: ArrayViewIndices

    @classmethod
    def p_with_indices(cls, indices: ArrayViewIndices) -> typing.Self:
        struct = cls()
        struct._indices = indices
        for key, value in inspect.get_annotations(cls).items():
            setattr(struct, key, value.p_with_indices(indices))
        return struct

    def __len__(self) -> int:
        return len(self._indices)
