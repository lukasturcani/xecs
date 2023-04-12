import inspect
import typing

from ecstasy._internal.commands import Commands
from ecstasy._internal.component import (
    Component,
    ComponentId,
    ComponentPool,
    ComponentT,
)
from ecstasy._internal.query import Query
from ecstasy.ecstasy import RustApp

P = typing.ParamSpec("P")
R = typing.TypeVar("R")


class SystemSignatureError(Exception):
    pass


SystemParameter: typing.TypeAlias = Query | Commands
NonQueryParameter: typing.TypeAlias = Commands


class System(typing.Protocol):
    def __call__(self, *args: SystemParameter) -> None:
        pass


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
    _pools: dict[ComponentId, ComponentPool[typing.Any]]
    _startup_systems: list[SystemSpec]
    _systems: list[SystemSpec]
    _commands: Commands

    @classmethod
    def new(cls) -> typing.Self:
        app = cls()
        app._rust_app = RustApp(
            num_pools=len(Component.component_ids),
            num_queries=Query.p_num_queries,
        )
        app._pools = {}
        app._startup_systems = []
        app._systems = []
        app._commands = Commands()
        return app

    def add_startup_system(self, system: System) -> None:
        query_args, other_args = self._get_system_args(system)
        self._startup_systems.append(
            SystemSpec(
                function=system,
                query_args=query_args,
                other_args=other_args,
            )
        )

    def add_system(self, system: System) -> None:
        query_args, other_args = self._get_system_args(system)
        self._systems.append(
            SystemSpec(
                function=system,
                query_args=query_args,
                other_args=other_args,
            )
        )

    def _get_system_args(
        self,
        system: typing.Callable[P, R],
    ) -> tuple[dict[str, Query], dict[str, NonQueryParameter]]:
        query_args: dict[str, Query] = {}
        other_args: dict[str, NonQueryParameter] = {}
        for name, parameter in inspect.signature(system).parameters.items():
            if typing.get_origin(parameter.annotation) is Query:
                (components,) = typing.get_args(parameter.annotation)
                first_component, *other_components = typing.get_args(
                    components
                )
                query_id = self._rust_app.add_query(
                    Component.component_ids[first_component],
                    tuple(
                        Component.component_ids[component]
                        for component in other_components
                    ),
                )
                query_args[name] = Query.p_new(query_id)

            elif parameter.annotation is Commands:
                other_args[name] = self._commands
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

    def run(self) -> None:
        for system in self._startup_systems:
            for query in system.query_args.values():
                query.p_result = self._rust_app.run_query(query.p_query_id)

            system.function(
                **system.query_args,
                **system.other_args,
            )

            self._commands.p_apply(
                app=self._rust_app,
                pools=self._pools,
            )

        for system in self._systems:
            for query in system.query_args.values():
                query.p_result = self._rust_app.run_query(query.p_query_id)

            system.function(
                **system.query_args,
                **system.other_args,
            )

            self._commands.p_apply(
                app=self._rust_app,
                pools=self._pools,
            )

    def add_component_pool(self, pool: ComponentPool[ComponentT]) -> None:
        component_id = Component.component_ids[type(pool.p_inner)]
        self._rust_app.add_component_pool(component_id, pool.p_capacity)
        self._pools[component_id] = pool
