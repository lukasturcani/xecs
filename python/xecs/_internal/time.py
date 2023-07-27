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
        self.time.update()

    def elapsed(self) -> xecs.Duration:
        return self.time.elapsed()
