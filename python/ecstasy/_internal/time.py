from ecstasy import ecstasy
from ecstasy._internal.resource import Resource


class Time(Resource):
    time: ecstasy.Time

    @staticmethod
    def default() -> "Time":
        return Time(ecstasy.Time.default())

    def delta(self) -> ecstasy.Duration:
        return self.time.delta()

    def update(self) -> None:
        self.time.update()

    def elapsed(self) -> ecstasy.Duration:
        return self.time.elapsed()
