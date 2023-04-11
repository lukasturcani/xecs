import ecstasy as ecs


class One(ecs.Component):
    x: ecs.Float64


class Two(ecs.Component):
    y: ecs.Float64


def system(query: ecs.Query[One, Two]) -> None:
    pass
