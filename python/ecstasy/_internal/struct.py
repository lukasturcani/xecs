import inspect
import typing


class Struct:
    _len: int

    @classmethod
    def p_create_pool(cls, size: int) -> typing.Self:
        pool = cls()
        pool._len = size
        for key, value in inspect.get_annotations(cls).items():
            setattr(pool, key, value.p_create_pool(size))
        return pool

    def p_spawn(self, num: int) -> None:
        for attr in inspect.get_annotations(self.__class__):
            getattr(self, attr).p_spawn(num)

    def __len__(self) -> int:
        return self._len
