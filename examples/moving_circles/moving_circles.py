import ecstasy as ecs
import numpy as np
import pygame


class Position(ecs.Component):
    value: ecs.Vec2

    def fill_random(
        self,
        generator: np.random.Generator,
        scale: float,
    ) -> None:
        num = len(self.value.x)
        self.value.x.fill(generator.random(num, dtype=np.float32) * scale)
        self.value.y.fill(generator.random(num, dtype=np.float32) * scale)


class Velocity(ecs.Component):
    value: ecs.Vec2

    def fill_random(
        self,
        generator: np.random.Generator,
        scale: float,
    ) -> None:
        num = len(self.value.x)
        self.value.x.fill(generator.random(num, dtype=np.float32) * scale)
        self.value.y.fill(generator.random(num, dtype=np.float32) * scale)


class Generator(ecs.Resource):
    value: np.random.Generator


class Params(ecs.Resource):
    num_circles: int
    max_position: float
    max_velocity: float


class Display(ecs.Resource):
    surface: pygame.Surface


def spawn_circles(
    params: Params,
    generator: Generator,
    world: ecs.World,
    commands: ecs.Commands,
) -> None:
    positioni, velocityi = commands.spawn(
        components=(Position, Velocity),
        num=params.num_circles,
    )
    world.get_view(Position, positioni).fill_random(
        generator.value, params.max_position
    )
    world.get_view(Velocity, velocityi).fill_random(
        generator.value, params.max_velocity
    )


def move_circles(query: ecs.Query[tuple[Position, Velocity]]) -> None:
    (position, velocity) = query.result()
    position.value += velocity.value * (
        ecs.Duration.from_millis(16).as_nanos() / 1e9
    )


def show_circles(
    display: Display,
    query: ecs.Query[tuple[Position]],
) -> None:
    (position_,) = query.result()
    display.surface.fill("purple")
    position = position_.value.numpy()
    for i in range(position.shape[1]):
        x, y = map(float, position[:, i])
        pygame.draw.circle(display.surface, "green", (x, y), 10)
    pygame.display.flip()


def main() -> None:
    pygame.init()
    app = ecs.App()
    num_circles = 10
    app.add_resource(
        Params(
            num_circles=num_circles,
            max_position=200,
            max_velocity=20,
        )
    )
    app.add_resource(Generator(np.random.default_rng(55)))
    app.add_resource(Display(pygame.display.set_mode((640, 640))))
    app.add_startup_system(spawn_circles)
    app.add_system(move_circles, ecs.Duration.from_millis(16))
    app.add_system(show_circles)
    app.add_pool(Position.create_pool(num_circles))
    app.add_pool(Velocity.create_pool(num_circles))
    app.run()


if __name__ == "__main__":
    main()
