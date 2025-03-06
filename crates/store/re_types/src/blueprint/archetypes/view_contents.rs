// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/view_contents.fbs".

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

/// **Archetype**: The contents of a `View`.
///
/// The contents are found by combining a collection of `QueryExpression`s.
///
/// ```diff
/// + /world/**           # add everything…
/// - /world/roads/**     # …but remove all roads…
/// + /world/roads/main   # …but show main road
/// ```
///
/// If there is multiple matching rules, the most specific rule wins.
/// If there are multiple rules of the same specificity, the last one wins.
/// If no rules match, the path is excluded.
///
/// Specifying a path without a `+` or `-` prefix is equivalent to `+`:
/// ```diff
/// /world/**           # add everything…
/// - /world/roads/**   # …but remove all roads…
/// /world/roads/main   # …but show main road
/// ```
///
/// The `/**` suffix matches the whole subtree, i.e. self and any child, recursively
/// (`/world/**` matches both `/world` and `/world/car/driver`).
/// Other uses of `*` are not (yet) supported.
///
/// Internally, `EntityPathFilter` sorts the rule by entity path, with recursive coming before non-recursive.
/// This means the last matching rule is also the most specific one. For instance:
/// ```diff
/// + /world/**
/// - /world
/// - /world/car/**
/// + /world/car/driver
/// ```
///
/// The last rule matching `/world/car/driver` is `+ /world/car/driver`, so it is included.
/// The last rule matching `/world/car/hood` is `- /world/car/**`, so it is excluded.
/// The last rule matching `/world` is `- /world`, so it is excluded.
/// The last rule matching `/world/house` is `+ /world/**`, so it is included.
#[derive(Clone, Debug, Default)]
pub struct ViewContents {
    /// The `QueryExpression` that populates the contents for the view.
    ///
    /// They determine which entities are part of the view.
    pub query: Option<SerializedComponentBatch>,
}

impl ViewContents {
    /// Returns the [`ComponentDescriptor`] for [`Self::query`].
    #[inline]
    pub fn descriptor_query() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ViewContents".into()),
            component_name: "rerun.blueprint.components.QueryExpression".into(),
            archetype_field_name: Some("query".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ViewContents".into()),
            component_name: "rerun.blueprint.components.ViewContentsIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [ViewContents::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [ViewContents::descriptor_query()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ViewContents::descriptor_indicator(),
            ViewContents::descriptor_query(),
        ]
    });

impl ViewContents {
    /// The total number of components in the archetype: 0 required, 1 recommended, 1 optional
    pub const NUM_COMPONENTS: usize = 2usize;
}

/// Indicator component for the [`ViewContents`] [`::re_types_core::Archetype`]
pub type ViewContentsIndicator = ::re_types_core::GenericIndicatorComponent<ViewContents>;

impl ::re_types_core::Archetype for ViewContents {
    type Indicator = ViewContentsIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.ViewContents".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "View contents"
    }

    #[inline]
    fn indicator() -> SerializedComponentBatch {
        #[allow(clippy::unwrap_used)]
        ViewContentsIndicator::DEFAULT.serialized().unwrap()
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
        let query = arrays_by_descr
            .get(&Self::descriptor_query())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_query()));
        Ok(Self { query })
    }
}

impl ::re_types_core::AsComponents for ViewContents {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [Some(Self::indicator()), self.query.clone()]
            .into_iter()
            .flatten()
            .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for ViewContents {}

impl ViewContents {
    /// Create a new `ViewContents`.
    #[inline]
    pub fn new(
        query: impl IntoIterator<Item = impl Into<crate::blueprint::components::QueryExpression>>,
    ) -> Self {
        Self {
            query: try_serialize_field(Self::descriptor_query(), query),
        }
    }

    /// Update only some specific fields of a `ViewContents`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `ViewContents`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            query: Some(SerializedComponentBatch::new(
                crate::blueprint::components::QueryExpression::arrow_empty(),
                Self::descriptor_query(),
            )),
        }
    }

    /// The `QueryExpression` that populates the contents for the view.
    ///
    /// They determine which entities are part of the view.
    #[inline]
    pub fn with_query(
        mut self,
        query: impl IntoIterator<Item = impl Into<crate::blueprint::components::QueryExpression>>,
    ) -> Self {
        self.query = try_serialize_field(Self::descriptor_query(), query);
        self
    }
}

impl ::re_byte_size::SizeBytes for ViewContents {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.query.heap_size_bytes()
    }
}
