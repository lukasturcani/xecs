import typing

Components = typing.TypeVarTuple("Components")


class Query(typing.Generic[*Components]):
    def result(self) -> tuple[*Components]:
        ...
