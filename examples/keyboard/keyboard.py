import pygame
import xecs as xx
from xecs_pygame import PyGamePlugin, Text


def main() -> None:
    app = xx.RealTimeApp()
    app.add_plugin(PyGamePlugin())
    app.add_startup_system(spawn_text)
    app.add_system(move_text)
    app.add_pool(xx.Transform2.create_pool(1))
    app.add_pool(Text.create_pool(1))
    app.run()


def spawn_text(
    commands: xx.Commands,
    world: xx.World,
) -> None:
    _, texti = commands.spawn((xx.Transform2, Text), 1)
    text = world.get_view(Text, texti)
    text.font.fill(pygame.font.SysFont("monospace", 32, True))
    text.text.fill("Move with me with WASD")


def move_text(
    keyboard: xx.Keyboard,
    text_query: xx.Query[xx.Transform2],
) -> None:
    transform = text_query.result()

    if xx.KeyboardButton.W in keyboard.pressed:
        transform.translation.y += 10
    if xx.KeyboardButton.S in keyboard.pressed:
        transform.translation.y -= 10
    if xx.KeyboardButton.A in keyboard.pressed:
        transform.translation.x -= 10
    if xx.KeyboardButton.D in keyboard.pressed:
        transform.translation.x += 10


if __name__ == "__main__":
    main()
