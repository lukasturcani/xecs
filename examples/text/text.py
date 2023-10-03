import pygame
import xecs as xx
from xecs_pygame import PyGamePlugin, Text


def main() -> None:
    app = xx.RealTimeApp()
    app.add_plugin(PyGamePlugin())
    app.add_startup_system(spawn_text)
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
    text.text.fill("Hello, world!")


if __name__ == "__main__":
    main()
