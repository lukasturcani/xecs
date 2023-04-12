import typing

T = typing.TypeVar("T")


class Commands:
    def spawn(self, components: T, num: int) -> T:
        pass
