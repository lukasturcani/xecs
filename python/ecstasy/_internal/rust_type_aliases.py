import typing

import numpy as np
import numpy.typing as npt

QueryId: typing.TypeAlias = int
ComponentId: typing.TypeAlias = int
GetItemKey: typing.TypeAlias = (
    list[int]
    | list[bool]
    | npt.NDArray[np.uint32]
    | npt.NDArray[np.bool_]
    | slice
)
