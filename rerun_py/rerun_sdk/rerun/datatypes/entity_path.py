# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/datatypes/entity_path.fbs".

# You can extend this class by creating a "EntityPathExt" class in "entity_path_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import pyarrow as pa
from attrs import define, field

from .._baseclasses import BaseBatch, BaseExtensionType
from .entity_path_ext import EntityPathExt

__all__ = ["EntityPath", "EntityPathArrayLike", "EntityPathBatch", "EntityPathLike", "EntityPathType"]


@define(init=False)
class EntityPath(EntityPathExt):
    """**Datatype**: A path to an entity in the `DataStore`."""

    def __init__(self: Any, path: EntityPathLike):
        """Create a new instance of the EntityPath datatype."""

        # You can define your own __init__ function as a member of EntityPathExt in entity_path_ext.py
        self.__attrs_init__(path=path)

    path: str = field(converter=str)

    def __str__(self) -> str:
        return str(self.path)


if TYPE_CHECKING:
    EntityPathLike = Union[EntityPath, str]
else:
    EntityPathLike = Any

EntityPathArrayLike = Union[EntityPath, Sequence[EntityPathLike], Sequence[str]]


class EntityPathType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.datatypes.EntityPath"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.utf8(), self._TYPE_NAME)


class EntityPathBatch(BaseBatch[EntityPathArrayLike]):
    _ARROW_TYPE = EntityPathType()

    @staticmethod
    def _native_to_pa_array(data: EntityPathArrayLike, data_type: pa.DataType) -> pa.Array:
        return EntityPathExt.native_to_pa_array_override(data, data_type)
