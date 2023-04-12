import inspect
import typing

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


class Component:
    component_ids: "typing.ClassVar[dict[type[Component], ComponentId]]" = {}
    _len: int

    @classmethod
    def create_pool(cls, size: int) -> ComponentPool[typing.Self]:
        pool = cls()
        pool._len = size
        for key, value in inspect.get_annotations(cls).items():
            setattr(pool, key, value.p_create_pool(size))
        return ComponentPool(pool, size)

    def p_spawn(self, num: int) -> None:
        for attr in inspect.get_annotations(self.__class__):
            getattr(self, attr).p_spawn(num)

    def __len__(self) -> int:
        return self._len

    def __init_subclass__(cls) -> None:
        super().__init_subclass__()
        cls.component_ids[cls] = len(cls.component_ids)
