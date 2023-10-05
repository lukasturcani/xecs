from typing import cast

from xecs._internal.component import Components, ComponentT
from xecs._internal.resource import Resource, ResourceT
from xecs.xecs import ArrayViewIndices


class World:
    """
    Stores and manages all entities, components, and resources.
    """

    def __init__(self) -> None:
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
        view = self.get_resource(Components).get_component(component)
        if indices is None:
            return view
        return view.p_new_view_with_indices(indices)
