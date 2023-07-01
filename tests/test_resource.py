import ecstasy as ecs


class MyResource(ecs.Resource):
    first: int
    second: float
    third: str


def test_resource_creation() -> None:
    resource = MyResource(1, second=32, third="hi")
    assert resource.first == 1
    assert resource.second == 32.0
    assert resource.third == "hi"
