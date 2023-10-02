Events
======

Events are one of the most flexible ways to send data
between systems and allow systems to communicate with each other.
Any system can publish an event and any system can subscribe to an event.
We send events using :class:`.EventWriter` and receive events using
:class:`.EventReader`. Any arbitrary Python type can be used to create
events:


.. testsetup:: events

  import xecs as xx

.. testcode:: events

  from dataclasses import dataclass

  @dataclass
  class MyEvent:
      message: str

  def writer_system(
      writer: xx.EventWriter[MyEvent],
  ) -> None:
      writer.send(MyEvent("Boom!"))

  def reader_system(
      reader: xx.EventReader[MyEvent],
  ) -> None:
      for event in reader.events:
          print(event.message)


.. testcode:: events
  :hide:

  def main() -> None:
      app = xx.RealTimeApp()
      app.add_system(writer_system)
      app.add_system(reader_system)
      app.update()

  main()

.. testoutput:: events

  Boom!

Every system will have access to all the events that have been triggered
since the last time that system ran.
