import inspect
import typing


class Struct:
    _len: int

    @classmethod
    def create_pool(cls, size: int) -> typing.Self:
        pool = cls()
        pool._len = size
        for key, value in inspect.get_annotations(cls).items():
            setattr(pool, key, value.create_pool(size))
        return pool

    def __len__(self) -> int:
        return self._len
