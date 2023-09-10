from typing import cast

from xecs._internal.component import Component, ComponentPool, ComponentT
from xecs._internal.resource import Resource, ResourceT
from xecs.xecs import ArrayViewIndices


class World:
    """
    Stores and manages all entities, components, and resources.
    """

    def __init__(self) -> None:
        self._pools: dict[type[Component], ComponentPool[Component]] = {}
        self._resources: dict[type[Resource], Resource] = {}

    def has_resource(self, resource: type[Resource]) -> bool:
        """
        Check if the world has a resource.

        Parameters:
            resource: The type of the resource.
        Returns:
            Whether the world has the resource.
        """
        return resource in self._resources

    def get_resource(self, resource: type[ResourceT]) -> ResourceT:
        """
        Get a resource.

        Parameters:
            resource: The type of the resource.
        Returns:
            The resource.
        """
        return cast(ResourceT, self._resources[resource])

    def add_resource(self, resource: Resource) -> None:
        """
        Add a resource to the world.

        Parameters:
            resource: The resource to add.
        """
        self._resources[type(resource)] = resource

    def p_get_pool(
        self,
        component: type[ComponentT],
    ) -> ComponentPool[ComponentT]:
        return cast(
            ComponentPool[ComponentT],
            self._pools[component],
        )

    def add_pool(self, pool: ComponentPool[ComponentT]) -> None:
        """
        Add a component pool to the world.

        Parameters:
            pool: The component pool to add.
        """
        component = type(pool.p_component)
        self._pools[component] = cast(ComponentPool[Component], pool)

    def get_view(
        self,
        component: type[ComponentT],
        indices: ArrayViewIndices | None = None,
    ) -> ComponentT:
        """
        Get a view of some components.

        Parameters:
            component:
                The component which you want to view.
            indices:
                The indices specifying which entities in the
                component pool you want to view.
        Returns:
            A component view of your selected entities.
        """
        if indices is None:
            return self.p_get_pool(component).p_component
        return self.p_get_pool(component).p_component.p_new_view_with_indices(
            indices
        )
