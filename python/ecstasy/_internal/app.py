import inspect
import typing

from ecstasy._internal.commands import Commands
from ecstasy._internal.component import Component, ComponentPool, ComponentT
from ecstasy._internal.query import Query

P = typing.ParamSpec("P")
R = typing.TypeVar("R")


class SystemSchedule:
    def after(self, *system: typing.Callable[P, R]) -> None:
        pass


ComponentId: typing.TypeAlias = int


class SystemSignatureError(Exception):
    pass


class App:
    _pools: dict[type[Component], ComponentPool[typing.Any]]
    _startup_systems: list[typing.Callable[[typing.Any], typing.Any]]
    _systems: list[typing.Callable[[typing.Any], typing.Any]]

    @classmethod
    def new(cls) -> typing.Self:
        app = cls()
        app._pools = {}
        return app

    def add_startup_system(self, system: typing.Callable[P, R]) -> None:
        for name, parameter in inspect.signature(system).parameters.items():
            match parameter.annotation:
                case Query():
                    print("got query")
                case Commands():
                    print("got commands")
                case _:
                    raise SystemSignatureError(
                        f"an annotation in {name} is not Query or Command"
                    )

    def add_system(self, system: typing.Callable[P, R]) -> SystemSchedule:
        pass

    def run(self) -> None:
        pass

    def add_component_pool(self, pool: ComponentPool[ComponentT]) -> None:
        self._pools[type(pool.inner)] = pool
