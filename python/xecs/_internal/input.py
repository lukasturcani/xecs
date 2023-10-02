from dataclasses import dataclass

from xecs._internal.resource import Resource


@dataclass(frozen=True, slots=True)
class MouseButton:
    """
    Represents a mouse button.

    Parameters:
        number: The number of the button.
    """

    number: int
    """
    The number of the button.
    """

    @staticmethod
    def left() -> "MouseButton":
        """
        The left button.
        """
        return MouseButton(1)

    @staticmethod
    def middle() -> "MouseButton":
        """
        The middle button.
        """
        return MouseButton(2)

    @staticmethod
    def right() -> "MouseButton":
        """
        The right button.
        """
        return MouseButton(3)

    @staticmethod
    def xinput1() -> "MouseButton":
        """
        The first extra button.
        """
        return MouseButton(4)

    @staticmethod
    def xinput2() -> "MouseButton":
        """
        The second extra button.
        """
        return MouseButton(5)


class Mouse(Resource):
    """
    Represents the current state of the mouse.
    """

    pressed: set[MouseButton]
    """
    The currently pressed buttons.
    """
