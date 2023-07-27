from collections.abc import Iterable

from xecs._internal.component import Component
from xecs._internal.world import World
from xecs.xecs import ArrayViewIndices, RustApp


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
            pool = self._world.p_get_pool(component)
            indices.append(pool.p_spawn(num))
            component_ids.append(component_id)
        self._app.spawn(component_ids, num)
        return indices
