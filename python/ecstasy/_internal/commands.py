import typing

from ecstasy._internal.component import Component, ComponentPool
from ecstasy._internal.world import World

if typing.TYPE_CHECKING:
    from ecstasy.ecstasy import ComponentId


class Commands:
    __slots__ = ("_world",)
    _world: World

    def __init__(self, world: World) -> None:
        self._world = world

    def spawn(
        self,
        component: type[Component],
        num: int,
    ) -> Component:
        component_id = Component.component_ids[component]
        self._world.pools[component_id].p_spawn(num)

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
