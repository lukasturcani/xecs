import numpy as np
import pygame
import xecs as xx


class PyGamePlugin(xx.Plugin):
    def build(self, app: xx.App) -> None:
        pygame.init()
        app.add_resource(Display(pygame.display.set_mode((640, 640)), "black"))
        app.add_system(draw)


class Display(xx.Resource):
    surface: pygame.Surface
    color: str


class Polygon(xx.Component):
    vertices: xx.PyField[list[tuple[float, float]]]
    color: xx.PyField[str]


def draw(
    display: Display,
    polygon_query: xx.Query[tuple[xx.Transform2, Polygon]],
) -> None:
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
            np.multiply(2, [x, y]) + display_origin + (r @ vertices).T
        )
        pygame.draw.polygon(
            display.surface, polygon.color.get(i), boid_polygon.tolist()
        )
    display.surface.blit(
        pygame.transform.flip(display.surface, False, True),
        dest=(0, 0),
    )

    pygame.display.flip()
