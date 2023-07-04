import ecstasy as ecs
import numpy as np


class Transform(ecs.Component):
    translation: ecs.Vec2
    rotation: ecs.Float


class Velocity(ecs.Component):
    inner: ecs.Vec2


class Separation(ecs.Component):
    displacement_sum: ecs.Vec2


class Alignment(ecs.Component):
    velocity_sum: ecs.Vec2
    num_neighbors: ecs.Float


class Cohesion(ecs.Component):
    translation_sum: ecs.Vec2
    num_neighbors: ecs.Float


class Params(ecs.Resource):
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


class Generator(ecs.Resource):
    value: np.random.Generator


def main() -> None:
    app = ecs.App()
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
    time_step = ecs.Duration.from_millis(16)
    app.add_system(calculate_separation, time_step)
    app.add_system(calculate_alignment, time_step)
    app.add_system(calculate_cohesion, time_step)
    app.add_system(update_boid_velocity, time_step)
    app.add_system(move_boids, time_step)
    app.add_component_pool(Transform.create_pool(num_boids))
    app.add_component_pool(Velocity.create_pool(num_boids))
    app.add_component_pool(Separation.create_pool(num_boids))
    app.add_component_pool(Alignment.create_pool(num_boids))
    app.add_component_pool(Cohesion.create_pool(num_boids))
    app.run()


def spawn_boids(
    params: Params,
    generator: Generator,
    commands: ecs.Commands,
) -> None:
    transforms = commands.spawn(Transform, params.num_boids)
    generator.value.random(out=transforms.translation.matrix)
    # TODO: should i remove the need for .matrix?
    transforms.translation.matrix *= params.max_translation

    velocities = commands.spawn(Velocity, params.num_boids)
    generator.value.random(out=velocities.inner.matrix),
    velocities.inner.matrix *= params.max_speed

    commands.spawn(Separation, params.num_boids)
    commands.spawn(Alignment, params.num_boids)
    commands.spawn(Cohesion, params.num_boids)


def move_boids(
    query: ecs.Query[tuple[Transform, Velocity]],
) -> None:
    transforms, velocities = query.result()
    transforms.translation += velocities.inner * time_step.period.as_secs()
    transforms.rotation = ecs.Quat.from_rotation_z(
        ecs.Vec2(0.0, 1.0).angle_between(velocities.inner)
    )


def calculate_separation(
    params: Params,
    query: ecs.Query[tuple[Transform, Separation]],
) -> None:
    (_, separations) = query.result()
    # TODO: Should i remove the need for [:]?
    separations.displacement_sum[:] = 0.0

    (transforms1, separations1), (
        transforms2,
        separations2,
    ) = query.combinations(2)
    displacement = transforms1.translation - transforms2.translation
    distance = displacement.length()
    needs_separation = distance < params.separation_radius

    displacement = displacement[needs_separation]
    separations1 = separations1[needs_separation]
    separations2 = separations2[needs_separation]

    separations1.displacement_sum += displacement
    separations2.displacement_sum -= displacement


def calculate_alignment(
    params: Params,
    query: ecs.Query[tuple[Transform, Velocity, Alignment]],
) -> None:
    (_, _, aligments) = query.result()
    alignments.velocity_sum[:] = 0.0
    alignments.num_neighbors[:] = 0

    boids1, boids2 = query.combinations(2)

    displacement = boids1[0].translation - boids2[0].translation
    distance = displacement.length()
    needs_alignment = (
        distance > params.separation_radius & distance < params.visible_radius
    )

    (_, velocities1, alignments1) = boids1[needs_alignment]
    (_, velocities2, alignments2) = boids2[needs_alignment]

    alignments1.velocity_sum += velocities2.inner
    alignments1.num_neighbors += 1
    alignments2.velocity_sum += velocities1.inner
    alignments2.num_neighbors += 1


def calculate_cohesion(
    params: Params,
    query: ecs.Query[tuple[Transform, Cohesion]],
) -> None:
    (_, cohesions) = query.result()
    cohesions.translation_sum[:] = 0.0
    cohesions.num_neighbors[:] = 0

    (transforms1, cohesions1), (transforms2, cohesions2) = query.combinations(
        2
    )
    displacement = transforms1.translation - transforms2.translation
    distance = displacement.length()
    needs_cohesion = (
        distance > params.separation_radius & distance < params.visible_radius
    )

    transforms1 = transforms1[needs_cohesion]
    cohesions1 = cohesions1[needs_cohesion]
    transforms2 = transforms2[needs_cohesion]
    cohesions2 = cohesions2[needs_cohesion]

    cohesions1.translation_sum += transforms2.translation
    cohesions1.num_neighbors += 1
    cohesions2.translation_sum += transforms1.translation
    cohesions2.num_neighbors += 1


def update_boid_velocity(
    params: Params,
    query: ecs.Query[
        tuple[Transform, Separation, Alignment, Cohesion, Velocity]
    ],
) -> None:
    (
        transforms,
        separations,
        alignments,
        cohesions,
        velocities,
    ) = query.result()

    alignment_update = alignments.num_neighbors > 0
    alignment_velocities = velocities[alignment_update]
    alignments = alignments[alignment_update]
    alignments.velocity_sum /= alignments.num_neighbors
    alignments.velocity_sum -= alignment_velocities.inner
    alignments.velocity_sum *= params.alignment_coefficient
    alignment_velocities.inner += alignments.velocity_sum

    cohesion_update = cohesions.num_neighbors > 0
    cohesions = cohesions[cohesion_update]
    cohesions.translation_sum /= cohesions.num_neighbors
    cohesions.translation_sum -= transforms[cohesion_update].translation
    cohesions.translation_sum *= params.cohesion_coefficient
    velocities[cohesion_update].inner += cohesions.translation_sum

    left_bounds = transforms.translation.x < params.left_margin
    velocities[left_bounds].inner.x += params.box_bound_coefficient

    right_bounds = transforms.translation.x > params.right_margin
    velocities[right_bounds].inner.x -= params.box_bound_coefficient

    bottom_bounds = transforms.translation.y < params.bottom_margin
    velocities[bottom_bounds].inner.y += params.box_bound_coefficient

    top_bounds = transforms.translation.y > params.top_margin
    velocities[top_bounds].inner.y += params.box_bound_coefficient

    separations.displacement_sum *= params.separation_coefficient
    velocities.inner += separations.displacement_sum
