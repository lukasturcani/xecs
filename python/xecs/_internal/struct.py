import inspect
import typing

import numpy as np
import numpy.typing as npt

from xecs.xecs import ArrayViewIndices


class Struct:
    _indices: ArrayViewIndices

    @classmethod
    def p_with_indices(cls, indices: ArrayViewIndices) -> typing.Self:
        struct = cls()
        struct._indices = indices
        for key, value in inspect.get_annotations(cls).items():
            setattr(struct, key, value.p_with_indices(indices))
        return struct

    def __getitem__(self, key: npt.NDArray[np.bool_]) -> typing.Self:
        cls = self.__class__
        struct = cls()
        struct._indices = self._indices[key]
        for attr_name in inspect.get_annotations(cls):
            attr_value = getattr(self, attr_name)
            setattr(
                struct,
                attr_name,
                attr_value.p_new_view_with_indices(struct._indices),
            )
        return struct

    def p_new_view_with_indices(
        self,
        indices: ArrayViewIndices,
    ) -> typing.Self:
        cls = self.__class__
        struct = cls()
        struct._indices = indices
        for attr_name in inspect.get_annotations(cls):
            attr_value = getattr(self, attr_name)
            setattr(
                struct,
                attr_name,
                attr_value.p_new_view_with_indices(indices),
            )
        return struct

    def __len__(self) -> int:
        return len(self._indices)
