from dataclasses import dataclass

import numpy as np
import pygame
import pygame_widgets
import xecs as xx
from pygame_widgets import slider
from pygame_widgets.textbox import TextBox
from xecs_pygame import Display, Polygon, PyGamePlugin


class Velocity(xx.Component):
    value: xx.Vec2

    def fill_random(
        self,
        generator: np.random.Generator,
        scale: float,
    ) -> None:
        self.value.fill(
            (generator.random((2, len(self)), dtype=np.float32) - 0.5) * scale
        )


class Separation(xx.Component):
    displacement_sum: xx.Vec2


class Alignment(xx.Component):
    velocity_sum: xx.Vec2
    num_neighbors: xx.Float


class Cohesion(xx.Component):
    translation_sum: xx.Vec2
    num_neighbors: xx.Float


class Params(xx.Resource):
    num_boids: int
    min_speed: float
    max_speed: float
    separation_radius: float
    visible_radius: float
    separation_coefficient: float
    alignment_coefficient: float
    cohesion_coefficient: float
    box_bound_coefficient: float
    box_size: float


class Generator(xx.Resource):
    value: np.random.Generator


@dataclass(slots=True, frozen=True)
class Slider:
    slider: slider.Slider
    textbox: TextBox

    def __post_init__(self) -> None:
        self.textbox.disable()

    def update(self) -> None:
        self.textbox.setText(self.slider.getValue())


class Ui(xx.Resource):
    min_speed_slider: Slider
    max_speed_slider: Slider
    separation_radius_slider: Slider
    visible_radius_slider: Slider
    separation_coefficient_slider: Slider
    alignment_coefficient_slider: Slider
    cohesion_coefficient_slider: Slider
    box_bound_coefficient_slider: Slider
    box_size_slider: Slider

    def update_sliders(self) -> None:
        self.min_speed_slider.update()
        self.max_speed_slider.update()
        self.separation_radius_slider.update()
        self.visible_radius_slider.update()
        self.separation_coefficient_slider.update()
        self.alignment_coefficient_slider.update()
        self.cohesion_coefficient_slider.update()
        self.box_bound_coefficient_slider.update()
        self.box_size_slider.update()


def main() -> None:
    app = xx.App()
    num_boids = 100
    app.add_plugin(PyGamePlugin())
    app.add_resource(
        Params(
            num_boids=num_boids,
            min_speed=15.0,
            max_speed=60.0,
            separation_radius=6.0,
            visible_radius=6.0,
            separation_coefficient=0.1,
            alignment_coefficient=0.005,
            cohesion_coefficient=0.0005,
            box_bound_coefficient=1.0,
            box_size=250.0,
        )
    )
    app.add_resource(Generator(np.random.default_rng(55)))
    app.add_startup_system(init_ui)
    app.add_startup_system(spawn_boids)
    time_step = xx.Duration.from_millis(16)
    app.add_system(calculate_separation, time_step)
    app.add_system(calculate_alignment, time_step)
    app.add_system(calculate_cohesion, time_step)
    app.add_system(update_boid_velocity, time_step)
    app.add_system(move_boids, time_step)
    app.add_system(handle_ui)
    app.add_pool(xx.Transform2.create_pool(num_boids))
    app.add_pool(Velocity.create_pool(num_boids))
    app.add_pool(Separation.create_pool(num_boids))
    app.add_pool(Alignment.create_pool(num_boids))
    app.add_pool(Cohesion.create_pool(num_boids))
    app.add_pool(Polygon.create_pool(num_boids))
    app.run()


def spawn_boids(
    params: Params,
    generator: Generator,
    world: xx.World,
    commands: xx.Commands,
) -> None:
    transformi, velocityi, polygoni, *_ = commands.spawn(
        components=(
            xx.Transform2,
            Velocity,
            Polygon,
            Separation,
            Alignment,
            Cohesion,
        ),
        num=params.num_boids,
    )
    world.get_view(xx.Transform2, transformi).fill_random(
        generator.value, params.box_size
    )
    world.get_view(Velocity, velocityi).fill_random(
        generator.value, params.max_speed
    )
    polygon = world.get_view(Polygon, polygoni)
    polygon.vertices.fill([(-4, -5), (0, 10), (4, -5)])
    polygon.color.fill("green")


def move_boids(
    query: xx.Query[tuple[xx.Transform2, Velocity]],
) -> None:
    transform, velocity = query.result()
    transform.translation += velocity.value * 16 / 1e3
    transform.rotation.fill(-velocity.value.angle_between_xy(0.0, 1.0))


def calculate_separation(
    params: Params,
    query: xx.Query[tuple[xx.Transform2, Separation]],
) -> None:
    (_, separation) = query.result()
    separation.displacement_sum.fill(0)

    boid1, boid2 = query.product_2()
    transform1, separation = boid1
    transform2, _ = boid2
    displacement = transform1.translation - transform2.translation
    distance = np.linalg.norm(displacement, axis=0)
    needs_separation = distance < params.separation_radius

    displacement = displacement[:, needs_separation]
    separation = separation[needs_separation]
    separation.displacement_sum += displacement


def calculate_alignment(
    params: Params,
    query: xx.Query[tuple[xx.Transform2, Velocity, Alignment]],
) -> None:
    (_, _, alignment) = query.result()
    alignment.velocity_sum.fill(0)
    alignment.num_neighbors.fill(0)

    boid1, boid2 = query.product_2()
    transform1, velocity1, alignment = boid1
    transform2, velocity2, _ = boid2

    displacement = transform1.translation - transform2.translation
    distance = np.linalg.norm(displacement, axis=0)
    needs_alignment = distance > params.separation_radius
    needs_alignment &= distance < params.visible_radius

    velocity1 = velocity1[needs_alignment]
    alignment = alignment[needs_alignment]
    velocity2 = velocity2[needs_alignment]

    alignment.velocity_sum += velocity2.value
    alignment.num_neighbors += 1


