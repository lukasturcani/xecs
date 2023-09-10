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

Entities which represent a person will have this component. In our
example, we also want to keep track of how much health each person has.
If you're not familiar with ECS you may be tempted to add a field to
the ``Person`` component, such as ``health: xx.Int``. However,
other entities may have health too. By splitting up health into
a separate component, we can eventually write systems which operate
on any entity which has health. For example, a damage system will not
care if the entity receiving damage is a person or a cow. In any case,
here is our new component:

.. testcode:: first-component

  class Health(xx.Component):
      value: xx.Int

Next, we add people into our :class:`~xecs.World` using a
"startup system". Startup systems are run once, before any other
system. We use :class:`~xecs.Commands` to spawn entities into our
:class:`~xecs.World`:

.. testcode:: first-component

  def spawn_people(
      commands: xx.Commands,
  ) -> None:
      commands.spawn((Person, Health), 5)


To show we've spawned our people, and their health, we can write a new system
which acts on all entities with a ``Person`` and ``Health`` component:

.. testcode:: first-component

  def report_person_health(
      query: xx.Query[tuple[Person, Health]],
  ) -> None:
      (person, health) = query.result()
      print(person)
      print(health)

The parameters of our system function determine what data
our system runs on. In this case we are getting all
entities with a ``Person`` and ``Health`` component. The
``person`` and ``health`` variables are actually arrays
of all ``Person`` and ``Health`` components, which belong
to entities containing both.

Finally, let's write our ``main`` function again and
register our new systems:

.. testcode:: first-component

  def main() -> None:
      app = xx.RealTimeApp()
      app.add_startup_system(spawn_people)
      app.add_system(report_person_health)
      app.add_pool(Person.create_pool(5))
      app.add_pool(Health.create_pool(5))
      app.update()

Notice we also called :meth:`~xecs.RealTimeApp.add_pool`. In :mod:`xecs` we
reserve memory ahead of time for our components. This means that as our app runs,
we can avoid unnecessary re-allocations.

.. testcode:: first-component
  :hide:

  main()

The output of our program will be as follows:

.. testoutput:: first-component

  <Person()>
  <Health(
      value=<xecs.Int32 [0, 0, 0, 0, 0]>,
  )>

Initializing Components
.......................

In the previous section we spawned a bunch of health components:

.. testcode:: first-component

  def spawn_people(
      commands: xx.Commands,
  ) -> None:
      commands.spawn((Person, Health), 5)

We also saw that when we printed out ``Health`` component, the values
were set to 0. Let's say our game requires full health to be a value of
``100``, we can edit our function so that newly spawned components are
set to this value:

.. testcode:: first-component

  def spawn_people(
      commands: xx.Commands,
      world: xx.World,
  ) -> None:
      personi, healthi = commands.spawn((Person, Health), 5)
      health = world.get_view(Health, healthi)
      health.value.fill(100)


There is a lot going on here so let's take it step by step. First,
we added ``world: xx.World`` to our parameter list, so that
our system has access to the our simulated :class:`~xecs.World`. The
:class:`~xecs.World` can be used by systems to access entities, resources
and even other systems. In our system we will use the :class:`~xecs.World`
to access the newly spawned ``Health`` components, so that we can set their
value to ``100``.

We also created the ``personi`` and ``healthi`` variables from the return
value of :meth:`~xecs.Commands.spawn`. Recall that our components are held
in a pool we created in our ``main()`` function. The
:meth:`~xecs.Commands.spawn` command returns the indices of the components
we just spawned. We can retrieve the actual components by using
:meth:`~xecs.World.get_view`.

The ``health`` variable has type ``Health`` and is an array of all
newly spawned health components. The ``value`` attribute is of type
:class:`~xecs.Int32` and holds all the health values. We call
:meth:`~xecs.Int32.fill` to set all the selected values to ``100``.

If we run our program again, our output will be:

.. testcode:: first-component
  :hide:

  main()

.. testoutput:: first-component

  <Person()>
  <Health(
      value=<xecs.Int32 [100, 100, 100, 100, 100]>,
  )>


Doing Math
..........

Getting access to our components in a system is step one, but more
often than not, we will want to perform some kind of numerical operation
on our data. Let's continue our example by adding a damage system. At
each step it will remove one health point from our entities:

.. testcode:: first-component

  def damage_system(
      query: xx.Query[tuple[Person, Health]],
  ) -> None:
      person, health = query.result()
      health.value -= 1

