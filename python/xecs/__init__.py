"""
A fast ECS library.
"""

from xecs._internal.app import (
    App,
    FixedTimeStepSystems,
    FixedTimeStepSystemSpec,
    PendingStartupSystems,
    PendingSystems,
    Plugin,
    StartupSystems,
    Systems,
    SystemSpec,
)
from xecs._internal.commands import Commands
from xecs._internal.component import Component, ComponentPool
from xecs._internal.float import Float
from xecs._internal.py_field import PyField
from xecs._internal.query import Query
from xecs._internal.resource import Resource
from xecs._internal.struct import Struct
from xecs._internal.transform2 import Transform2
from xecs._internal.vec2 import Vec2
from xecs._internal.world import World
from xecs.xecs import ArrayViewIndices, Duration, Float32

__all__ = [
    "App",
    "ArrayViewIndices",
    "Commands",
    "Component",
    "ComponentPool",
    "Duration",
    "FixedTimeStepSystems",
    "FixedTimeStepSystemSpec",
    "Float",
    "Float32",
    "PendingStartupSystems",
    "PendingSystems",
    "Plugin",
    "PyField",
    "Query",
    "Resource",
    "StartupSystems",
    "Struct",
    "Systems",
    "SystemSpec",
    "Transform2",
    "Vec2",
    "World",
]
