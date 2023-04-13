import typing

import numpy as np
import numpy.typing as npt

Key: typing.TypeAlias = npt.NDArray[np.uint32 | np.bool_] | slice
