from typing import Any, Generic, TypeVar

from xecs._internal.resource import Resource

T = TypeVar("T")


class EventReader(Generic[T]):
    """
    Gives access to events of type ``T``.
    """

    __slots__ = ("events",)

    events: list[T]
    """The events sent since the last time the system was called."""

    def __init__(self) -> None:
        self.events = []


class EventWriter(Generic[T]):
    """
    Sends events of type ``T``.
    """

    p_readers: list[EventReader[T]]

    def __init__(self) -> None:
        self.p_readers = []

    def send(self, event: T) -> None:
        """
        Send an event which can be read by :class:`.EventReader`.

        Parameters:
            event: The event to send.
        """
        for reader in self.p_readers:
            reader.events.append(event)


class Events(Resource):
    writers: dict[type, EventWriter[Any]]
