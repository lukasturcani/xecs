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

    def to_str(self, nesting: int) -> str:
        cls = type(self)
        fields = []
        indent = " " * 4 * nesting
        for attr_name in inspect.get_annotations(cls):
            attr_value = getattr(self, attr_name)
            if isinstance(attr_value, Struct):
                attr_str = attr_value.to_str(nesting + 1)
            else:
                attr_str = attr_value.to_str()
            fields.append(f"{indent}{attr_name}={attr_str},")
            joined = "\n    ".join(fields)
        return f"<xecs.{type(self).__name__}(\n    {joined}\n{indent})>"

    def __len__(self) -> int:
        return len(self._indices)
