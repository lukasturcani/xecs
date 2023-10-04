import pytest
import xecs as xx


class First(xx.Component):
    pass


class Second(xx.Component):
    pass


def test_entity_id() -> None:
    pass


def system1(query: xx.Query[tuple[xx.EntityId, First]]) -> None:
    entity_id, first = query.result()


def system2(query: xx.Query[tuple[xx.EntityId, Second]]) -> None:
    entity_id, second = query.result()


def system3(query: xx.Query[xx.EntityId]) -> None:
    entity_id = query.result()
