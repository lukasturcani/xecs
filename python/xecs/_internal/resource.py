import inspect
import typing


@typing.dataclass_transform()
class Resource:
    """
    A base class for resources.
    """

    def __init_subclass__(cls) -> None:
        setattr(cls, "__init__", cls._subclass_init)

    def _subclass_init(self, *args: typing.Any, **kwargs: typing.Any) -> None:
        for attr, value in zip(inspect.get_annotations(type(self)), args):
            setattr(self, attr, value)
        for attr, value in kwargs.items():
            setattr(self, attr, value)


ResourceT = typing.TypeVar("ResourceT", bound=Resource)
