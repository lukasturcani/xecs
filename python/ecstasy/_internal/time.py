import time

from ecstasy._internal.resource import Resource


class Time(Resource):
    _startup: int
    _delta: int
