import inspect
import typing

import numpy as np
import numpy.typing as npt

from xecs._internal.py_field import PyField, PyFieldError
from xecs._internal.resource import Resource
from xecs._internal.struct import Struct
from xecs.xecs import ArrayViewIndices

if typing.TYPE_CHECKING:
    from xecs.xecs import ComponentId

ComponentT = typing.TypeVar("ComponentT", bound="Component")


class MissingPoolError(Exception):
    pass


class Component:
    """
    A base class for components.
    """

    p_component_ids: "typing.ClassVar[dict[type[Component], ComponentId]]" = {}
    """
    Maps each component type to a unique ID. This is automatically
    populated by subclasses.
    """
    p_indices: ArrayViewIndices

    @classmethod
    def p_from_indices(cls, indices: ArrayViewIndices) -> typing.Self:
        component = cls()
        component.p_indices = indices
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
                    value.p_from_indices(indices, getattr(cls, key)),
                )
            elif issubclass(value, Struct):
                setattr(
                    component,
                    key,
                    value.p_from_indices(indices),
                )
            else:
                setattr(
                    component,
                    key,
                    value.p_from_indices(
                        indices,
                        getattr(cls, key, value.p_default_value()),
                    ),
                )
        return component

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
        cls.p_component_ids[cls] = len(cls.p_component_ids)

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


class Components(Resource):
    _components: dict[type[Component], Component]

    def add_component(self, component: Component) -> None:
        self._components[type(component)] = component

    def get_component(self, component: type[ComponentT]) -> ComponentT:
        return typing.cast(ComponentT, self._components[component])

    def has_component(self, component: type[ComponentT]) -> bool:
        return component in self._components
