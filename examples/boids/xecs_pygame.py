from typing import Callable

import numpy as np
import pygame
import xecs as xx


class PyGamePlugin(xx.RealTimeAppPlugin):
    def __init__(self, window_size: tuple[int, int] = (640, 640)) -> None:
        super().__init__()
        self._window_size = window_size

    def build(self, app: xx.RealTimeApp) -> None:
        pygame.init()
        app.add_resource(
            Display(pygame.display.set_mode(self._window_size), "black", [])
        )
        app.add_system(draw)
        app.add_pool(Rectangle.create_pool(0))
        app.add_pool(Polygon.create_pool(0))


class Display(xx.Resource):
    surface: pygame.Surface
    color: str
    hooks: list[Callable[[], None]]


class Rectangle(xx.Component):
    size: xx.PyField[tuple[int, int]]
    color: xx.PyField[str]
    width: xx.PyField[int]


class Polygon(xx.Component):
    vertices: xx.PyField[list[tuple[float, float]]]
    color: xx.PyField[str]


def draw(
    display: Display,
    polygon_query: xx.Query[tuple[xx.Transform2, Polygon]],
    rectangle_query: xx.Query[tuple[xx.Transform2, Rectangle]],
) -> None:
    scale = 2
    (transform, polygon) = polygon_query.result()
    display.surface.fill(display.color)
    display_origin = np.array(display.surface.get_size()) / 2
    for i in range(len(transform)):
        x = transform.translation.x.get(i)
        y = transform.translation.y.get(i)
        angle = transform.rotation.get(i)
        r = [
            [np.cos(angle), -np.sin(angle)],
            [np.sin(angle), np.cos(angle)],
        ]
        vertices = np.array(polygon.vertices.get(i)).T
        boid_polygon = (
            np.multiply(scale, [x, y]) + display_origin + (r @ vertices).T
        )
        pygame.draw.polygon(
            display.surface, polygon.color.get(i), boid_polygon.tolist()
        )

    (transform, rectangle) = rectangle_query.result()
    for i in range(len(transform)):
        x = transform.translation.x.get(i)
        y = transform.translation.y.get(i)
        (w, h) = rectangle.size.get(i)
        pygame.draw.rect(
            display.surface,
            rectangle.color.get(i),
            pygame.Rect(
                x * scale + display_origin[0],
                y * scale + display_origin[1],
                scale * w,
                scale * h,
            ),
            rectangle.width.get(i),
        )

    display.surface.blit(
        pygame.transform.flip(display.surface, False, True),
        dest=(0, 0),
    )

    for hook in display.hooks:
        hook()
    pygame.display.flip()
