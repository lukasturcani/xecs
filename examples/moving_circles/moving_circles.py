import numpy as np
import xecs as xx
from xecs_pygame import Circle, PyGamePlugin


class Velocity(xx.Component):
    value: xx.Vec2

    def fill_random(
        self,
        generator: np.random.Generator,
        scale: float,
    ) -> None:
        self.value.fill(
            generator.random((2, len(self)), dtype=np.float32) * scale
        )


class Params(xx.Resource):
    num_circles: int
    max_position: float
    max_velocity: float
    generator: np.random.Generator
    time_step: xx.Duration


def main() -> None:
    app = xx.RealTimeApp()
    app.add_plugin(PyGamePlugin())
    num_circles = 20
    params = Params(
        num_circles=num_circles,
        max_position=200,
        max_velocity=50,
        generator=np.random.default_rng(9),
        time_step=xx.Duration.from_millis(16),
    )
    app.add_resource(params)
    app.add_startup_system(spawn_circles)
    app.add_system(move_circles, params.time_step)
    app.add_pool(xx.Transform2.create_pool(num_circles))
    app.add_pool(Velocity.create_pool(num_circles))
    app.add_pool(Circle.create_pool(num_circles))
    app.run()


def spawn_circles(
    params: Params,
    world: xx.World,
    commands: xx.Commands,
) -> None:
    positioni, velocityi, _ = commands.spawn(
        components=(xx.Transform2, Velocity, Circle),
        num=params.num_circles,
    )
    world.get_view(xx.Transform2, positioni).fill_random(
        params.generator, params.max_position
    )
    world.get_view(Velocity, velocityi).fill_random(
        params.generator, params.max_velocity
    )


def move_circles(
    params: Params,
    query: xx.Query[tuple[xx.Transform2, Velocity]],
) -> None:
    (transform, velocity) = query.result()
    transform.translation += velocity.value * (
        params.time_step.as_nanos() / 1e9
    )
    velocity.value.x[transform.translation.x > params.max_position] *= -1
    velocity.value.y[transform.translation.y > params.max_position] *= -1
    velocity.value.x[transform.translation.x < 0] *= -1
    velocity.value.y[transform.translation.y < 0] *= -1


if __name__ == "__main__":
    main()
