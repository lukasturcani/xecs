import typing
from collections.abc import Sequence

from xecs._internal.component import Component

if typing.TYPE_CHECKING:
    from xecs.xecs import QueryId

T = typing.TypeVar("T")


class Query(typing.Generic[T]):
    p_num_queries: typing.ClassVar[int] = 0
    p_query_id: "QueryId"
    p_result: T
    p_components: Sequence[type[Component]]

    @classmethod
    def p_new(
        cls,
        query_id: int,
        components: Sequence[type[Component]],
    ) -> typing.Self:
        query = cls()
        query.p_query_id = query_id
        query.p_components = components
        return query

    def result(self) -> T:
        return self.p_result

    def __class_getitem__(cls, key: typing.Any) -> typing.Any:
        cls.p_num_queries += 1
        return super().__class_getitem__(key)  # type: ignore
