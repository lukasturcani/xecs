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


class System(typing.Protocol):
    def __call__(self, *args: SystemParameter) -> None:
        pass


class SystemSpec:
    __slots__ = "function", "kwargs"

    def __init__(
        self,
        function: System,
        kwargs: dict[str, SystemParameter],
    ) -> None:
        self.function = function
        self.kwargs = kwargs


class App:
    _rust_app: RustApp
    _pools: dict[ComponentId, ComponentPool[typing.Any]]
    _startup_systems: list[SystemSpec]
    _systems: list[SystemSpec]
    _commands: Commands

    @classmethod
    def new(cls) -> typing.Self:
        app = cls()
        app._rust_app = RustApp()
        app._pools = {}
        app._startup_systems = []
        app._systems = []
        app._commands = Commands()
        return app

    def add_startup_system(self, system: System) -> None:
        self._startup_systems.append(
            SystemSpec(
                function=system,
                kwargs=self._get_system_kwargs(system),
            )
        )

    def add_system(self, system: System) -> None:
        self._systems.append(
            SystemSpec(
                function=system,
                kwargs=self._get_system_kwargs(system),
            )
        )

    def _get_system_kwargs(
        self, system: typing.Callable[P, R]
    ) -> dict[str, SystemParameter]:
        kwargs: dict[str, SystemParameter] = {}
        for name, parameter in inspect.signature(system).parameters.items():
            if typing.get_origin(parameter.annotation) is Query:
                (components,) = typing.get_args(parameter.annotation)
                kwargs[name] = Query.p_new(typing.get_args(components))
            elif parameter.annotation is Commands:
                kwargs[name] = self._commands
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
        return kwargs

    def run(self) -> None:
        for system in self._startup_systems:
            system.function(**system.kwargs)

        self._commands.p_apply(
            app=self._rust_app,
            pools=self._pools,
        )

        for system in self._systems:
            system.function(**system.kwargs)

    def add_component_pool(self, pool: ComponentPool[ComponentT]) -> None:
        component_id = Component.component_ids[type(pool.p_inner)]
        self._rust_app.add_component_pool(component_id, pool.p_capacity)
        self._pools[component_id] = pool
