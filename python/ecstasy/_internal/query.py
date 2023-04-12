import typing

T = typing.TypeVar("T")


class Query(typing.Generic[T]):
    p_num_queries: typing.ClassVar[int] = 0
    p_query_id: int
    p_result: T

    @classmethod
    def p_new(cls, query_id: int) -> typing.Self:
        query = cls()
        query.p_query_id = query_id
        return query

    def result(self) -> T:
        return self.p_result

    def __class_getitem__(cls, key: typing.Any) -> typing.Any:
        cls.p_num_queries += 1
        return super().__class_getitem__(key)
