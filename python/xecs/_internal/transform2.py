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
        min_translation: tuple[float, float] = (0.0, 0.0),
        max_translation: tuple[float, float] = (10.0, 0.0),
    ) -> None:
        """
        Create a transforms with random values.

        Parameters:
            generator:
                The random number generator to use.
            min_translation:
                The minimum (x, y) coordinate.
            max_translation:
                The maximum (x, y) coordinate.
        """
        min_x, min_y = min_translation
        max_x, max_y = max_translation
        x_diff = max_x - min_x
        y_diff = max_y - min_y
        self.translation.x.fill(
            generator.random(len(self), dtype=np.float32) * x_diff + min_x
        )
        self.translation.y.fill(
            generator.random(len(self), dtype=np.float32) * y_diff + min_y
        )
        self.rotation.fill(
            generator.random(len(self), dtype=np.float32) * 2 * np.pi
        )
