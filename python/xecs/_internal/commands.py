from collections.abc import Iterable

from xecs._internal.component import Component
from xecs._internal.entity_id import EntityId
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

        return self._app.spawn(
            [Component.p_component_ids[component] for component in components],
            num,
        )

    def despawn(self, entity_ids: EntityId) -> None:
        self._app.despawn(entity_ids.value)
