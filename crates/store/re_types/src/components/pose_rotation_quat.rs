// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/rotation_quat.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch as _, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: A 3D rotation expressed as a quaternion that doesn't propagate in the transform hierarchy.
///
/// Note: although the x,y,z,w components of the quaternion will be passed through to the
/// datastore as provided, when used in the Viewer, quaternions will always be normalized.
/// If normalization fails the rotation is treated as an invalid transform.
#[derive(Clone, Debug, Default, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct PoseRotationQuat(pub crate::datatypes::Quaternion);

impl ::re_types_core::Component for PoseRotationQuat {
    #[inline]
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor::new("rerun.components.PoseRotationQuat")
    }
}

::re_types_core::macros::impl_into_cow!(PoseRotationQuat);

impl ::re_types_core::Loggable for PoseRotationQuat {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        crate::datatypes::Quaternion::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::Quaternion::to_arrow_opt(data.into_iter().map(|datum| {
            datum.map(|datum| match datum.into() {
                ::std::borrow::Cow::Borrowed(datum) => ::std::borrow::Cow::Borrowed(&datum.0),
                ::std::borrow::Cow::Owned(datum) => ::std::borrow::Cow::Owned(datum.0),
            })
        }))
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        crate::datatypes::Quaternion::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }

    #[inline]
    fn from_arrow(arrow_data: &dyn arrow::array::Array) -> DeserializationResult<Vec<Self>>
    where
        Self: Sized,
    {
        crate::datatypes::Quaternion::from_arrow(arrow_data).map(bytemuck::cast_vec)
    }
}

impl<T: Into<crate::datatypes::Quaternion>> From<T> for PoseRotationQuat {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::Quaternion> for PoseRotationQuat {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::Quaternion {
        &self.0
    }
}

impl std::ops::Deref for PoseRotationQuat {
    type Target = crate::datatypes::Quaternion;

    #[inline]
    fn deref(&self) -> &crate::datatypes::Quaternion {
        &self.0
    }
}

impl std::ops::DerefMut for PoseRotationQuat {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::Quaternion {
        &mut self.0
    }
}

impl ::re_byte_size::SizeBytes for PoseRotationQuat {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::Quaternion>::is_pod()
    }
}
