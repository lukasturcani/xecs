ECS
===

App logic in :mod:`xecs` is written using an entity component system (ECS).
ECS is a design pattern used in software development,
particularly in game development, to manage the behavior and data of entities
within a system efficiently. It's a way to structure code and organize data
in a flexible and scalable manner. ECS separates the concerns of an
entity's identity, its data, and its behavior into distinct components, making
it highly modular and conducive to parallel processing.

Here's a breakdown of the core concepts in an ECS:

* **Entity**: An entity is a general-purpose object or game entity. It's essentially an identifier that
  can represent anything in your system. Entities don't contain any data or logic themselves;
  they're just used to group components together.
* **Component**: A component is a self-contained unit of data that represents a specific aspect or
  attribute of an entity. For example, you might have components for position, velocity,
  health, rendering, and more. Each component typically contains only data and no behavior. In
  :mod:`xecs` we define components using a dataclass-like syntax:

  .. testcode:: ecs

    import xecs as xx

    class Person(xx.Component):
        stamina: xx.Int
        is_damaged: xx.Bool
        height: xx.Float

* **System**: Systems are responsible for processing entities that have specific combinations of
  components and applying behaviors or functionality to them. Systems operate on entities that
  match a set of component requirements, making it easy to implement different behaviors
  independently and in parallel. For example, you might have a rendering system, a physics
  system, and an input system. In :mod:`xecs` systems are normal Python functions:

  .. testcode:: ecs

    def print_person_system(query: xx.Query[Person]) -> None:
        person = query.result()
        print(person)


  .. testcode:: ecs

    def spawn_people(commands: xx.Commands, world: xx.World) -> None:
        (personi,) = commands.spawn((Person,), 2)
        person = world.get_view(Person, personi)

    app = xx.SimulationApp()
    app.add_startup_system(spawn_people)
    app.add_system(print_person_system)
    app.add_pool(Person.create_pool(2))
    app.update(xx.Duration.from_millis(1))



Your First System
-----------------


Your First Components
---------------------