Recall that ``health`` has type ``Health`` and is actually an array
of all ``Health`` components on entities which also have a ``Person``
component. The ``value`` attribute is of type :class:`~xecs.Int32`.
It is an array holding all the integers representing the
health values. The primitive types in
:mod:`xecs` such as :class:`~xecs.Bool`, :class:`~xecs.Int32`
and :class:`~xecs.Float32` are arrays holding a value for each
entity in the current view. Numerical types such as :class:`~xecs.Int32`
and :class:`~xecs.Float32` provide element wise arithmetic operations, much
like `NumPy <https://numpy.org/>`_.

Our values can be updated in-place using operators such as ``+=``,
``-=``, ``*=`` and so on. The right hand side of the operator can be
a single number, a list of numbers or a NumPy array. When using
list or array of numbers the operation is performed element-wise.
Operators such as ``+``, ``-`` and ``*`` do not update our
components in-place, instead they return a NumPy array of the
results. If we want to place the results back into our
components we can use :meth:`~xecs.Int32.fill`. Finally,
if we want to use NumPy functions, we can convert our component
values into NumPy arrays with :meth:`~xecs.Int32.numpy`.

Filtering Components
....................

For some systems we want to filter out entities based on the values
of components. Take for example a healing system, which adds a health
point to any entity with less than 50 health:


.. testcode:: first-component

  def healing_system(
      query: xx.Query[tuple[Person, Health]],
  ) -> None:
      person, health = query.result()
      low_health = health[health.value < 50]
      low_health.value += 1

In this system, ``health.value < 50`` returns a boolean mask.
When the mask is used to index, as in ``health[...]``, a new
``Health`` component is returned, holding only entities where
the mask was ``True``.

.. testcode:: first-component
  :hide:

  def main() -> None:
      app = xx.RealTimeApp()
      app.add_startup_system(spawn_people)
      app.add_system(damage_system)
      app.add_system(healing_system)
      app.add_pool(Person.create_pool(5))
      app.add_pool(Health.create_pool(5))
      app.update()

  main()

Combining Entities
..................

As you can tell, :mod:`xecs` focuses a lot on element-wise operations.
In fact, this is the primary tool it uses for its performance. As a result,
if you find yourself using a for-loop inside an :mod:`xecs` system, chances are
something has gone wrong.

One common reason to reach for a for-loop is to go through all pairs of entities
because we expect them to have some kind of interaction. Let's write a new
app. In this app we will:

* Spawn some entities.
* Assign them some positions.
* Go through all pairs of entities.
* If two entities are "close", we will consider them to be neighbors and
  increase their neighbor count.

.. testcode:: neighbors

  import xecs as xx

  class Neighbors(xx.Component):
      num_neighbors: xx.Int

  def spawn_entities(
      commands: xx.Commands,
      world: xx.World,
  ) -> None:
      _, transformi = commands.spawn((Neighbors, xx.Transform2), 5)
      transform = world.get_view(xx.Transform2, transformi)
      transform.translation.x.fill([1, 2, 3, 4, 5])

  def count_neighbors(
      query: xx.Query[tuple[Neighbors, xx.Transform2]],
  ) -> None:
      (neighbors, transform1), (_, transform2) = query.product_2()
      x_distance = abs(transform1.translation.x - transform2.translation.x)
      neighbors[x_distance < 2].num_neighbors += 1

  def print_neighbors(query: xx.Query[Neighbors]) -> None:
      print(query.result())

  def main() -> None:
      app = xx.RealTimeApp()
      app.add_startup_system(spawn_entities)
      app.add_system(count_neighbors)
      app.add_system(print_neighbors)
      app.add_pool(Neighbors.create_pool(5))
      app.add_pool(xx.Transform2.create_pool(5))
      app.update()

  if __name__ == "__main__":
      main()

.. testcode:: neighbors
  :hide:

  def print_neighbors(query: xx.Query[Neighbors]) -> None:
      print(sorted(query.result().num_neighbors.numpy()))
  def main() -> None:
      app = xx.RealTimeApp()
      app.add_startup_system(spawn_entities)
      app.add_system(count_neighbors)
      app.add_system(print_neighbors)
      app.add_pool(Neighbors.create_pool(5))
      app.add_pool(xx.Transform2.create_pool(5))
      app.update()
  main()

.. testoutput:: neighbors
  :hide:

  [1, 1, 2, 2, 2]
