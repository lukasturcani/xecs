from dataclasses import dataclass

import numpy as np
import numpy.typing as npt
import pygame
import xecs as xx
from xecs_pygame import Circle, PyGamePlugin, Rectangle, Text


class Velocity(xx.Component):
    value: xx.Vec2


class Score(xx.Resource):
    value: int


class Params(xx.Resource):
    num_colliders_per_row: int
    num_collider_rows: int
    num_colliders: int
    box_size: int
    circle_radius: int
    bounds_width: int
    paddle_x: int


class Paddle(xx.Component):
    pass


class Collider(xx.Component):
    pass


def main() -> None:
    app = xx.RealTimeApp()
    app.add_plugin(PyGamePlugin())
    num_circles = 1
    num_paddles = 1
    num_scoreboards = 1
    num_bounds = 1
    num_colliders_per_row = 8
    num_collider_rows = 7
    params = Params(
        num_colliders_per_row=num_colliders_per_row,
        num_collider_rows=num_collider_rows,
        num_colliders=num_colliders_per_row * num_collider_rows,
        box_size=1000,
        circle_radius=15,
        bounds_width=2,
        paddle_x=200,
    )
    app.add_startup_system(spawn_circle)
    app.add_startup_system(spawn_paddle)
    app.add_startup_system(spawn_colliders)
    app.add_startup_system(spawn_scoreboard)
    app.add_startup_system(spawn_bounds)
    app.add_system(update_circle_position)
    app.add_system(move_paddle)
    app.add_system(handle_collider_collision)
    app.add_system(handle_paddle_collision)
    app.add_system(update_scoreboard)
    app.add_pool(
        xx.Transform2.create_pool(
            params.num_colliders
            + num_circles
            + num_paddles
            + num_scoreboards
            + num_bounds
        )
    )
    app.add_pool(Velocity.create_pool(num_circles))
    app.add_pool(Circle.create_pool(num_circles))
    app.add_pool(
        Rectangle.create_pool(num_bounds + params.num_colliders + num_paddles)
    )
    app.add_pool(Collider.create_pool(params.num_colliders))
    app.add_pool(Text.create_pool(num_scoreboards))
    app.add_pool(Paddle.create_pool(num_paddles))
    app.add_resource(Score(value=0))
    app.add_resource(params)
    app.run()


def spawn_circle(
    params: Params, commands: xx.Commands, world: xx.World
) -> None:
    transformi, velocityi, circlei = commands.spawn(
        (xx.Transform2, Velocity, Circle), 1
    )
    transform = world.get_view(xx.Transform2, transformi)
    transform.translation.y.fill(-250)

    velocity = world.get_view(Velocity, velocityi)
    velocity.value.x.fill(5)
    velocity.value.y.fill(5)

    circle = world.get_view(Circle, circlei)
    circle.radius.fill(params.circle_radius)
    circle.color.fill("red")


def spawn_paddle(
    params: Params, commands: xx.Commands, world: xx.World
) -> None:
    transformi, rectanglei, _ = commands.spawn(
        (xx.Transform2, Rectangle, Paddle), 1
    )
    transform = world.get_view(xx.Transform2, transformi)
    transform.translation.x.fill(-params.paddle_x / 2)
    transform.translation.y.fill(-300)
    rectangle = world.get_view(Rectangle, rectanglei)
    rectangle.length_x.fill(params.paddle_x)
    rectangle.length_y.fill(20)


def spawn_colliders(
    params: Params, commands: xx.Commands, world: xx.World
) -> None:
    box_size = 1000
    collider_padding = 5
    total_x_padding = (1 + params.num_colliders_per_row) * collider_padding
    collider_x = (box_size - total_x_padding) // params.num_colliders_per_row
    collider_y = 30

    transformi, rectanglei, _ = commands.spawn(
        (xx.Transform2, Rectangle, Collider), 56
    )
    transform = world.get_view(xx.Transform2, transformi)
    transform.translation.x.fill(
        np.tile(
            np.arange(params.num_colliders_per_row, dtype=np.float32),
            params.num_collider_rows,
        )
        * (collider_x + collider_padding)
        + collider_padding
        - (box_size / 2)
    )
    transform.translation.y.fill(
        np.arange(params.num_colliders, dtype=np.float32)
        // params.num_colliders_per_row
        * (collider_y + collider_padding)
        + (box_size / 2)
        - (params.num_collider_rows * (collider_y + collider_padding))
    )

    rectangle = world.get_view(Rectangle, rectanglei)
    rectangle.color.fill("blue")
    rectangle.length_x.fill(collider_x)
    rectangle.length_y.fill(collider_y)


def spawn_scoreboard(commands: xx.Commands, world: xx.World) -> None:
    transformi, texti = commands.spawn((xx.Transform2, Text), 1)
    transform = world.get_view(xx.Transform2, transformi)
    transform.translation.x.fill(-800)
    transform.translation.y.fill(400)

    text = world.get_view(Text, texti)
    text.font.fill(pygame.font.SysFont("monospace", 32, True))
    text.text.fill("Score: 0")


def spawn_bounds(
    params: Params, commands: xx.Commands, world: xx.World
) -> None:
    transformi, rectanglei = commands.spawn((xx.Transform2, Rectangle), 1)
    transform = world.get_view(xx.Transform2, transformi)
    transform.translation.x.fill(-params.box_size / 2)
    transform.translation.y.fill(-params.box_size / 2)

    rectangle = world.get_view(Rectangle, rectanglei)
    rectangle.length_x.fill(params.box_size)
    rectangle.length_y.fill(params.box_size)
    rectangle.width.fill(params.bounds_width)
    rectangle.color.fill("gray")


