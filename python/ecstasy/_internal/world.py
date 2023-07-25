import typing

from ecstasy._internal.component import Component, ComponentPool
from ecstasy._internal.resource import Resource

if typing.TYPE_CHECKING:
    from ecstasy.ecstasy import ComponentId


class World:
    pools: "dict[ComponentId, ComponentPool[Component]]"
    resources: dict[type[Resource], Resource]
