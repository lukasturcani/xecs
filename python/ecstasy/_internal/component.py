import inspect
import typing

import numpy as np
import numpy.typing as npt

from ecstasy.ecstasy import ArrayViewIndices

ComponentId: typing.TypeAlias = int

ComponentT = typing.TypeVar("ComponentT", bound="Component")


class ComponentPool(typing.Generic[ComponentT]):
    __slots__ = "p_inner", "p_capacity"

    p_inner: ComponentT
    p_capacity: int

    def __init__(self, inner: ComponentT, capacity: int) -> None:
        self.p_inner = inner
        self.p_capacity = capacity

    def p_spawn(self, num: int) -> None:
        self.p_inner.p_spawn(num)


Key: typing.TypeAlias = npt.NDArray[np.uint32 | np.bool_] | slice


class Component:
    component_ids: "typing.ClassVar[dict[type[Component], ComponentId]]" = {}
    _len: int
    _indices: ArrayViewIndices

    @classmethod
    def create_pool(cls, size: int) -> ComponentPool[typing.Self]:
        pool = cls()
        pool._len = size
        for key, value in inspect.get_annotations(cls).items():
            setattr(pool, key, value.p_create_pool(size))
        return ComponentPool(pool, size)

    def p_spawn(self, num: int) -> None:
        self._indices.spawn(num)

    def __getitem__(self, key: Key) -> typing.Self:
        pass

    def __len__(self) -> int:
        return self._len

    def __init_subclass__(cls) -> None:
        super().__init_subclass__()
        cls.component_ids[cls] = len(cls.component_ids)