def update_circle_position(
    params: Params,
    circle_query: xx.Query[tuple[xx.Transform2, Velocity]],
) -> None:
    transform, velocity = circle_query.result()
    transform.translation += velocity.value

    velocity.value.x[
        transform.translation.x - params.circle_radius - params.bounds_width
        < -params.box_size / 2
        or transform.translation.x + params.circle_radius + params.bounds_width
        > params.box_size / 2
    ] *= -1
    velocity.value.y[
        transform.translation.y - params.circle_radius - params.bounds_width
        < -params.box_size / 2
        or transform.translation.y + params.circle_radius + params.bounds_width
        > params.box_size / 2
    ] *= -1


def move_paddle(
    params: Params,
    paddle_query: xx.Query[tuple[xx.Transform2, Paddle]],
    keyboard: xx.Keyboard,
) -> None:
    transform, _ = paddle_query.result()
    if (
        xx.KeyboardButton.LEFT in keyboard.pressed
        and transform.translation.x.get(0)
        > (-params.box_size / 2) + (params.bounds_width * 2)
    ):
        transform.translation.x -= 4
    if (
        xx.KeyboardButton.RIGHT in keyboard.pressed
        and transform.translation.x.get(0) + params.paddle_x
        < (params.box_size / 2) - (params.bounds_width * 2)
    ):
        transform.translation.x += 4


def update_scoreboard(score: Score, scoreboard_query: xx.Query[Text]) -> None:
    text = scoreboard_query.result()
    text.text.fill(f"Score: {score.value}")


def handle_collider_collision(
    params: Params,
    commands: xx.Commands,
    collider_query: xx.Query[
        tuple[xx.EntityId, xx.Transform2, Rectangle, Collider]
    ],
    circle_query: xx.Query[tuple[xx.Transform2, Velocity, Circle]],
    score: Score,
) -> None:
    entity_id, collider_transform, rectangle, _ = collider_query.result()
    circle_transform, velocity, _ = circle_query.result()

    collisions = get_collisions(
        circle_transform.translation,
        params.circle_radius,
        collider_transform.translation,
        rectangle.length_x,
        rectangle.length_y,
    )
    if np.any(collisions.bottom):
        commands.despawn(entity_id[collisions.bottom])
        velocity.value.y[velocity.value.y > 0] *= -1
        score.value += 1
    elif np.any(collisions.top):
        commands.despawn(entity_id[collisions.top])
        velocity.value.y[velocity.value.y < 0] *= -1
        score.value += 1
    elif np.any(collisions.left):
        commands.despawn(entity_id[collisions.left])
        velocity.value.x[velocity.value.x < 0] *= -1
        score.value += 1
    elif np.any(collisions.right):
        commands.despawn(entity_id[collisions.right])
        velocity.value.x[velocity.value.x > 0] *= -1
        score.value += 1


@dataclass(frozen=True, slots=True)
class Collisions:
    top: npt.NDArray[np.bool_]
    bottom: npt.NDArray[np.bool_]
    left: npt.NDArray[np.bool_]
    right: npt.NDArray[np.bool_]


def get_collisions(
    circle_position: xx.Vec2,
    circle_radius: int,
    rectangle_position: xx.Vec2,
    rectangle_x: xx.Float,
    rectangle_y: xx.Float,
) -> Collisions:
    rectangle_end_x = rectangle_position.x + rectangle_x

    x_check = (
        (rectangle_position.x <= circle_position.x.get(0) + circle_radius)
        & (circle_position.x.get(0) + circle_radius <= rectangle_end_x)
    ) | (
        (rectangle_position.x <= circle_position.x.get(0) - circle_radius)
        & (circle_position.x.get(0) - circle_radius <= rectangle_end_x)
    )
    rectangle_end_y = rectangle_position.y + rectangle_y
    y_check = (
        (rectangle_position.y <= circle_position.y.get(0) + circle_radius)
        & (circle_position.y.get(0) + circle_radius <= rectangle_end_y)
    ) | (
        (rectangle_position.y <= circle_position.y.get(0) - circle_radius)
        & (circle_position.y.get(0) - circle_radius <= rectangle_end_y)
    )
    collision = x_check & y_check
    top_collision = collision & (
        circle_position.y.get(0) + circle_radius > rectangle_end_y
    )
    bottom_collision = collision & (
        circle_position.y.get(0) - circle_radius < rectangle_position.y
    )
    left_collision = collision & (
        circle_position.x.get(0) + circle_radius > rectangle_end_x
    )
    right_collision = collision & (
        circle_position.x.get(0) - circle_radius < rectangle_position.x
    )
    return Collisions(
        top=top_collision,
        bottom=bottom_collision,
        left=left_collision,
        right=right_collision,
    )


def handle_paddle_collision(
    params: Params,
    paddle_query: xx.Query[tuple[xx.Transform2, Rectangle, Paddle]],
    circle_query: xx.Query[tuple[xx.Transform2, Velocity, Circle]],
) -> None:
    paddle_transform, rectangle, _ = paddle_query.result()
    circle_transform, velocity, _ = circle_query.result()

    collisions = get_collisions(
        circle_transform.translation,
        params.circle_radius,
        paddle_transform.translation,
        rectangle.length_x,
        rectangle.length_y,
    )
    if np.any(collisions.bottom):
        velocity.value.y[velocity.value.y > 0] *= -1
    elif np.any(collisions.top):
        velocity.value.y[velocity.value.y < 0] *= -1
    elif np.any(collisions.left):
        velocity.value.x[velocity.value.x < 0] *= -1
    elif np.any(collisions.right):
        velocity.value.x[velocity.value.x > 0] *= -1


if __name__ == "__main__":
    main()
