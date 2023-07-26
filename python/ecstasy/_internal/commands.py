from collections.abc import Iterable

from ecstasy._internal.component import Component
from ecstasy._internal.world import World
from ecstasy.ecstasy import ArrayViewIndices


class Commands:
    __slots__ = ("_world",)
    _world: World

    def __init__(self, world: World) -> None:
        self._world = world

    def spawn(
        self,
        components: Iterable[type[Component]],
        num: int,
    ) -> list[ArrayViewIndices]:
        indices = []
        for component in components:
            component_id = Component.component_ids[component]
            pool = self._world.pools[component_id]
            indices.append(pool.p_spawn(num))
        return indices
