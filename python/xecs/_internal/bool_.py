from typing import cast

from xecs.xecs import Bool


def bool_(*, default: bool) -> Bool:
    """
    Provide additional data about a component field.

    Parameters:
        default: The default value for the field.
    """
    return cast(Bool, default)
