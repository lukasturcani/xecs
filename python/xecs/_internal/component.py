import inspect
import typing

import numpy as np
import numpy.typing as npt

from xecs._internal.py_field import PyField, PyFieldError
from xecs._internal.struct import Struct
from xecs.xecs import ArrayViewIndices

if typing.TYPE_CHECKING:
    from xecs.xecs import ComponentId

ComponentT = typing.TypeVar("ComponentT", bound="Component")


class ComponentPool(typing.Generic[ComponentT]):
    """
    A preallocated pool of components.
    """

    __slots__ = "p_component", "p_capacity"

    p_component: ComponentT
    p_capacity: int

    @staticmethod
    def p_new(
        component: ComponentT,
        capacity: int,
    ) -> "ComponentPool[ComponentT]":
        component_pool: ComponentPool[ComponentT] = ComponentPool()
        component_pool.p_component = component
        component_pool.p_capacity = capacity
        return component_pool

    def p_spawn(self, num: int) -> ArrayViewIndices:
        return self.p_component.p_indices.spawn(num)


class Component:
    """
    A base class for components.
    """

    component_ids: "typing.ClassVar[dict[type[Component], ComponentId]]" = {}
    """
    Maps each component type to a unique ID. This is automatically
    populated by subclasses.
    """
    p_indices: ArrayViewIndices

    @classmethod
    def create_pool(cls, capacity: int) -> ComponentPool[typing.Self]:
        """
        Create a preallocated pool of components.

        Parameters:
            capacity: The maximum number of components the pool can hold.
        Returns:
            The component pool.
        """
        component = cls()
        component.p_indices = ArrayViewIndices.with_capacity(capacity)
        for key, value in inspect.get_annotations(cls).items():
            if typing.get_origin(value) is PyField:
                if not hasattr(cls, key):
                    error = PyFieldError("no default value")
                    error.add_note(
                        "To use PyField, you must provide a "
                        "default value with py_field(default=...)."
                    )
                    raise error
                setattr(
                    component,
                    key,
                    value.p_with_indices(
                        component.p_indices, getattr(cls, key)
                    ),
                )
            else:
                setattr(
                    component,
                    key,
                    value.p_with_indices(component.p_indices),
                )
        return ComponentPool.p_new(component, capacity)

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

    def __repr__(self) -> str:
        return str(self)

    def __str__(self) -> str:
        cls = type(self)
        fields = []
        joined = None
        for attr_name in inspect.get_annotations(cls):
            attr_value = getattr(self, attr_name)
            if isinstance(attr_value, Struct):
                attr_str = attr_value.to_str(1)
            else:
                attr_str = attr_value.to_str()
            fields.append(f"{attr_name}={attr_str},")
            joined = "\n    ".join(fields)
        if joined is not None:
            return f"<{type(self).__name__}(\n    {joined}\n)>"
        else:
            return f"<{type(self).__name__}()>"
