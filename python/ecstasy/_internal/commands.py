from collections.abc import Iterable

from ecstasy._internal.component import Component
from ecstasy._internal.world import World
from ecstasy.ecstasy import ArrayViewIndices, RustApp


class Commands:
    __slots__ = "_app", "_world"

    def __init__(self, app: RustApp, world: World) -> None:
        self._app = app
        self._world = world

    def spawn(
        self,
        components: Iterable[type[Component]],
        num: int,
    ) -> list[ArrayViewIndices]:
        indices = []
        component_ids = []
        for component in components:
            component_id = Component.component_ids[component]
            pool = self._world.get_component_pool(component)
            indices.append(pool.p_spawn(num))
            component_ids.append(component_id)
        self._app.spawn(component_ids, num)
        return indices
