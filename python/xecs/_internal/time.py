from xecs import xecs
from xecs._internal.resource import Resource


class Time(Resource):
    time: xecs.Time

    @staticmethod
    def default() -> "Time":
        return Time(xecs.Time.default())

    def delta(self) -> xecs.Duration:
        return self.time.delta()

    def update(self) -> None:
        return self.time.update()

    def update_with_delta(self, delta: xecs.Duration) -> None:
        return self.time.update_with_delta(delta)

    def elapsed(self) -> xecs.Duration:
        return self.time.elapsed()
