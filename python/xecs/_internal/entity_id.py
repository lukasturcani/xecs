from xecs._internal.component import Component
from xecs.xecs import UInt32


class EntityId(Component):
    value: UInt32
