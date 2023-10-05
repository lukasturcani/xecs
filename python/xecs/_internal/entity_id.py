from xecs._internal.component import Component
from xecs.xecs import UInt32


class EntityId(Component):
    """
    A component that stores the entity id.
    """

    value: UInt32
    """The entity id."""
