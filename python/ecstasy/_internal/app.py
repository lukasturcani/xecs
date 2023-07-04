import inspect
import typing
from collections import abc

from ecstasy._internal.commands import Commands
from ecstasy._internal.component import (
    Component,
    ComponentPool,
    ComponentT,
)
from ecstasy._internal.query import Query
from ecstasy._internal.resource import Resource
from ecstasy._internal.time import Time
from ecstasy.ecstasy import RustApp
from ecstasy.ecstasy import Time as RustTime

if typing.TYPE_CHECKING:
    from ecstasy.ecstasy import ComponentId, Duration

P = typing.ParamSpec("P")
R = typing.TypeVar("R")
ResourceT = typing.TypeVar("ResourceT", bound=Resource)


class SystemSignatureError(Exception):
    pass


SystemParameter: typing.TypeAlias = Query | Commands | Resource
NonQueryParameter: typing.TypeAlias = Commands | Resource
System: typing.TypeAlias = abc.Callable


class SystemSpec:
    __slots__ = "function", "query_args", "other_args"

    def __init__(
        self,
        function: System,
        query_args: dict[str, Query],
        other_args: dict[str, NonQueryParameter],
    ) -> None:
        self.function = function
        self.query_args = query_args
        self.other_args = other_args


class App:
    _rust_app: RustApp
    _pools: "dict[ComponentId, ComponentPool[Component]]"
    _pending_startup_systems: list[System]
    _startup_systems: list[SystemSpec]
    _pending_systems: list[System]
    _systems: list[SystemSpec]
    _commands: Commands
    _resources: dict[type[Resource], Resource]

    @classmethod
    def new(cls) -> typing.Self:
        app = cls()
        app._rust_app = RustApp(
            num_pools=len(Component.component_ids),
            num_queries=Query.p_num_queries,
        )
        app._pools = {}
        app._pending_startup_systems = []
        app._startup_systems = []
        app._pending_systems = []
        app._systems = []
        app._commands = Commands()
        app._resources = {}
        return app

    def add_resource(self, resource: Resource) -> None:
        self._resources[type(resource)] = resource

    def add_startup_system(self, system: System) -> None:
        self._pending_startup_systems.append(system)

    def add_system(
        self,
        system: System,
        run_condition: "Duration | None" = None,
    ) -> None:
        self._pending_systems.append(system)

    def _get_system_args(
        self,
        system: abc.Callable[P, R],
    ) -> tuple[dict[str, Query], dict[str, NonQueryParameter]]:
        query_args: dict[str, Query] = {}
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

    def _run_query(self, query: Query) -> None:
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

    def _run_systems(self, systems: list[SystemSpec]) -> None:
        for system in systems:
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
        for system in self._pending_systems:
            query_args, other_args = self._get_system_args(system)
            self._systems.append(
                SystemSpec(
                    function=system,
                    query_args=query_args,
                    other_args=other_args,
                )
            )
        self._pending_systems = []

    def p_run_startup_systems(self) -> None:
        self._run_systems(self._startup_systems)

    def p_run_systems(self) -> None:
        self._run_systems(self._systems)

    def update(self) -> None:
        self.p_process_pending_systems()
        if Time not in self._resources:
            self._resources[Time] = Time(RustTime.default())
        self._update()

    def _update(self) -> None:
        self._get_resource(Time).update()
        self.p_run_startup_systems()
        self.p_run_systems()

    def run(self) -> None:
        self.p_process_pending_systems()
        if Time not in self._resources:
            self._resources[Time] = Time(RustTime.default())
        self._run()

    def _run(self) -> None:
        while True:
            self._update()

    def add_component_pool(self, pool: ComponentPool[ComponentT]) -> None:
        component_id = Component.component_ids[type(pool.p_component)]
        self._rust_app.add_component_pool(component_id, pool.p_capacity)
        self._pools[component_id] = pool  # type: ignore

    def _get_resource(self, resource: type[ResourceT]) -> ResourceT:
        return self._resources[resource]  # type: ignore
