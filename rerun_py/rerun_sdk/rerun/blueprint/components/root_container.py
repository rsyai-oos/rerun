# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/components/root_container.fbs".

# You can extend this class by creating a "RootContainerExt" class in "root_container_ext.py".

from __future__ import annotations

from ... import datatypes
from ..._baseclasses import (
    ComponentBatchMixin,
    ComponentDescriptor,
    ComponentMixin,
)

__all__ = ["RootContainer", "RootContainerBatch"]


class RootContainer(datatypes.Uuid, ComponentMixin):
    """**Component**: The container that sits at the root of a viewport."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of RootContainerExt in root_container_ext.py

    # Note: there are no fields here because RootContainer delegates to datatypes.Uuid


class RootContainerBatch(datatypes.UuidBatch, ComponentBatchMixin):
    _COMPONENT_DESCRIPTOR: ComponentDescriptor = ComponentDescriptor("rerun.blueprint.components.RootContainer")


# This is patched in late to avoid circular dependencies.
RootContainer._BATCH_TYPE = RootContainerBatch  # type: ignore[assignment]
