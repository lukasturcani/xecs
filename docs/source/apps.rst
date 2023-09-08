Apps
====

:mod:`xecs` programs are called apps. There are two kinds of apps:

* :class:`~xecs.RealTimeApp`: For programs, like games, which run in
  real time. Normally, these are used when interaction with humans
  is required.
* :class:`~xecs.SimulationApp`: For programs which run as quickly as possible.
  These do not interact with humans, we simply want to run our simulations and
  get our output without any delay.

:class:`~xecs.RealTimeApp` example:

.. testcode:: real-time-app

  import xecs as xx

  def main() -> None:
      app = xx.RealTimeApp()
      app.run()

  if __name__ == "__main__":
      main()

:class:`~xecs.SimulationApp` example:

.. testcode:: simulation-app

  import xecs as xx

  def main() -> None:
      app = xx.SimulationApp()
      app.run(500)

  if __name__ == "__main__":
      main()


These examples do not do any useful work, but they are complete, working
apps. They form the basic boilerplate on top of which we will build useful
programs.
