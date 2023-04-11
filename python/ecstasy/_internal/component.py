import inspect
import typing

ComponentT = typing.TypeVar("ComponentT", bound="Component")


class ComponentPool(typing.Generic[ComponentT]):
    __slots__ = ("inner",)

    inner: ComponentT

    def __init__(self, inner: ComponentT) -> None:
        self.inner = inner


class Component:
    _component_types: "typing.ClassVar[list[type[Component]]]" = []
    _len: int

    @classmethod
    def create_pool(cls, size: int) -> ComponentPool[typing.Self]:
        pool = cls()
        pool._len = size
        for key, value in inspect.get_annotations(cls).items():
            setattr(pool, key, value.create_pool(size))
        return ComponentPool(pool)

    def __len__(self) -> int:
        return self._len

    def __init_subclass__(cls) -> None:
        super().__init_subclass__()
        cls._component_types.append(cls)
