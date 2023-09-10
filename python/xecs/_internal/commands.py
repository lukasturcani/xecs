from collections.abc import Iterable

from xecs._internal.component import Component
from xecs._internal.world import World
from xecs.xecs import ArrayViewIndices, RustApp


class Commands:
    """
    Make changes to the :class:`.World`.
    """

    __slots__ = "_app", "_world"

    _app: RustApp
    _world: World

    @staticmethod
    def p_new(app: RustApp, world: World) -> "Commands":
        commands = Commands()
        commands._app = app
        commands._world = world
        return commands

    def spawn(
        self,
        components: Iterable[type[Component]],
        num: int,
    ) -> list[ArrayViewIndices]:
        """
        Spawn new entities into the :class:`~xecs.World`.

        Parameters:
            components: The components the entities hold.
            num: The number of entities.
        Returns:
            For each component type in `components`, the indices
            of the new components in each component pool.
        See Also:
            * :meth:`.World.get_view`: The return indices can
              be used with this method to access the newly spawned
              entities.
        """
        indices = []
        component_ids = []
        for component in components:
            component_id = Component.component_ids[component]
            pool = self._world.p_get_pool(component)
            indices.append(pool.p_spawn(num))
            component_ids.append(component_id)
        self._app.spawn(component_ids, num)
        return indices
