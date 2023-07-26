import typing
from typing import cast

from ecstasy._internal.component import Component, ComponentPool, ComponentT
from ecstasy._internal.resource import Resource, ResourceT

if typing.TYPE_CHECKING:
    from ecstasy.ecstasy import ComponentId


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

    def get_component_pool(
        self,
        component: type[ComponentT],
    ) -> ComponentPool[ComponentT]:
        return cast(
            ComponentPool[ComponentT],
            self._pools[component],
        )

    def add_component_pool(self, pool: ComponentPool[ComponentT]) -> None:
        component = type(pool.p_component)
        self._pools[component] = cast(ComponentPool[Component], pool)
