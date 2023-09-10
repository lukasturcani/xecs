import typing
from collections.abc import Sequence
from typing import cast

from xecs._internal.component import Component
from xecs.xecs import product_2

if typing.TYPE_CHECKING:
    from xecs.xecs import QueryId

T = typing.TypeVar("T")


class Query(typing.Generic[T]):
    """
    A system parameter providing selective access to component data.
    """

    p_num_queries: typing.ClassVar[int] = 0
    p_query_id: "QueryId"
    p_result: T
    p_components: Sequence[type[Component]]
    p_tuple_query: bool

    @classmethod
    def p_new(
        cls,
        query_id: int,
        components: Sequence[type[Component]],
        tuple_query: bool,
    ) -> typing.Self:
        query = cls()
        query.p_query_id = query_id
        query.p_components = components
        query.p_tuple_query = tuple_query
        return query

    def result(self) -> T:
        """
        Get the component data matching the query.

        Returns:
            The component data.
        """
        return self.p_result

    def product_2(self) -> tuple[T, T]:
        """
        Get the cartesian product of component data matching the query.

        Returns:
            Every pair of entities.
        """
        if self.p_tuple_query:
            query_result = cast(Sequence[Component], self.p_result)
        else:
            query_result = cast(Sequence[Component], (self.p_result,))

        indices1, indices2 = product_2(
            [component.p_indices for component in query_result]
        )
        return cast(
            tuple[T, T],
            (
                tuple(
                    component.p_new_view_with_indices(indices)
                    for component, indices in zip(
                        query_result, indices1, strict=True
                    )
                ),
                tuple(
                    component.p_new_view_with_indices(indices)
                    for component, indices in zip(
                        query_result, indices2, strict=True
                    )
                ),
            ),
        )

    def __class_getitem__(cls, key: typing.Any) -> typing.Any:
        cls.p_num_queries += 1
        return super().__class_getitem__(key)  # type: ignore
