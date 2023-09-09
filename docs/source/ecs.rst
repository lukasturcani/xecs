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
    :hide:

    def spawn_people(commands: xx.Commands, world: xx.World) -> None:
        (personi,) = commands.spawn((Person,), 2)
        person = world.get_view(Person, personi)

    app = xx.SimulationApp()
    app.add_startup_system(spawn_people)
    app.add_system(print_person_system)
    app.add_pool(Person.create_pool(2))
    app.update(xx.Duration.from_millis(1))

  .. testoutput:: ecs
    :hide:

    <Person(
      	stamina=<xecs.Int32 [0, 0]>,
        is_damaged=<xecs.Bool [false, false]>,
        height=<xecs.Float32 [0.0, 0.0]>,
    )>

Your First System
-----------------

A simple system does not have to take any parameters:

.. testcode:: first-system

  def hello_world() -> None:
      print("Hello world!")

We can create a working program by combining the above snippet with our basic
boilerplate:

.. testcode:: first-system

  import xecs as xx

  def hello_world() -> None:
      print("Hello world!")

  def main() -> None:
      app = xx.RealTimeApp()
      app.add_system(hello_world)
      app.update()

  if __name__ == "__name__":
      main()

If you copied the above code into a file called ``xecs_hello_world.py``,
you can run your code with:

.. code-block:: bash

  python xecs_hello_world.py

The program will print:

.. testcode:: first-system
  :hide:

  main()

.. testoutput:: first-system

  Hello world!


Your First Components
---------------------

.. testsetup:: first-component

  import xecs as xx

In ECS we model game objects, such as people, as entities.
An entity is essentially just a bundle of components. To start, we
create a ``Person`` component:

.. testcode:: first-component

  class Person(xx.Component):
      pass

Entities which represent a person will have this component.

.. testcode:: first-component

  class Health(xx.Component):
      value: xx.Int


.. testcode:: first-component

  def spawn_people(
      commands: xx.Commands,
  ) -> None:
      commands.spawn((Person, Health), 5)


.. testcode:: first-component

  def report_person_health(
      query: xx.Query[tuple[Person, Health]],
  ) -> None:
      (person, health) = query.result()
      print(person)
      print(health)

.. testcode:: first-component

  def main() -> None:
      app = xx.RealTimeApp()
      app.add_startup_system(spawn_people)
      app.add_system(report_person_health)
      app.add_pool(Person.create_pool(5))
      app.add_pool(Health.create_pool(5))
      app.update()

.. testcode:: first-component
  :hide:

  main()
