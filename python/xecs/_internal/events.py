from dataclasses import dataclass, field
from typing import Any, Generic, TypeVar

from xecs._internal.resource import Resource

T = TypeVar("T")


@dataclass(frozen=True, slots=True)
class EventReader(Generic[T]):
    events: list[T] = field(default_factory=list)


class EventWriter(Generic[T]):
    p_readers: list[EventReader[T]]

    def __init__(self) -> None:
        self.p_readers = []

    def send(self, event: T) -> None:
        for reader in self.p_readers:
            reader.events.append(event)


class Events(Resource):
    writers: dict[type, EventWriter[Any]]
