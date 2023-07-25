import ecstasy as ecs
import numpy as np


class Position(ecs.Component):
    inner: ecs.Vec2


class Velocity(ecs.Component):
    inner: ecs.Vec2


class Generator(ecs.Resource):
    inner: np.random.Generator


class Params(ecs.Resource):
    num_circles: int
    max_position: float
    max_velocity: float


def spawn_circles(
    params: Params,
    generator: Generator,
    commands: ecs.Commands,
) -> None:
    positions = commands.spawn(Position, params.num_circles)
    velocities = commands.spawn(Velocity, params.num_circles)


def move_circles(query: ecs.Query[tuple[Position, Velocity]]) -> None:
    (position, velocity) = query.result()


def main() -> None:
    app = ecs.App()
    num_circles = 10
    app.add_resource(
        Params(
            num_circles=num_circles,
            max_position=10,
            max_velocity=2,
        )
    )
    app.add_resource(Generator(np.random.default_rng(55)))
    app.add_startup_system(spawn_circles)
    app.add_system(move_circles, ecs.Duration.from_millis(16))
    app.add_component_pool(Position.create_pool(num_circles))
    app.add_component_pool(Velocity.create_pool(num_circles))
    app.run()


if __name__ == "__main__":
    main()
