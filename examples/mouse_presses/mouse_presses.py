import xecs as xx
from xecs_pygame import Circle, PyGamePlugin


class LeftCircle(xx.Component):
    pass


class MiddleCircle(xx.Component):
    pass


class RightCircle(xx.Component):
    pass


class XInput1Circle(xx.Component):
    pass


class XInput2Circle(xx.Component):
    pass


def main() -> None:
    app = xx.RealTimeApp()
    app.add_plugin(PyGamePlugin())
    app.add_startup_system(spawn_circles)
    app.add_system(mouse_presses)
    app.add_pool(xx.Transform2.create_pool(5))
    app.add_pool(Circle.create_pool(5))
    app.add_pool(LeftCircle.create_pool(1))
    app.add_pool(MiddleCircle.create_pool(1))
    app.add_pool(RightCircle.create_pool(1))
    app.add_pool(XInput1Circle.create_pool(1))
    app.add_pool(XInput2Circle.create_pool(1))
    app.run()


def spawn_circles(
    world: xx.World,
    commands: xx.Commands,
) -> None:
    _, transformi, circlei = commands.spawn(
        (LeftCircle, xx.Transform2, Circle), 1
    )
    transform = world.get_view(xx.Transform2, transformi)
    transform.translation.x.fill(-320)
    circle = world.get_view(Circle, circlei)
    circle.radius.fill(60)
    circle.color.fill("red")

    _, transformi, circlei = commands.spawn(
        (MiddleCircle, xx.Transform2, Circle), 1
    )
    transform = world.get_view(xx.Transform2, transformi)
    transform.translation.x.fill(-160)
    circle = world.get_view(Circle, circlei)
    circle.radius.fill(60)
    circle.color.fill("green")

    _, transformi, circlei = commands.spawn(
        (RightCircle, xx.Transform2, Circle), 1
    )
    transform = world.get_view(xx.Transform2, transformi)
    transform.translation.x.fill(0)
    circle = world.get_view(Circle, circlei)
    circle.radius.fill(60)
    circle.color.fill("blue")

    _, transformi, circlei = commands.spawn(
        (XInput1Circle, xx.Transform2, Circle), 1
    )
    transform = world.get_view(xx.Transform2, transformi)
    transform.translation.x.fill(160)
    circle = world.get_view(Circle, circlei)
    circle.radius.fill(60)
    circle.color.fill("yellow")

    _, transformi, circlei = commands.spawn(
        (XInput2Circle, xx.Transform2, Circle), 1
    )
    transform = world.get_view(xx.Transform2, transformi)
    transform.translation.x.fill(320)
    circle = world.get_view(Circle, circlei)
    circle.radius.fill(60)
    circle.color.fill("purple")


def mouse_presses(
    mouse: xx.Mouse,
    left_circle_query: xx.Query[tuple[LeftCircle, Circle]],
    middle_circle_query: xx.Query[tuple[MiddleCircle, Circle]],
    right_circle_query: xx.Query[tuple[RightCircle, Circle]],
    xinput1_circle_query: xx.Query[tuple[XInput1Circle, Circle]],
    xinput2_circle_query: xx.Query[tuple[XInput2Circle, Circle]],
) -> None:
    _, left_circle = left_circle_query.result()
    if xx.MouseButton.left() in mouse.pressed:
        left_circle.width.fill(0)
    else:
        left_circle.width.fill(2)

    _, middle_circle = middle_circle_query.result()
    if xx.MouseButton.middle() in mouse.pressed:
        middle_circle.width.fill(0)
    else:
        middle_circle.width.fill(2)

    _, right_circle = right_circle_query.result()
    if xx.MouseButton.right() in mouse.pressed:
        right_circle.width.fill(0)
    else:
        right_circle.width.fill(2)

    _, xinput1_circle = xinput1_circle_query.result()
    if xx.MouseButton.xinput1() in mouse.pressed:
        xinput1_circle.width.fill(0)
    else:
        xinput1_circle.width.fill(2)

    _, xinput2_circle = xinput2_circle_query.result()
    if xx.MouseButton.xinput2() in mouse.pressed:
        xinput2_circle.width.fill(0)
    else:
        xinput2_circle.width.fill(2)


if __name__ == "__main__":
    main()
