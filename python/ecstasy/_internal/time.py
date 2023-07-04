from ecstasy import ecstasy
from ecstasy._internal.resource import Resource


class Time(Resource):
    time: ecstasy.Time

    def delta(self) -> ecstasy.Duration:
        return self.time.delta()

    def update(self) -> None:
        self.time.update()
