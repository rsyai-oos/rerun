// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/asset3d.fbs".

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

/// **Archetype**: A prepacked 3D asset (`.gltf`, `.glb`, `.obj`, `.stl`, etc.).
///
/// See also [`archetypes::Mesh3D`][crate::archetypes::Mesh3D].
///
/// If there are multiple [`archetypes::InstancePoses3D`][crate::archetypes::InstancePoses3D] instances logged to the same entity as a mesh,
/// an instance of the mesh will be drawn for each transform.
///
/// ## Example
///
/// ### Simple 3D asset
/// ```ignore
/// use rerun::external::anyhow;
///
/// fn main() -> anyhow::Result<()> {
///     let args = std::env::args().collect::<Vec<_>>();
///     let Some(path) = args.get(1) else {
///         anyhow::bail!("Usage: {} <path_to_asset.[gltf|glb|obj|stl]>", args[0]);
///     };
///
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_asset3d").spawn()?;
///
///     rec.log_static("world", &rerun::ViewCoordinates::RIGHT_HAND_Z_UP())?; // Set an up-axis
///     rec.log("world/asset", &rerun::Asset3D::from_file(path)?)?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/1200w.png">
///   <img src="https://static.rerun.io/asset3d_simple/af238578188d3fd0de3e330212120e2842a8ddb2/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Asset3D {
    /// The asset's bytes.
    pub blob: Option<SerializedComponentBatch>,

    /// The Media Type of the asset.
    ///
    /// Supported values:
    /// * `model/gltf-binary`
    /// * `model/gltf+json`
    /// * `model/obj` (.mtl material files are not supported yet, references are silently ignored)
    /// * `model/stl`
    ///
    /// If omitted, the viewer will try to guess from the data blob.
    /// If it cannot guess, it won't be able to render the asset.
    pub media_type: Option<SerializedComponentBatch>,

    /// A color multiplier applied to the whole asset.
    ///
    /// For mesh who already have `albedo_factor` in materials,
    /// it will be overwritten by actual `albedo_factor` of [`archetypes::Asset3D`][crate::archetypes::Asset3D] (if specified).
    pub albedo_factor: Option<SerializedComponentBatch>,
}

impl Asset3D {
    /// Returns the [`ComponentDescriptor`] for [`Self::blob`].
    #[inline]
    pub fn descriptor_blob() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Asset3D".into()),
            component_name: "rerun.components.Blob".into(),
            archetype_field_name: Some("blob".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::media_type`].
    #[inline]
    pub fn descriptor_media_type() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Asset3D".into()),
            component_name: "rerun.components.MediaType".into(),
            archetype_field_name: Some("media_type".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::albedo_factor`].
    #[inline]
    pub fn descriptor_albedo_factor() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Asset3D".into()),
            component_name: "rerun.components.AlbedoFactor".into(),
            archetype_field_name: Some("albedo_factor".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Asset3D".into()),
            component_name: "rerun.components.Asset3DIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Asset3D::descriptor_blob()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Asset3D::descriptor_media_type(),
            Asset3D::descriptor_indicator(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Asset3D::descriptor_albedo_factor()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Asset3D::descriptor_blob(),
            Asset3D::descriptor_media_type(),
            Asset3D::descriptor_indicator(),
            Asset3D::descriptor_albedo_factor(),
        ]
    });

impl Asset3D {
    /// The total number of components in the archetype: 1 required, 2 recommended, 1 optional
    pub const NUM_COMPONENTS: usize = 4usize;
}

/// Indicator component for the [`Asset3D`] [`::re_types_core::Archetype`]
pub type Asset3DIndicator = ::re_types_core::GenericIndicatorComponent<Asset3D>;

impl ::re_types_core::Archetype for Asset3D {
    type Indicator = Asset3DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Asset3D".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Asset 3D"
    }

    #[inline]
    fn indicator() -> SerializedComponentBatch {
        #[allow(clippy::unwrap_used)]
        Asset3DIndicator::DEFAULT.serialized().unwrap()
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentDescriptor, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_descr: ::nohash_hasher::IntMap<_, _> = arrow_data.into_iter().collect();
        let blob = arrays_by_descr
            .get(&Self::descriptor_blob())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_blob()));
        let media_type = arrays_by_descr
            .get(&Self::descriptor_media_type())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_media_type())
            });
        let albedo_factor = arrays_by_descr
            .get(&Self::descriptor_albedo_factor())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_albedo_factor())
            });
        Ok(Self {
            blob,
            media_type,
            albedo_factor,
        })
    }
}

impl ::re_types_core::AsComponents for Asset3D {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.blob.clone(),
            self.media_type.clone(),
            self.albedo_factor.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Asset3D {}

impl Asset3D {
    /// Create a new `Asset3D`.
    #[inline]
    pub fn new(blob: impl Into<crate::components::Blob>) -> Self {
        Self {
            blob: try_serialize_field(Self::descriptor_blob(), [blob]),
            media_type: None,
            albedo_factor: None,
        }
    }

