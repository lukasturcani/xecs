import typing

from ecstasy._internal.component import Component, ComponentId, ComponentPool
from ecstasy.ecstasy import RustApp

T = typing.TypeVar("T")


ComponentBundle: typing.TypeAlias = (
    list[type[Component]] | tuple[type[Component], ...]
)


class Commands:
    _spawn_commands: list[tuple[ComponentBundle, int]]

    def __init__(self) -> None:
        self._spawn_commands = []

    def spawn(
        self,
        components: ComponentBundle,
        num: int,
    ) -> None:
        # TODO: Commands should not be applied immediately
        self._spawn_commands.append((components, num))

    def p_apply(
        self,
        app: RustApp,
        component_ids: dict[type[Component], ComponentId],
        pools: dict[ComponentId, ComponentPool[typing.Any]],
    ) -> None:
        # TODO: This function will probably need to be removed
        # once a World object is added.

        for components, num in self._spawn_commands:
            entity_component_ids = []
            for component in components:
                component_id = component_ids[component]
                pool = pools[component_id]
                pool.p_spawn(num)
                entity_component_ids.append(component_id)
            app.spawn(entity_component_ids, num)

        self._spawn_commands = []
