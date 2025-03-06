// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/transform_mat3x3.fbs".

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

/// **Component**: A 3x3 transformation matrix Matrix that doesn't propagate in the transform hierarchy.
///
/// 3x3 matrixes are able to represent any affine transformation in 3D space,
/// i.e. rotation, scaling, shearing, reflection etc.
///
/// Matrices in Rerun are stored as flat list of coefficients in column-major order:
/// ```text
///             column 0       column 1       column 2
///        -------------------------------------------------
/// row 0 | flat_columns[0] flat_columns[3] flat_columns[6]
/// row 1 | flat_columns[1] flat_columns[4] flat_columns[7]
/// row 2 | flat_columns[2] flat_columns[5] flat_columns[8]
/// ```
#[derive(Clone, Debug, Default, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct PoseTransformMat3x3(pub crate::datatypes::Mat3x3);

impl ::re_types_core::Component for PoseTransformMat3x3 {
    #[inline]
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor::new("rerun.components.PoseTransformMat3x3")
    }
}

::re_types_core::macros::impl_into_cow!(PoseTransformMat3x3);

impl ::re_types_core::Loggable for PoseTransformMat3x3 {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        crate::datatypes::Mat3x3::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::Mat3x3::to_arrow_opt(data.into_iter().map(|datum| {
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
        crate::datatypes::Mat3x3::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }

    #[inline]
    fn from_arrow(arrow_data: &dyn arrow::array::Array) -> DeserializationResult<Vec<Self>>
    where
        Self: Sized,
    {
        crate::datatypes::Mat3x3::from_arrow(arrow_data).map(bytemuck::cast_vec)
    }
}

impl<T: Into<crate::datatypes::Mat3x3>> From<T> for PoseTransformMat3x3 {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::Mat3x3> for PoseTransformMat3x3 {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::Mat3x3 {
        &self.0
    }
}

impl std::ops::Deref for PoseTransformMat3x3 {
    type Target = crate::datatypes::Mat3x3;

    #[inline]
    fn deref(&self) -> &crate::datatypes::Mat3x3 {
        &self.0
    }
}

impl std::ops::DerefMut for PoseTransformMat3x3 {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::Mat3x3 {
        &mut self.0
    }
}

impl ::re_byte_size::SizeBytes for PoseTransformMat3x3 {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::Mat3x3>::is_pod()
    }
}
