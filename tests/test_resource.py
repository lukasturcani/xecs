import xecs as xx


class MyResource(xx.Resource):
    first: int
    second: float
    third: str


def test_resource_creation() -> None:
    resource = MyResource(1, second=32, third="hi")
    assert resource.first == 1
    assert resource.second == 32.0
    assert resource.third == "hi"
