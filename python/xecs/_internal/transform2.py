import numpy as np

from xecs._internal.component import Component
from xecs._internal.vec2 import Vec2
from xecs.xecs import Float32


class Transform2(Component):
    """
    Describes the position and orientation of an entity in 2D space.
    """

    translation: Vec2
    """The translation of the entity."""
    rotation: Float32
    """The rotation of the entity."""

    def fill_random(
        self,
        generator: np.random.Generator,
        scale: float,
    ) -> None:
        """
        Create a transforms with random values.

        The transforms will be centered around the origin.

        Parameters:
            generator:
                The random number generator to use.
            scale:
                The maximum value for the translation, both negative and
                positive.
        """
        self.translation.fill(
            (generator.random((2, len(self)), dtype=np.float32) - 0.5) * scale
        )
        self.rotation.fill(
            generator.random(len(self), dtype=np.float32) * 2 * np.pi
        )
