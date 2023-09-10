.. xecs documentation master file, created by
   sphinx-quickstart on Fri Sep  8 16:06:17 2023.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Welcome to xecs!
================

GitHub: https://github.com/lukasturcani/xecs

.. toctree::
  :maxdepth: 2
  :caption: Contents:
  :hidden:

  Getting Started <getting_started>
  Examples <examples>
  Developer's Guide <developers_guide>

.. toctree::
  :maxdepth: 2
  :caption: Core API:
  :hidden:

  Bool <_autosummary/xecs.Bool>
  Int32 <_autosummary/xecs.Int32>
  Float32 <_autosummary/xecs.Float32>
  PyField <_autosummary/xecs.PyField>
  Vec2 <_autosummary/xecs.Vec2>
  Transform2 <_autosummary/xecs.Transform2>
  Query <_autosummary/xecs.Query>
  Commands <_autosummary/xecs.Commands>
  World <_autosummary/xecs.World>
  RealTimeApp <_autosummary/xecs.RealTimeApp>
  SimulationApp <_autosummary/xecs.SimulationApp>
  Modules <modules>

:mod:`xecs` is a Python library (written in Rust!) for a performant
entity component system (ECS). You can use it to write simulations, games
or any other high-performance piece of software.

If you are familiar with `Bevy <https://bevyengine.org/>`_ and
`NumPy <https://numpy.org/>`_ -- the API of :mod:`xecs` should be
familiar to you. Here is a little taste:

.. testcode:: taste

  import xecs as xx

  class Velocity(xx.Component):
      value: xx.Vec2

  def update_positions(query: xx.Query[tuple[xx.Transform2, Velocity]]) -> None:
      (transform, velocity) = query.result()
      transform.translation += velocity.value

.. testcode:: taste
  :hide:

  app = xx.RealTimeApp()
  app.add_pool(xx.Transform2.create_pool(0))
  app.add_pool(Velocity.create_pool(0))
  app.add_system(update_positions)
  app.update()

The goals of :mod:`xecs` are as follows:

* **Fast**: Operations are executed in parallel as much as possible
  and the library is written in Rust to be cache friendly and performant.
* **Simple**: Data is defined with a dataclass-like syntax and systems are regular
  Python functions.
* **Typed**: Types form an integral part of the API, making code clean but
  also easily verified with type checkers.
* **NumPy-friendly**: Our data types can be used seamlessly with NumPy.
* **Python-friendly**: User code is regular Python code, allowing
  full integration with the Python ecosystem. We avoid things like Numba
  which cause pain during debugging and limit use of pure Python libraries.


Installation
------------

.. code-block:: bash

  pip install xecs


Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