    /// Update only some specific fields of a `Asset3D`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `Asset3D`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            blob: Some(SerializedComponentBatch::new(
                crate::components::Blob::arrow_empty(),
                Self::descriptor_blob(),
            )),
            media_type: Some(SerializedComponentBatch::new(
                crate::components::MediaType::arrow_empty(),
                Self::descriptor_media_type(),
            )),
            albedo_factor: Some(SerializedComponentBatch::new(
                crate::components::AlbedoFactor::arrow_empty(),
                Self::descriptor_albedo_factor(),
            )),
        }
    }

    /// Partitions the component data into multiple sub-batches.
    ///
    /// Specifically, this transforms the existing [`SerializedComponentBatch`]es data into [`SerializedComponentColumn`]s
    /// instead, via [`SerializedComponentBatch::partitioned`].
    ///
    /// This makes it possible to use `RecordingStream::send_columns` to send columnar data directly into Rerun.
    ///
    /// The specified `lengths` must sum to the total length of the component batch.
    ///
    /// [`SerializedComponentColumn`]: [::re_types_core::SerializedComponentColumn]
    #[inline]
    pub fn columns<I>(
        self,
        _lengths: I,
    ) -> SerializationResult<impl Iterator<Item = ::re_types_core::SerializedComponentColumn>>
    where
        I: IntoIterator<Item = usize> + Clone,
    {
        let columns = [
            self.blob
                .map(|blob| blob.partitioned(_lengths.clone()))
                .transpose()?,
            self.media_type
                .map(|media_type| media_type.partitioned(_lengths.clone()))
                .transpose()?,
            self.albedo_factor
                .map(|albedo_factor| albedo_factor.partitioned(_lengths.clone()))
                .transpose()?,
        ];
        Ok(columns
            .into_iter()
            .flatten()
            .chain([::re_types_core::indicator_column::<Self>(
                _lengths.into_iter().count(),
            )?]))
    }

    /// Helper to partition the component data into unit-length sub-batches.
    ///
    /// This is semantically similar to calling [`Self::columns`] with `std::iter::take(1).repeat(n)`,
    /// where `n` is automatically guessed.
    #[inline]
    pub fn columns_of_unit_batches(
        self,
    ) -> SerializationResult<impl Iterator<Item = ::re_types_core::SerializedComponentColumn>> {
        let len_blob = self.blob.as_ref().map(|b| b.array.len());
        let len_media_type = self.media_type.as_ref().map(|b| b.array.len());
        let len_albedo_factor = self.albedo_factor.as_ref().map(|b| b.array.len());
        let len = None
            .or(len_blob)
            .or(len_media_type)
            .or(len_albedo_factor)
            .unwrap_or(0);
        self.columns(std::iter::repeat(1).take(len))
    }

    /// The asset's bytes.
    #[inline]
    pub fn with_blob(mut self, blob: impl Into<crate::components::Blob>) -> Self {
        self.blob = try_serialize_field(Self::descriptor_blob(), [blob]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::Blob`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_blob`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_blob(
        mut self,
        blob: impl IntoIterator<Item = impl Into<crate::components::Blob>>,
    ) -> Self {
        self.blob = try_serialize_field(Self::descriptor_blob(), blob);
        self
    }

    /// The Media Type of the asset.
    ///
    /// Supported values:
    /// * `model/gltf-binary`
    /// * `model/gltf+json`
    /// * `model/obj` (.mtl material files are not supported yet, references are silently ignored)
    /// * `model/stl`
    ///
    /// If omitted, the viewer will try to guess from the data blob.
    /// If it cannot guess, it won't be able to render the asset.
    #[inline]
    pub fn with_media_type(mut self, media_type: impl Into<crate::components::MediaType>) -> Self {
        self.media_type = try_serialize_field(Self::descriptor_media_type(), [media_type]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::MediaType`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_media_type`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_media_type(
        mut self,
        media_type: impl IntoIterator<Item = impl Into<crate::components::MediaType>>,
    ) -> Self {
        self.media_type = try_serialize_field(Self::descriptor_media_type(), media_type);
        self
    }

    /// A color multiplier applied to the whole asset.
    ///
    /// For mesh who already have `albedo_factor` in materials,
    /// it will be overwritten by actual `albedo_factor` of [`archetypes::Asset3D`][crate::archetypes::Asset3D] (if specified).
    #[inline]
    pub fn with_albedo_factor(
        mut self,
        albedo_factor: impl Into<crate::components::AlbedoFactor>,
    ) -> Self {
        self.albedo_factor = try_serialize_field(Self::descriptor_albedo_factor(), [albedo_factor]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::AlbedoFactor`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_albedo_factor`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_albedo_factor(
        mut self,
        albedo_factor: impl IntoIterator<Item = impl Into<crate::components::AlbedoFactor>>,
    ) -> Self {
        self.albedo_factor = try_serialize_field(Self::descriptor_albedo_factor(), albedo_factor);
        self
    }
}

impl ::re_byte_size::SizeBytes for Asset3D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.blob.heap_size_bytes()
            + self.media_type.heap_size_bytes()
            + self.albedo_factor.heap_size_bytes()
    }
}
