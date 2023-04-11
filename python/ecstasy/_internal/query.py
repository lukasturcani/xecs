import typing

T = typing.TypeVar("T")


class Query(typing.Generic[T]):
    def result(self) -> T:
        ...
