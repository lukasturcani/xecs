from typing import Any, cast

import polars as pl


class Int32:
    def __get__(self, instance: Any, owner: type | None = None) -> pl.Expr:
        return pl.col(self._name)

    def __set_name__(self, owner: type, name: str) -> None:
        self._name = name

    @classmethod
    def iadd(cls):
        pass


def int32(*, default: int) -> Int32:
    """
    Provide additional data about a component field.

    Parameters:
        default: The default value for the field.
    """
    return cast(Int32, default)
