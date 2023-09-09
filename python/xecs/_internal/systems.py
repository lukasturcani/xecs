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
    __slots__ = "function", "query_args", "other_args"

    def __init__(
        self,
        function: System,
        query_args: dict[str, Query[Any]],
        other_args: dict[str, NonQueryParameter],
    ) -> None:
        self.function = function
        self.query_args = query_args
        self.other_args = other_args


class FixedTimeStepSystemSpec:
    __slots__ = (
        "function",
        "query_args",
        "other_args",
        "time_step",
        "time_to_simulate",
    )

    def __init__(
        self,
        function: System,
        query_args: dict[str, Query[Any]],
        other_args: dict[str, NonQueryParameter],
        time_step: Duration,
    ) -> None:
        self.function = function
        self.query_args = query_args
        self.other_args = other_args
        self.time_step = time_step
        self.time_to_simulate = Duration.new(0, 0)


class PendingStartupSystems(Resource):
    systems: list[System]


class StartupSystems(Resource):
    systems: list[SystemSpec]


class PendingSystems(Resource):
    systems: list[tuple[System, Duration | None]]


class Systems(Resource):
    systems: list[SystemSpec]


class FixedTimeStepSystems(Resource):
    systems: list[FixedTimeStepSystemSpec]
