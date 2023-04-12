import typing

from ecstasy._internal.component import Component
from ecstasy.ecstasy import RustQuery

T = typing.TypeVar("T")


class Query(typing.Generic[T]):
    _query: RustQuery

    @classmethod
    def p_new(cls, components: tuple[type[Component], ...]) -> typing.Self:
        query = cls()
        first, *rest = components
        query._query = RustQuery(
            first_component=Component.component_ids[first],
            other_components=tuple(
                Component.component_ids[component] for component in rest
            ),
        )
        return query

    def result(self) -> T:
        ...
