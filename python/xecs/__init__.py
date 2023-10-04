"""
A fast ECS library.
"""

from xecs._internal.bool_ import bool_
from xecs._internal.commands import Commands
from xecs._internal.component import Component, ComponentPool
from xecs._internal.events import EventReader, EventWriter
from xecs._internal.float32 import float32
from xecs._internal.float_ import Float, float_
from xecs._internal.input import Keyboard, KeyboardButton, Mouse, MouseButton
from xecs._internal.int32 import int32
from xecs._internal.int_ import Int, int_
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
    "bool_",
    "Bool",
    "Commands",
    "Component",
    "ComponentPool",
    "Duration",
    "EventReader",
    "EventWriter",
    "FixedTimeStepSystems",
    "FixedTimeStepSystemSpec",
    "float_",
    "Float",
    "float32",
    "Float32",
    "int_",
    "Int",
    "int32",
    "Int32",
    "Keyboard",
    "KeyboardButton",
    "Mouse",
    "MouseButton",
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