def calculate_cohesion(
    params: Params,
    query: xx.Query[tuple[xx.Transform2, Cohesion]],
) -> None:
    (_, cohesion) = query.result()
    cohesion.translation_sum.fill(0)
    cohesion.num_neighbors.fill(0)

    boid1, boid2 = query.product_2()
    transform1, cohesion = boid1
    transform2, _ = boid2

    displacement = transform1.translation - transform2.translation
    distance = np.linalg.norm(displacement, axis=0)
    needs_cohesion = distance > params.separation_radius
    needs_cohesion &= distance < params.visible_radius

    transform1 = transform1[needs_cohesion]
    cohesion = cohesion[needs_cohesion]
    transform2 = transform2[needs_cohesion]

    cohesion.translation_sum += transform2.translation
    cohesion.num_neighbors += 1


def update_boid_velocity(
    params: Params,
    query: xx.Query[
        tuple[xx.Transform2, Separation, Alignment, Cohesion, Velocity]
    ],
) -> None:
    (
        transform,
        separation,
        alignment,
        cohesion,
        velocity,
    ) = query.result()

    alignment_update = alignment.num_neighbors > 0
    alignment_velocities = velocity[alignment_update]
    alignment = alignment[alignment_update]
    alignment.velocity_sum /= alignment.num_neighbors
    alignment.velocity_sum -= alignment_velocities.value
    alignment.velocity_sum *= params.alignment_coefficient
    alignment_velocities.value += alignment.velocity_sum

    cohesion_update = cohesion.num_neighbors > 0
    cohesion = cohesion[cohesion_update]
    cohesion.translation_sum /= cohesion.num_neighbors
    cohesion.translation_sum -= transform[cohesion_update].translation
    cohesion.translation_sum *= params.cohesion_coefficient
    velocity[cohesion_update].value += cohesion.translation_sum

    left_bounds = transform.translation.x < -params.box_size / 2
    velocity[left_bounds].value.x += params.box_bound_coefficient

    right_bounds = transform.translation.x > params.box_size / 2
    velocity[right_bounds].value.x -= params.box_bound_coefficient

    bottom_bounds = transform.translation.y < -params.box_size / 2
    velocity[bottom_bounds].value.y += params.box_bound_coefficient

    top_bounds = transform.translation.y > params.box_size / 2
    velocity[top_bounds].value.y -= params.box_bound_coefficient

    separation.displacement_sum *= params.separation_coefficient
    velocity.value += separation.displacement_sum
    velocity.value.clamp_length(params.min_speed, params.max_speed)


def init_ui(world: xx.World, display: Display) -> None:
    display.color = "purple"
    world.add_resource(
        Ui(
            min_speed_slider=Slider(
                slider=slider.Slider(
                    display.surface, 100, 100, 800, 40, min=0, max=99, step=1
                ),
                textbox=TextBox(
                    display.surface, 475, 200, 50, 50, fontSize=30
                ),
            ),
            max_speed_slider=Slider(
                slider=slider.Slider(
                    display.surface, 100, 100, 800, 40, min=0, max=99, step=1
                ),
                textbox=TextBox(
                    display.surface, 475, 200, 50, 50, fontSize=30
                ),
            ),
            separation_radius_slider=Slider(
                slider=slider.Slider(
                    display.surface, 100, 100, 800, 40, min=0, max=99, step=1
                ),
                textbox=TextBox(
                    display.surface, 475, 200, 50, 50, fontSize=30
                ),
            ),
            visible_radius_slider=Slider(
                slider=slider.Slider(
                    display.surface, 100, 100, 800, 40, min=0, max=99, step=1
                ),
                textbox=TextBox(
                    display.surface, 475, 200, 50, 50, fontSize=30
                ),
            ),
            separation_coefficient_slider=Slider(
                slider=slider.Slider(
                    display.surface, 100, 100, 800, 40, min=0, max=99, step=1
                ),
                textbox=TextBox(
                    display.surface, 475, 200, 50, 50, fontSize=30
                ),
            ),
            alignment_coefficient_slider=Slider(
                slider=slider.Slider(
                    display.surface, 100, 100, 800, 40, min=0, max=99, step=1
                ),
                textbox=TextBox(
                    display.surface, 475, 200, 50, 50, fontSize=30
                ),
            ),
            cohesion_coefficient_slider=Slider(
                slider=slider.Slider(
                    display.surface, 100, 100, 800, 40, min=0, max=99, step=1
                ),
                textbox=TextBox(
                    display.surface, 475, 200, 50, 50, fontSize=30
                ),
            ),
            box_bound_coefficient_slider=Slider(
                slider=slider.Slider(
                    display.surface, 100, 100, 800, 40, min=0, max=99, step=1
                ),
                textbox=TextBox(
                    display.surface, 475, 200, 50, 50, fontSize=30
                ),
            ),
            box_size_slider=Slider(
                slider=slider.Slider(
                    display.surface, 100, 100, 800, 40, min=0, max=99, step=1
                ),
                textbox=TextBox(
                    display.surface, 475, 200, 50, 50, fontSize=30
                ),
            ),
        )
    )


def handle_ui(
    params: Params,
    ui: Ui,
) -> None:
    ui.update_sliders()
    pygame_widgets.update(pygame.event.get())


if __name__ == "__main__":
    main()
