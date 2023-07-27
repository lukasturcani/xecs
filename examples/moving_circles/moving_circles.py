import numpy as np
import pygame
import xecs as xx


class Position(xx.Component):
    value: xx.Vec2

    def fill_random(
        self,
        generator: np.random.Generator,
        scale: float,
    ) -> None:
        num = len(self.value.x)
        self.value.x.fill(generator.random(num, dtype=np.float32) * scale)
        self.value.y.fill(generator.random(num, dtype=np.float32) * scale)


class Velocity(xx.Component):
    value: xx.Vec2

    def fill_random(
        self,
        generator: np.random.Generator,
        scale: float,
    ) -> None:
        num = len(self.value.x)
        self.value.x.fill(generator.random(num, dtype=np.float32) * scale)
        self.value.y.fill(generator.random(num, dtype=np.float32) * scale)


class Generator(xx.Resource):
    value: np.random.Generator


class Params(xx.Resource):
    num_circles: int
    max_position: float
    max_velocity: float


class Display(xx.Resource):
    surface: pygame.Surface


def main() -> None:
    pygame.init()
    app = xx.App()
    num_circles = 10
    app.add_resource(
        Params(
            num_circles=num_circles,
            max_position=200,
            max_velocity=50,
        )
    )
    app.add_resource(Generator(np.random.default_rng(55)))
    app.add_resource(Display(pygame.display.set_mode((640, 640))))
    app.add_startup_system(spawn_circles)
    app.add_system(move_circles, xx.Duration.from_millis(16))
    app.add_system(show_circles)
    app.add_pool(Position.create_pool(num_circles))
    app.add_pool(Velocity.create_pool(num_circles))
    app.run()


def spawn_circles(
    params: Params,
    generator: Generator,
    world: xx.World,
    commands: xx.Commands,
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


def move_circles(
    params: Params,
    query: xx.Query[tuple[Position, Velocity]],
) -> None:
    (position, velocity) = query.result()
    position.value += velocity.value * (
        xx.Duration.from_millis(16).as_nanos() / 1e9
    )
    velocity.value.x[position.value.x > params.max_position] *= -1
    velocity.value.y[position.value.y > params.max_position] *= -1
    velocity.value.x[position.value.x < 0] *= -1
    velocity.value.y[position.value.y < 0] *= -1


def show_circles(
    display: Display,
    query: xx.Query[tuple[Position]],
) -> None:
    (position_,) = query.result()
    display.surface.fill("purple")
    position = position_.value.numpy()
    for i in range(position.shape[1]):
        x, y = map(float, position[:, i])
        pygame.draw.circle(display.surface, "green", (x, y), 10)
    pygame.display.flip()


if __name__ == "__main__":
    main()
