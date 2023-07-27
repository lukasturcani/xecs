import inspect
import typing

import numpy as np
import numpy.typing as npt

from xecs.xecs import ArrayViewIndices

if typing.TYPE_CHECKING:
    from xecs.xecs import ComponentId

ComponentT = typing.TypeVar("ComponentT", bound="Component")


class ComponentPool(typing.Generic[ComponentT]):
    __slots__ = "p_component", "p_capacity"

    p_component: ComponentT
    p_capacity: int

    def __init__(self, component: ComponentT, capacity: int) -> None:
        self.p_component = component
        self.p_capacity = capacity

    def p_spawn(self, num: int) -> ArrayViewIndices:
        return self.p_component.p_indices.spawn(num)


class Component:
    component_ids: "typing.ClassVar[dict[type[Component], ComponentId]]" = {}
    p_indices: ArrayViewIndices

    @classmethod
    def create_pool(cls, capacity: int) -> ComponentPool[typing.Self]:
        component = cls()
        component.p_indices = ArrayViewIndices.with_capacity(capacity)
        for key, value in inspect.get_annotations(cls).items():
            setattr(
                component,
                key,
                value.p_with_indices(component.p_indices),
            )
        return ComponentPool(component, capacity)

    def __getitem__(self, key: npt.NDArray[np.bool_]) -> typing.Self:
        cls = self.__class__
        component = cls()
        component.p_indices = self.p_indices[key]
        for attr_name in inspect.get_annotations(cls):
            attr_value = getattr(self, attr_name)
            setattr(
                component,
                attr_name,
                attr_value.p_new_view_with_indices(component.p_indices),
            )
        return component

    def p_new_view_with_indices(
        self,
        indices: ArrayViewIndices,
    ) -> typing.Self:
        cls = self.__class__
        component = cls()
        component.p_indices = indices
        for attr_name in inspect.get_annotations(cls):
            attr_value = getattr(self, attr_name)
            setattr(
                component,
                attr_name,
                attr_value.p_new_view_with_indices(indices),
            )
        return component

    def __len__(self) -> int:
        return len(self.p_indices)

    def __init_subclass__(cls) -> None:
        super().__init_subclass__()
        cls.component_ids[cls] = len(cls.component_ids)
