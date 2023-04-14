import typing

import numpy as np
import numpy.typing as npt

QueryId: typing.TypeAlias = int
ComponentId: typing.TypeAlias = int
Key: typing.TypeAlias = npt.NDArray[np.uint32 | np.bool_] | slice
