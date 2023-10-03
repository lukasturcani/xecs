import xecs as xx
from xecs_pygame import Circle, Display, PyGamePlugin


def main() -> None:
    app = xx.RealTimeApp()
    app.add_plugin(PyGamePlugin())
    app.add_startup_system(spawn_circle)
    app.add_system(update_circle)
    app.add_pool(Circle.create_pool(1))
    app.add_pool(xx.Transform2.create_pool(1))
    app.run()


def spawn_circle(world: xx.World, commands: xx.Commands) -> None:
    _, circlei = commands.spawn((xx.Transform2, Circle), 1)
    circle = world.get_view(Circle, circlei)
    circle.radius.fill(100)
    circle.color.fill("purple")


def update_circle(
    mouse: xx.Mouse,
    circle_query: xx.Query[tuple[xx.Transform2, Circle]],
    display: Display,
) -> None:
    transform, _ = circle_query.result()
    x, y = display.surface.get_size()
    transform.translation.x.fill(mouse.position[0] - x / 2)
    transform.translation.y.fill(-mouse.position[1] + y / 2)


if __name__ == "__main__":
    main()
