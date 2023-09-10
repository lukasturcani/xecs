Plugins
=======

:mod:`xecs` aims to divide its functionality into modular components called
plugins. Plugins can be used to provide features such as rendering, physics,
UIs or really anything else that can be re-used across apps. In fact, every
app can be converted into a plugin. Plugins can also be easily released as
separate Python packages making them easy to distribute and re-use.

Creating Your First Plugin
--------------------------

.. testcode:: plugin

  import xecs as xx

  def my_system() -> None:
      print("Hello plugin!")

  class FirstPlugin(xx.RealTimeAppPlugin):
      def build(self, app: xx.RealTimeApp) -> None:
          app.add_system(my_system)

  def main() -> None:
      app = xx.RealTimeApp()
      app.add_plugin(FirstPlugin())
      app.update()

  if __name__ == "__main__":
      main()

.. testcode:: plugin
  :hide:

  main()

.. testoutput:: plugin
  :hide:

  Hello plugin!
