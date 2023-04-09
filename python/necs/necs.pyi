import numpy as np
import numpy.typing as npt

from necs._internal.element_type import ElementType

def assign_value_at_indices(
    array: npt.NDArray[ElementType],
    indices: npt.NDArray[np.uint64],
    value: ElementType,
) -> None: ...
