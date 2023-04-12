import typing

T = typing.TypeVar("T")


class Query(typing.Generic[T]):
    p_query_id: int

    @classmethod
    def p_new(cls, query_id: int) -> typing.Self:
        query = cls()
        query.p_query_id = query_id
        return query

    def result(self) -> T:
        self._app.run_query(self.p_query_id)
