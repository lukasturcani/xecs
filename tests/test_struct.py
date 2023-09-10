import xecs as xx


class MyStruct(xx.Struct):
    x: xx.Int


class MyComponent(xx.Component):
    s: MyStruct


def test_to_str() -> None:
    pool = MyComponent.create_pool(2)
    pool.p_spawn(2)
    assert (
        repr(pool.p_component)
        == str(pool.p_component)
        == (
            "<MyComponent(\n    "
            "s=<MyStruct(\n        x=<xecs.Int32 [0, 0]>,\n    )>,\n)>"
        )
    )
