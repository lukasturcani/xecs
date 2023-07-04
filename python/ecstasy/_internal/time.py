from ecstasy._internal.resource import Resource
from ecstasy.ecstasy import Duration, Instant


class Time(Resource):
    startup: Instant
    delta: Duration
