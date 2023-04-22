import typing

import ecstasy as ecs
from ecstasy._internal.rust_type_aliases import GetItemKey

FloatArray: typing.TypeAlias = ecs.Float32 | ecs.Float64
IntArray: typing.TypeAlias = (
    ecs.Int8
    | ecs.Int16
    | ecs.Int32
    | ecs.Int64
    | ecs.UInt8
    | ecs.UInt16
    | ecs.UInt32
    | ecs.UInt64
)
Array: typing.TypeAlias = FloatArray | IntArray
