import xecs as xx


class System1Ticks(xx.Resource):
    num: int


class System2Ticks(xx.Resource):
    num: int


def system1(ticks: System1Ticks) -> None:
    ticks.num += 1


def system2(ticks: System2Ticks) -> None:
    ticks.num += 1


def test_scheduling() -> None:
    app = xx.RealTimeApp()
    system1_ticks = System1Ticks(num=0)
    system2_ticks = System2Ticks(num=0)
    app.add_resource(system1_ticks)
    app.add_resource(system2_ticks)
    app.add_system(system1)
    app.add_system(system2, xx.Duration.from_millis(2))
    app.run(
        frame_time=xx.Duration.from_millis(1),
        max_run_time=xx.Duration.from_millis(2),
    )
    assert system1_ticks.num == 3
    assert system2_ticks.num == 1
