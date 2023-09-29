"""
A fast ECS library.
"""

from xecs._internal.commands import Commands
from xecs._internal.component import Component, ComponentPool
from xecs._internal.float import Float
from xecs._internal.int import Int
from xecs._internal.py_field import PyField, py_field
from xecs._internal.query import Query
from xecs._internal.real_time_app import (
    RealTimeApp,
    RealTimeAppPlugin,
)
from xecs._internal.resource import Resource
from xecs._internal.simulation_app import SimulationApp
from xecs._internal.struct import Struct
from xecs._internal.systems import (
    FixedTimeStepSystems,
    FixedTimeStepSystemSpec,
    PendingStartupSystems,
    PendingSystems,
    StartupSystems,
    Systems,
    SystemSpec,
)
from xecs._internal.transform2 import Transform2
from xecs._internal.vec2 import Vec2
from xecs._internal.world import World
from xecs.xecs import ArrayViewIndices, Bool, Duration, Float32, Int32

__all__ = [
    "ArrayViewIndices",
    "Bool",
    "Commands",
    "Component",
    "ComponentPool",
    "Duration",
    "FixedTimeStepSystems",
    "FixedTimeStepSystemSpec",
    "Float",
    "Float32",
    "Int",
    "Int32",
    "PendingStartupSystems",
    "PendingSystems",
    "PyField",
    "py_field",
    "Query",
    "RealTimeApp",
    "RealTimeAppPlugin",
    "Resource",
    "SimulationApp",
    "StartupSystems",
    "Struct",
    "Systems",
    "SystemSpec",
    "Transform2",
    "Vec2",
    "World",
]
