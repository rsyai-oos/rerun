# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

__all__ = ["Radius", "RadiusArray", "RadiusArrayLike", "RadiusLike", "RadiusType"]

from dataclasses import dataclass
from typing import Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa


@dataclass
class Radius:
    """A Radius component."""

    value: float

    def __array__(self) -> npt.ArrayLike:
        return np.asarray(self.value)


RadiusLike = Union[Radius, float]

RadiusArrayLike = Union[RadiusLike, Sequence[RadiusLike], npt.NDArray[np.float32]]


# --- Arrow support ---

from .radius_ext import RadiusArrayExt  # noqa: E402


class RadiusType(pa.ExtensionType):  # type: ignore[misc]
    def __init__(self: type[pa.ExtensionType]) -> None:
        pa.ExtensionType.__init__(self, pa.float32(), "rerun.radius")

    def __arrow_ext_serialize__(self: type[pa.ExtensionType]) -> bytes:
        # since we don't have a parameterized type, we don't need extra metadata to be deserialized
        return b""

    @classmethod
    def __arrow_ext_deserialize__(
        cls: type[pa.ExtensionType], storage_type: Any, serialized: Any
    ) -> type[pa.ExtensionType]:
        # return an instance of this subclass given the serialized metadata.
        return RadiusType()

    def __arrow_ext_class__(self: type[pa.ExtensionType]) -> type[pa.ExtensionArray]:
        return RadiusArray


# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(RadiusType())


class RadiusArray(pa.ExtensionArray, RadiusArrayExt):  # type: ignore[misc]
    @staticmethod
    def from_similar(data: RadiusArrayLike | None) -> pa.Array:
        if data is None:
            return RadiusType().wrap_array(pa.array([], type=RadiusType().storage_type))
        else:
            return RadiusArrayExt._from_similar(
                data,
                mono=Radius,
                mono_aliases=RadiusLike,
                many=RadiusArray,
                many_aliases=RadiusArrayLike,
                arrow=RadiusType,
            )
