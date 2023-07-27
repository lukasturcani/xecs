from typing import cast

from xecs._internal.component import Component, ComponentPool, ComponentT
from xecs._internal.resource import Resource, ResourceT
from xecs.xecs import ArrayViewIndices


class World:
    def __init__(self) -> None:
        self._pools: dict[type[Component], ComponentPool[Component]] = {}
        self._resources: dict[type[Resource], Resource] = {}

    def has_resource(self, resource: type[Resource]) -> bool:
        return resource in self._resources

    def get_resource(self, resource: type[ResourceT]) -> ResourceT:
        return cast(ResourceT, self._resources[resource])

    def add_resource(self, resource: Resource) -> None:
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
        component = type(pool.p_component)
        self._pools[component] = cast(ComponentPool[Component], pool)

    def get_view(
        self,
        component: type[ComponentT],
        indices: ArrayViewIndices | None = None,
    ) -> ComponentT:
        if indices is None:
            return self.p_get_pool(component).p_component
        return self.p_get_pool(component).p_component.p_new_view_with_indices(
            indices
        )
