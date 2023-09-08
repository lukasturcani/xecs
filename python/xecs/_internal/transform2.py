import numpy as np

from xecs._internal.component import Component
from xecs._internal.vec2 import Vec2
from xecs.xecs import Float32


class Transform2(Component):
    translation: Vec2
    rotation: Float32

    def fill_random(
        self,
        generator: np.random.Generator,
        scale: float,
    ) -> None:
        self.translation.fill(
            (generator.random((2, len(self)), dtype=np.float32) - 0.5) * scale
        )
        self.rotation.fill(
            generator.random(len(self), dtype=np.float32) * 2 * np.pi
        )
