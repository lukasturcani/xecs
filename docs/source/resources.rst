Resources
=========

So far we have covered components, which represent data
attached to a single entity. However, our apps will also
require shared, globally unique data. We do this with
resources:

.. testsetup:: resources

  import xecs as xx
  import numpy as np

.. testcode:: resources

  class Params(xx.Resource):
      max_velocity: float
      generator: np.random.Generator

Unlike components, resources can hold any type, including
arbitrary Python types, such as the ones you make yourself.
When we want to access a resource in a system, we simply
add it as a type hint:

.. testcode:: resources

  def my_system(params: Params) -> None:
      print(params.generator.random())

Finally, we need to initialize resources and add them
to our app:

.. testcode:: resources

  def main() -> None:
      app = xx.RealTimeApp()
      app.add_resource(
          Params(
              max_velocity=100.0,
              generator=np.random.default_rng(72),
          ),
      )
      app.add_system(my_system)
      app.update()

.. testcode:: resources
  :hide:

  main()

.. testoutput:: resources
  :hide:

  0.8378367344738
