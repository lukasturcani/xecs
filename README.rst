xecs
====

:Documentation: https://xecs.readthedocs.io

``xecs`` is a Python library (written in Rust!) for a performant
entity component system (ECS). You can use it to write simulations, games
or any other high-performance piece of software.

If you are familiar with `Bevy <https://bevyengine.org/>`_ and
`NumPy <https://numpy.org/>`_ -- the API of ``xecs`` should be
familiar to you.

The goals of ``xecs`` are as follows:

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


Code Preview
------------

.. code-block:: python

  import xecs as xx

  class Velocity(xx.Component):
      value: xx.Vec2

  def update_positions(query: xx.Query[tuple[xx.Transform2, Velocity]]) -> None:
      (transform, velocity) = query.result()
      transform.translation += velocity.value

Installation
------------

.. code-block:: bash

  pip install xecs
