import numpy as np
import xecs as xx


class Transform(xx.Component):
    translation: xx.Vec2
    rotation: xx.Float

    def fill_random(
        self,
        generator: np.random.Generator,
        scale: float,
    ) -> None:
        self.translation.fill(
            generator.random((2, len(self)), dtype=np.float32) * scale
        )
        self.rotation.fill(
            generator.random(len(self), dtype=np.float32) * 2 * np.pi
        )


class Velocity(xx.Component):
    value: xx.Vec2

    def fill_random(
        self,
        generator: np.random.Generator,
        scale: float,
    ) -> None:
        self.value.fill(
            generator.random((2, len(self)), dtype=np.float32) * scale
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
    max_translation: float
    max_speed: float
    separation_radius: float
    visible_radius: float
    separation_coefficient: float
    alignment_coefficient: float
    cohesion_coefficient: float
    box_bound_coefficient: float
    left_margin: float
    right_margin: float
    bottom_margin: float
    top_margin: float


class Generator(xx.Resource):
    value: np.random.Generator


def main() -> None:
    app = xx.App()
    num_boids = 100
    app.add_resource(
        Params(
            num_boids=num_boids,
            max_translation=150.0,
            max_speed=60.0,
            separation_radius=3.0,
            visible_radius=6.0,
            separation_coefficient=0.1,
            alignment_coefficient=0.005,
            cohesion_coefficient=0.0005,
            box_bound_coefficient=0.2,
            left_margin=0.0,
            right_margin=150.0,
            bottom_margin=0.0,
            top_margin=150.0,
        )
    )
    app.add_resource(Generator(np.random.default_rng(55)))
    app.add_startup_system(spawn_boids)
    time_step = xx.Duration.from_millis(16)
    app.add_system(calculate_separation, time_step)
    app.add_system(calculate_alignment, time_step)
    app.add_system(calculate_cohesion, time_step)
    app.add_system(update_boid_velocity, time_step)
    app.add_system(move_boids, time_step)
    app.add_pool(Transform.create_pool(num_boids))
    app.add_pool(Velocity.create_pool(num_boids))
    app.add_pool(Separation.create_pool(num_boids))
    app.add_pool(Alignment.create_pool(num_boids))
    app.add_pool(Cohesion.create_pool(num_boids))
    app.run()


def spawn_boids(
    params: Params,
    generator: Generator,
    world: xx.World,
    commands: xx.Commands,
) -> None:
    transformi, velocityi, *_ = commands.spawn(
        components=(Transform, Velocity, Separation, Alignment, Cohesion),
        num=params.num_boids,
    )
    world.get_view(Transform, transformi).fill_random(
        generator.value, params.max_translation
    )
    world.get_view(Velocity, velocityi).fill_random(
        generator.value, params.max_speed
    )


def move_boids(
    query: xx.Query[tuple[Transform, Velocity]],
) -> None:
    transform, velocity = query.result()
    transform.translation += velocity.value * 16 / 1e3
    xx.Vec2.from_xy(0.0, 1.0, len(transform)).angle_between(
        velocity.value, out=transform.rotation
    )


def calculate_separation(
    params: Params,
    query: xx.Query[tuple[Transform, Separation]],
) -> None:
    (_, separations) = query.result()
    separations.displacement_sum.fill(0)

    boid1, boid2 = query.combinations_2()
    transform1, separation1 = boid1
    transform2, separation2 = boid2
    displacement = transform1.translation - transform2.translation
    distance = np.linalg.norm(displacement, axis=0)
    needs_separation = distance < params.separation_radius

    displacement = displacement[needs_separation]
    separation1 = separation1[needs_separation]
    separation2 = separation2[needs_separation]

    separation1.displacement_sum += displacement
    separation2.displacement_sum -= displacement


def calculate_alignment(
    params: Params,
    query: xx.Query[tuple[Transform, Velocity, Alignment]],
) -> None:
    (_, _, alignment) = query.result()
    alignment.velocity_sum.fill(0)
    alignment.num_neighbors.fill(0)

    boid1, boid2 = query.combinations_2()
    transform1, *_ = boid1
    transform2, *_ = boid2

    displacement = transform1.translation - transform2.translation
    distance = np.linalg.norm(displacement, axis=0)
    needs_alignment = distance > params.separation_radius
    needs_alignment &= distance < params.visible_radius

    (_, velocity1, alignment1) = boid1[needs_alignment]
    (_, velocity2, alignment2) = boid2[needs_alignment]

    alignment1.velocity_sum += velocity2.value
    alignment1.num_neighbors += 1
    alignment2.velocity_sum += velocity1.value
    alignment2.num_neighbors += 1


def calculate_cohesion(
    params: Params,
    query: xx.Query[tuple[Transform, Cohesion]],
) -> None:
    (_, cohesion) = query.result()
    cohesion.translation_sum.fill(0)
    cohesion.num_neighbors.fill(0)

    boid1, boid2 = query.combinations_2()
    transform1, cohesion1 = boid1
    transform2, cohesion2 = boid2

    displacement = transform1.translation - transform2.translation
    distance = np.linalg.norm(displacement, axis=0)
    needs_cohesion = (
        distance > params.separation_radius & distance < params.visible_radius
    )

    transform1 = transform1[needs_cohesion]
    cohesion1 = cohesion1[needs_cohesion]
    transform2 = transform2[needs_cohesion]
    cohesion2 = cohesion2[needs_cohesion]

    cohesion1.translation_sum += transform2.translation
    cohesion1.num_neighbors += 1
    cohesion2.translation_sum += transform1.translation
    cohesion2.num_neighbors += 1


def update_boid_velocity(
    params: Params,
    query: xx.Query[
        tuple[Transform, Separation, Alignment, Cohesion, Velocity]
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

    left_bounds = transform.translation.x < params.left_margin
    velocity[left_bounds].value.x += params.box_bound_coefficient

    right_bounds = transform.translation.x > params.right_margin
    velocity[right_bounds].value.x -= params.box_bound_coefficient

    bottom_bounds = transform.translation.y < params.bottom_margin
    velocity[bottom_bounds].value.y += params.box_bound_coefficient

    top_bounds = transform.translation.y > params.top_margin
    velocity[top_bounds].value.y += params.box_bound_coefficient

    separation.displacement_sum *= params.separation_coefficient
    velocity.value += separation.displacement_sum
