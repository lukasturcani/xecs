from dataclasses import dataclass

import pytest
import xecs as xx


@dataclass(frozen=True, slots=True)
class Boom:
    message: str


class NumCalls(xx.Resource):
    count: int


def first_writer(writer: xx.EventWriter[Boom]) -> None:
    writer.send(Boom("first"))


def second_writer(writer: xx.EventWriter[Boom]) -> None:
    writer.send(Boom("second"))


def first_reader(reader: xx.EventReader[Boom], num_calls: NumCalls) -> None:
    num_calls.count += 1
    if num_calls.count == 1:
        assert reader.events == [Boom("first")]
    else:
        assert reader.events == [Boom("second"), Boom("first")]


def second_reader(reader: xx.EventReader[Boom]) -> None:
    assert reader.events == [Boom("first"), Boom("second")]


def test_events(app: xx.RealTimeApp) -> None:
    app.add_resource(NumCalls(0))
    app.add_system(first_writer)
    app.add_system(first_reader)
    app.add_system(second_writer)
    app.add_system(second_reader)
    app.update()
    app.update()


@pytest.fixture
def app() -> xx.RealTimeApp:
    app = xx.RealTimeApp()
    return app
