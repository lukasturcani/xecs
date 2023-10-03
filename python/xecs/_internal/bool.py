import builtins
from typing import cast

from xecs.xecs import Bool


def bool(*, default: builtins.bool) -> Bool:
    return cast(Bool, default)
