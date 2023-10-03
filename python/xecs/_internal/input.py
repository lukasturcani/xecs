from dataclasses import dataclass
from enum import Enum, auto

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

    position: tuple[int, int]
    """
    The x and y position of the mouse.
    """


class KeyboardButton(Enum):
    """
    Represents as keyboard button.
    """

    KEY_1 = auto()
    """The ``1`` key over the main keyboard."""
    KEY_2 = auto()
    """The ``2`` key over the main keyboard."""
    KEY_3 = auto()
    """The ``3`` key over the main keyboard."""
    KEY_4 = auto()
    """The ``4`` key over the main keyboard."""
    KEY_5 = auto()
    """The ``5`` key over the main keyboard."""
    KEY_6 = auto()
    """The ``6`` key over the main keyboard."""
    KEY_7 = auto()
    """The ``7`` key over the main keyboard."""
    KEY_8 = auto()
    """The ``8`` key over the main keyboard."""
    KEY_9 = auto()
    """The ``9`` key over the main keyboard."""
    KEY_0 = auto()
    """The ``0`` key over the main keyboard."""
    A = auto()
    """The ``A`` key."""
    B = auto()
    """The ``B`` key."""
    C = auto()
    """The ``C`` key."""
    D = auto()
    """The ``D`` key."""
    E = auto()
    """The ``E`` key."""
    F = auto()
    """The ``F`` key."""
    G = auto()
    """The ``G`` key."""
    H = auto()
    """The ``H`` key."""
    I = auto()  # noqa: E741
    """The ``I`` key."""
    J = auto()
    """The ``J`` key."""
    K = auto()
    """The ``K`` key."""
    L = auto()
    """The ``L`` key."""
    M = auto()
    """The ``M`` key."""
    N = auto()
    """The ``N`` key."""
    O = auto()  # noqa: E741
    """The ``O`` key."""
    P = auto()
    """The ``P`` key."""
    Q = auto()
    """The ``Q`` key."""
    R = auto()
    """The ``R`` key."""
    S = auto()
    """The ``S`` key."""
    T = auto()
    """The ``T`` key."""
    U = auto()
    """The ``U`` key."""
    V = auto()
    """The ``V`` key."""
    W = auto()
    """The ``W`` key."""
    X = auto()
    """The ``X`` key."""
    Y = auto()
    """The ``Y`` key."""
    Z = auto()
    """The ``Z`` key."""
    ESCAPE = auto()
    """The ``Escape`` / ``ESC`` key."""
    F1 = auto()
    """The ``F1`` key."""
    F2 = auto()
    """The ``F2`` key."""
    F3 = auto()
    """The ``F3`` key."""
    F4 = auto()
    """The ``F4`` key."""
    F5 = auto()
    """The ``F5`` key."""
    F6 = auto()
    """The ``F6`` key."""
    F7 = auto()
    """The ``F7`` key."""
    F8 = auto()
    """The ``F8`` key."""
    F9 = auto()
    """The ``F9`` key."""
    F10 = auto()
    """The ``F10`` key."""
    F11 = auto()
    """The ``F11`` key."""
    F12 = auto()
    """The ``F12`` key."""
    F13 = auto()
    """The ``F13`` key."""
    F14 = auto()
    """The ``F14`` key."""
    F15 = auto()
    """The ``F15`` key."""
    F16 = auto()
    """The ``F16`` key."""
    F17 = auto()
    """The ``F17`` key."""
    F18 = auto()
    """The ``F18`` key."""
    F19 = auto()
    """The ``F19`` key."""
    F20 = auto()
    """The ``F20`` key."""
    F21 = auto()
    """The ``F21`` key."""
    F22 = auto()
    """The ``F22`` key."""
    F23 = auto()
    """The ``F23`` key."""
    F24 = auto()
    """The ``F24`` key."""
    SNAPSHOT = auto()
    """The ``Snapshot`` / ``Print Screen`` key."""
    SCROLL = auto()
    """The ``Scroll`` / ``Scroll Lock`` key."""
    PAUSE = auto()
    """The ``Pause`` / ``Break`` key."""
    INSERT = auto()
    """The ``Insert`` key."""
    HOME = auto()
    """The ``Home`` key."""
    DELETE = auto()
    """The ``Delete`` key."""
    END = auto()
    """The ``End`` key."""
    PAGE_DOWN = auto()
    """The ``Page Down`` key."""
    PAGE_UP = auto()
    """The ``Page Up`` key."""
    LEFT = auto()
    """The ``Left`` / ``Left Arrow`` key."""
    UP = auto()
    """The ``Up`` / ``Up Arrow`` key."""
    RIGHT = auto()
    """The ``Right`` / ``Right Arrow`` key."""
    DOWN = auto()
    """The ``Down`` / ``Down Arrow`` key."""
    BACK = auto()
    """The ``Backspace`` / ``Back`` key."""
    RETURN = auto()
    """The ``Return`` / ``Enter`` key."""
    SPACE = auto()
    """The ``Space`` / ``Spacebar`` key."""
    COMPOSE = auto()
    """The ``Compose`` key."""
    CARET = auto()
    """The ``Caret``  / ``^`` key."""
    NUM_LOCK = auto()
    """The ``Num Lock`` key."""
    NUMPAD_0 = auto()
    """The ``0`` key on the numpad."""
    NUMPAD_1 = auto()
    """The ``1`` key on the numpad."""
    NUMPAD_2 = auto()
    """The ``2`` key on the numpad."""
    NUMPAD_3 = auto()
    """The ``3`` key on the numpad."""
    NUMPAD_4 = auto()
    """The ``4`` key on the numpad."""
    NUMPAD_5 = auto()
    """The ``5`` key on the numpad."""
    NUMPAD_6 = auto()
    """The ``6`` key on the numpad."""
    NUMPAD_7 = auto()
    """The ``7`` key on the numpad."""
    NUMPAD_8 = auto()
    """The ``8`` key on the numpad."""
    NUMPAD_9 = auto()
    """The ``9`` key on the numpad."""
    ABNT_C1 = auto()
    """The ``Abnt C1`` key."""
    ABNT_C2 = auto()
    """The ``Abnt C2`` key."""
    NUMPAD_ADD = auto()
    """The ``+`` key on the numpad."""
    APOSTROPHE = auto()
    """The ``Apostrophe`` / ``'`` key."""
    APPS = auto()
    """The ``Apps`` key."""
    ASTERISK = auto()
    """The ``Asterisk`` / ``*`` key."""
    PLUS = auto()
    """The ``+`` key."""
    AT = auto()
    """The ``At`` / ``@`` key."""
    AX = auto()
    """The ``Ax`` key."""
    BACKSLASH = auto()
    """The ``Backslash`` / ``\\`` key."""
    CALCULATOR = auto()
    """The ``Calculator`` key."""
    CAPITAL = auto()
    """The ``Capital`` / ``Caps Lock`` key."""
    COLON = auto()
    """The ``Colon`` / ``:`` key."""
    COMMA = auto()
    """The ``Comma`` / ``,`` key."""
    CONVERT = auto()
    """The ``Convert`` key."""
    NUMPAD_DECIMAL = auto()
    """The ``.`` key on the numpad."""
    NUMPAD_DIVIDE = auto()
    """The ``/`` key on the numpad."""
    EQUALS = auto()
    """The ``=`` key."""
    GRAVE = auto()
    """The ``Grave`` / ``Backtick`` / ````` key."""
    KANA = auto()
    """The ``Kana`` key."""
    KANJI = auto()
    """The ``Kanji`` key."""
    ALT_LEFT = auto()
    """The ``Left Alt`` / ``Alt`` key."""
    BRACKET_LEFT = auto()
    """The ``Left Bracket`` / ``[`` key."""
    CONTROL_LEFT = auto()
    """The ``Left Control`` / ``Control`` key."""
    SHIFT_LEFT = auto()
    """The ``Left Shift`` / ``Shift`` key."""
    SUPER_LEFT = auto()
    """The ``Left Super`` / ``Super`` / ``Windows`` / ``Command`` key."""
    MAIL = auto()
    """The ``Mail`` key."""
    MEDIA_SELECT = auto()
    """The ``Media Select`` key."""
    MEDIA_STOP = auto()
    """The ``Media Stop`` key."""
    MINUS = auto()
    """The ``-`` key."""
    NUMPAD_MULTIPLY = auto()
    """The ``*`` key on the numpad."""
    MUTE = auto()
    """The ``Mute`` key."""
    MY_COMPUTER = auto()
    """The ``My Computer`` key."""
    NAVIGATE_FORWARD = auto()
    """The ``Navigate Forward`` key."""
    NAVIGATE_BACKWARD = auto()
    """The ``Navigate Backward`` key."""
    NEXT_TRACK = auto()
    """The ``Next Track`` key."""
    NO_CONVERT = auto()
    """The ``No Convert`` key."""
    NUMPAD_COMMA = auto()
    """The ``Comma`` / ``,`` key on the numpad."""
    NUMPAD_ENTER = auto()
    """The ``Enter`` key on the numpad."""
    NUMPAD_EQUALS = auto()
    """The ``=`` key on the numpad."""
    OEM_102 = auto()
    """The ``OEM 102`` key."""
    PERIOD = auto()
    """The ``Period`` / ``.`` key."""
    PLAY_PAUSE = auto()
    """The ``Play Pause`` key."""
    POWER = auto()
    """The ``Power`` key."""
    PREV_TRACK = auto()
    """The ``Previous Track`` key."""
    ALT_RIGHT = auto()
    """The ``Right Alt`` / ``Alt`` key."""
    BRACKET_RIGHT = auto()
    """The ``Right Bracket`` / ``]`` key."""
    CONTROL_RIGHT = auto()
    """The ``Right Control`` / ``Control`` key."""
    SHIFT_RIGHT = auto()
    """The ``Right Shift`` / ``Shift`` key."""
    SUPER_RIGHT = auto()
    """The ``Right Super`` / ``Super`` / ``Windows`` / ``Command`` key."""
    SEMICOLON = auto()
    """The ``Semicolon`` / ``;`` key."""
    SLASH = auto()
    """The ``Slash`` / ``/`` key."""
    SLEEP = auto()
    """The ``Sleep`` key."""
    STOP = auto()
    """The ``Stop`` key."""
    NUMPAD_SUBTRACT = auto()
    """The ``-`` key on the numpad."""
    SYSRQ = auto()
    """The ``Sysrq`` key."""
    TAB = auto()
    """The ``Tab`` key."""
    UNDERLINE = auto()
    """The ``Underline`` / ``_`` key."""
    UNLABELED = auto()
    """The ``Unlabeled`` key."""
    VOLUME_DOWN = auto()
    """The ``Volume Down`` key."""
    VOLUME_UP = auto()
    """The ``Volume Up`` key."""
    WAKE = auto()
    """The ``Wake`` key."""
    WEB_BACK = auto()
    """The ``Web Back`` key."""
    WEB_FAVORITES = auto()
    """The ``Web Favorites`` key."""
    WEB_FORWARD = auto()
    """The ``Web Forward`` key."""
    WEB_HOME = auto()
    """The ``Web Home`` key."""
    WEB_REFRESH = auto()
    """The ``Web Refresh`` key."""
    WEB_SEARCH = auto()
    """The ``Web Search`` key."""
    WEB_STOP = auto()
    """The ``Web Stop`` key."""
    YEN = auto()
    """The ``Yen`` key."""
    COPY = auto()
    """The ``Copy`` key."""
    PASTE = auto()
    """The ``Paste`` key."""
    CUT = auto()
    """The ``Cut`` key."""


class Keyboard(Resource):
    """
    Represents the current state of the keyboard.
    """

    pressed: set[KeyboardButton]
    """
    The currently pressed buttons.
    """
