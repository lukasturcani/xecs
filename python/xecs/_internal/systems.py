from collections import abc
from typing import Any, TypeAlias

from xecs._internal.commands import Commands
from xecs._internal.query import Query
from xecs._internal.resource import Resource
from xecs._internal.world import World
from xecs.xecs import Duration


class SystemSignatureError(Exception):
    pass


SystemParameter: TypeAlias = Query[Any] | Commands | Resource
NonQueryParameter: TypeAlias = Commands | Resource | World
System: TypeAlias = abc.Callable[..., Any]


class SystemSpec:
    """
    Specification for a system.
    """

    __slots__ = "function", "query_args", "other_args"

    function: System
    """The function which runs the system."""
    query_args: dict[str, Query[Any]]
    """The query arguments for the system."""
    other_args: dict[str, NonQueryParameter]
    """The non-query arguments for the system."""

    def __init__(
        self,
        function: System,
        query_args: dict[str, Query[Any]],
        other_args: dict[str, NonQueryParameter],
    ) -> None:
        """
        Parameters:
            function: The function which runs the system.
            query_args: The query arguments for the system.
            other_args: The non-query arguments for the system.
        """
        self.function = function
        self.query_args = query_args
        self.other_args = other_args


class FixedTimeStepSystemSpec:
    """
    Specification for a fixed time step system.
    """

    __slots__ = (
        "function",
        "query_args",
        "other_args",
        "time_step",
        "time_to_simulate",
    )

    function: System
    """The function which runs the system."""
    query_args: dict[str, Query[Any]]
    """The query arguments for the system."""
    other_args: dict[str, NonQueryParameter]
    """The non-query arguments for the system."""
    time_step: Duration
    """The time span between runs of the system."""
    time_to_simulate: Duration
    """The amount of time which has not been simulated yet by the system."""

    def __init__(
        self,
        function: System,
        query_args: dict[str, Query[Any]],
        other_args: dict[str, NonQueryParameter],
        time_step: Duration,
    ) -> None:
        """
        Parameters:
            function: The function which runs the system.
            query_args: The query arguments for the system.
            other_args: The non-query arguments for the system.
            time_step: The time span between runs of the system.
        """
        self.function = function
        self.query_args = query_args
        self.other_args = other_args
        self.time_step = time_step
        self.time_to_simulate = Duration.new(0, 0)


class PendingStartupSystems(Resource):
    """
    A resource holding startup systems to be added to the world.
    """

    systems: list[System]
    """Startup systems to be added to the world."""


class StartupSystems(Resource):
    """
    A resource holding startup systems to be run.
    """

    systems: list[SystemSpec]
    """Startup systems to be run."""


class PendingSystems(Resource):
    """
    A resource holding systems to be added to the world.
    """

    systems: list[tuple[System, Duration | None]]
    """
    Systems to be added to the world, together with
    their time step, if any.
    """


class Systems(Resource):
    """A resource hodling systems to be run."""

    systems: list[SystemSpec]
    """Systems to be run."""


class FixedTimeStepSystems(Resource):
    """
    A resource for holding fixed time step systems.
    """

    systems: list[FixedTimeStepSystemSpec]
    """The fixed time step systems to be run."""
