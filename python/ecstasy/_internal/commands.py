import typing
from typing import TypeAlias

from ecstasy._internal.component import Component, ComponentPool
from ecstasy._internal.world import World

if typing.TYPE_CHECKING:
    from ecstasy.ecstasy import ComponentId

T = typing.TypeVar("T")


ComponentBundle: TypeAlias = (
    list[type[Component]] | tuple[type[Component], ...]
)


class Commands:
    __slots__ = "_world",
    _world: World

    def __init__(self, world: World) -> None:
        self._world = world

    def spawn(
        self,
        components: ComponentBundle,
        num: int,
    ) -> None:


    def p_apply(
        self,
        app: RustApp,
        pools: "dict[ComponentId, ComponentPool[typing.Any]]",
    ) -> None:
        # TODO: This function will probably need to be removed
        # once a World object is added.

        for components, num in self._spawn_commands:
            entity_component_ids = []
            for component in components:
                component_id = Component.component_ids[component]
                pool = pools[component_id]
                pool.p_spawn(num)
                entity_component_ids.append(component_id)
            app.spawn(entity_component_ids, num)

        self._spawn_commands = []
