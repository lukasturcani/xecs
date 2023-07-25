import inspect
import typing
from collections import abc
from time import sleep
from typing import Any, TypeAlias, cast

from ecstasy._internal.commands import Commands
from ecstasy._internal.component import (
    Component,
    ComponentPool,
    ComponentT,
)
from ecstasy._internal.query import Query
from ecstasy._internal.resource import Resource, ResourceT
from ecstasy._internal.time import Time
from ecstasy.ecstasy import Duration, Instant, RustApp
from ecstasy.ecstasy import Time as RustTime

if typing.TYPE_CHECKING:
    from ecstasy.ecstasy import ComponentId

P = typing.ParamSpec("P")
R = typing.TypeVar("R")


class SystemSignatureError(Exception):
    pass


SystemParameter: TypeAlias = Query[Any] | Commands | Resource
NonQueryParameter: TypeAlias = Commands | Resource
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


class StartupSystems(Resource):
    systems: list[SystemSpec]


class Systems(Resource):
    systems: list[SystemSpec]


class FixedTimeStepSystems(Resource):
    systems: list[FixedTimeStepSystemSpec]


class App:
    world: World
    _rust_app: RustApp
    _pending_startup_systems: list[System]
    _pending_systems: list[tuple[System, Duration | None]]
    _commands: Commands

    def __init__(self) -> None:
        self._rust_app = RustApp(
            num_pools=len(Component.component_ids),
            num_queries=Query.p_num_queries,
        )
        self._pending_startup_systems = []
        self._pending_systems = []
        self._commands = Commands()

    def add_resource(self, resource: Resource) -> None:
        self.world.resources[type(resource)] = resource

    def add_startup_system(self, system: System) -> None:
        self._pending_startup_systems.append(system)

    def add_system(
        self,
        system: System,
        run_condition: Duration | None = None,
    ) -> None:
        self._pending_systems.append((system, run_condition))

    def _get_system_args(
        self,
        system: abc.Callable[P, R],
    ) -> tuple[dict[str, Query[Any]], dict[str, NonQueryParameter]]:
        query_args: dict[str, Query[Any]] = {}
        other_args: dict[str, NonQueryParameter] = {}
        for name, parameter in inspect.signature(system).parameters.items():
            if typing.get_origin(parameter.annotation) is Query:
                (components,) = typing.get_args(parameter.annotation)
                first_component, *other_components = typing.get_args(
                    components
                )
                component_ids = [Component.component_ids[first_component]]
                component_ids.extend(
                    Component.component_ids[component]
                    for component in other_components
                )

                query_id = self._rust_app.add_query(
                    first_component=component_ids[0],
                    other_components=component_ids[1:],
                )
                query_args[name] = Query.p_new(query_id, component_ids)

            elif parameter.annotation is Commands:
                other_args[name] = self._commands
            elif issubclass(parameter.annotation, Resource):
                other_args[name] = self._resources[parameter.annotation]
            else:
                expected_type = " | ".join(
                    arg.__name__ for arg in typing.get_args(SystemParameter)
                )
                raise SystemSignatureError(
                    f'annotation of parameter "{name}" in '
                    f'"{system.__name__}" is "{parameter.annotation}" '
                    "but needs to be "
                    f"{expected_type}"
                )
        return query_args, other_args

    def _run_query(self, query: Query[Any]) -> None:
        component_indices = self._rust_app.run_query(query.p_query_id)
        query.p_result = tuple(
            pool.p_component.p_new_view_with_indices(indices)
            for pool, indices in zip(
                (
                    self._pools[component_id]
                    for component_id in query.p_component_ids
                ),
                iter(lambda: component_indices.next(), None),
                strict=True,
            )
        )

    def p_process_pending_systems(self) -> None:
        for system in self._pending_startup_systems:
            query_args, other_args = self._get_system_args(system)
            self._startup_systems.append(
                SystemSpec(
                    function=system,
                    query_args=query_args,
                    other_args=other_args,
                )
            )
        self._pending_startup_systems = []
        for system, run_condition in self._pending_systems:
            query_args, other_args = self._get_system_args(system)

            match run_condition:
                case Duration():
                    self._fixed_time_step_systems.append(
                        FixedTimeStepSystemSpec(
                            system, query_args, other_args, run_condition
                        )
                    )
                case None:
                    self._systems.append(
                        SystemSpec(system, query_args, other_args)
                    )
        self._pending_systems = []

    def p_run_startup_systems(self) -> None:
        for system in self._startup_systems:
            for query in system.query_args.values():
                self._run_query(query)

            system.function(
                **system.query_args,
                **system.other_args,
            )
            self._commands.p_apply(
                app=self._rust_app,
                pools=self._pools,
            )

    def p_run_systems(self) -> None:
        for system in self._systems:
            for query in system.query_args.values():
                self._run_query(query)

            system.function(
                **system.query_args,
                **system.other_args,
            )
            self._commands.p_apply(
                app=self._rust_app,
                pools=self._pools,
            )

    def p_run_fixed_time_step_systems(
        self,
        time_since_last_update: Duration,
    ) -> None:
        for system in self._fixed_time_step_systems:
            system.time_to_simulate += time_since_last_update
            while system.time_to_simulate >= system.time_step:
                for query in system.query_args.values():
                    self._run_query(query)

                system.function(
                    **system.query_args,
                    **system.other_args,
                )
                self._commands.p_apply(
                    app=self._rust_app,
                    pools=self._pools,
                )
                system.time_to_simulate -= system.time_step

    def update(self) -> None:
        self.p_process_pending_systems()
        if Time not in self._resources:
            self._resources[Time] = Time(RustTime.default())
        self._update()

    def _update(self) -> None:
        time = self._get_resource(Time)
        time.update()
        self.p_run_startup_systems()
        self.p_run_systems()
        self.p_run_fixed_time_step_systems(time.delta())

    def run(
        self,
        frame_time: Duration = Duration.from_nanos(int(1e9 / 60)),
        max_run_time: Duration | None = None,
    ) -> None:
        self.p_process_pending_systems()
        if Time not in self._resources:
            self._resources[Time] = Time(RustTime.default())
        self._run(frame_time, max_run_time)

    def _run(
        self,
        frame_time: Duration,
        max_run_time: Duration | None,
    ) -> None:
        time = self._get_resource(Time)
        while True:
            start = Instant.now()
            self._update()
            if max_run_time is not None and time.elapsed() >= max_run_time:
                break
            sleep_time = frame_time - start.elapsed()
            sleep(sleep_time.as_nanos() / 1e9)

    def add_component_pool(self, pool: ComponentPool[ComponentT]) -> None:
        component_id = Component.component_ids[type(pool.p_component)]
        self._rust_app.add_component_pool(component_id, pool.p_capacity)
        self._pools[component_id] = cast(ComponentPool[Component], pool)

    def _get_resource(self, resource: type[ResourceT]) -> ResourceT:
        return cast(ResourceT, self._resources[resource])
