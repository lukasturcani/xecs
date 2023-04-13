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
    _indices: ArrayViewIndices

    @classmethod
    def create_pool(cls, capacity: int) -> ComponentPool[typing.Self]:
        component = cls()
        component._indices = ArrayViewIndices.with_capacity(capacity)
        for key, value in inspect.get_annotations(cls).items():
            setattr(
                component,
                key,
                value.p_with_indices(component._indices),
            )
        return ComponentPool(component, capacity)

    def p_spawn(self, num: int) -> None:
        self._indices.spawn(num)

    def __getitem__(self, key: Key) -> typing.Self:
        pass

    def __len__(self) -> int:
        return len(self._indices)

    def __init_subclass__(cls) -> None:
        super().__init_subclass__()
        cls.component_ids[cls] = len(cls.component_ids)
