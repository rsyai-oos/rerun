// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/datatypes/filter_by_range.fbs".

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

/// **Datatype**: Configuration for the filter-by-range feature of the dataframe view.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilterByRange {
    /// Beginning of the time range.
    pub start: crate::datatypes::TimeInt,

    /// End of the time range (inclusive).
    pub end: crate::datatypes::TimeInt,
}

::re_types_core::macros::impl_into_cow!(FilterByRange);

impl ::re_types_core::Loggable for FilterByRange {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow::datatypes::*;
        DataType::Struct(Fields::from(vec![
            Field::new(
                "start",
                <crate::datatypes::TimeInt>::arrow_datatype(),
                false,
            ),
            Field::new("end", <crate::datatypes::TimeInt>::arrow_datatype(), false),
        ]))
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use ::re_types_core::{arrow_helpers::as_array_ref, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok({
            let fields = Fields::from(vec![
                Field::new(
                    "start",
                    <crate::datatypes::TimeInt>::arrow_datatype(),
                    false,
                ),
                Field::new("end", <crate::datatypes::TimeInt>::arrow_datatype(), false),
            ]);
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let validity: Option<arrow::buffer::NullBuffer> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            as_array_ref(StructArray::new(
                fields,
                vec![
                    {
                        let (somes, start): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.start.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let start_validity: Option<arrow::buffer::NullBuffer> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        as_array_ref(PrimitiveArray::<Int64Type>::new(
                            ScalarBuffer::from(
                                start
                                    .into_iter()
                                    .map(|datum| datum.map(|datum| datum.0).unwrap_or_default())
                                    .collect::<Vec<_>>(),
                            ),
                            start_validity,
                        ))
                    },
                    {
                        let (somes, end): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.end.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let end_validity: Option<arrow::buffer::NullBuffer> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        as_array_ref(PrimitiveArray::<Int64Type>::new(
                            ScalarBuffer::from(
                                end.into_iter()
                                    .map(|datum| datum.map(|datum| datum.0).unwrap_or_default())
                                    .collect::<Vec<_>>(),
                            ),
                            end_validity,
                        ))
                    },
                ],
                validity,
            ))
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{arrow_zip_validity::ZipValidity, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow::array::StructArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.blueprint.datatypes.FilterByRange")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.columns());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name().as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let start = {
                    if !arrays_by_name.contains_key("start") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "start",
                        ))
                        .with_context("rerun.blueprint.datatypes.FilterByRange");
                    }
                    let arrow_data = &**arrays_by_name["start"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<Int64Array>()
                        .ok_or_else(|| {
                            let expected = DataType::Int64;
                            let actual = arrow_data.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.blueprint.datatypes.FilterByRange#start")?
                        .into_iter()
                        .map(|res_or_opt| res_or_opt.map(crate::datatypes::TimeInt))
                };
                let end = {
                    if !arrays_by_name.contains_key("end") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "end",
                        ))
                        .with_context("rerun.blueprint.datatypes.FilterByRange");
                    }
                    let arrow_data = &**arrays_by_name["end"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<Int64Array>()
                        .ok_or_else(|| {
                            let expected = DataType::Int64;
                            let actual = arrow_data.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.blueprint.datatypes.FilterByRange#end")?
                        .into_iter()
                        .map(|res_or_opt| res_or_opt.map(crate::datatypes::TimeInt))
                };
                ZipValidity::new_with_validity(::itertools::izip!(start, end), arrow_data.nulls())
                    .map(|opt| {
                        opt.map(|(start, end)| {
                            Ok(Self {
                                start: start
                                    .ok_or_else(DeserializationError::missing_data)
                                    .with_context(
                                        "rerun.blueprint.datatypes.FilterByRange#start",
                                    )?,
                                end: end
                                    .ok_or_else(DeserializationError::missing_data)
                                    .with_context("rerun.blueprint.datatypes.FilterByRange#end")?,
                            })
                        })
                        .transpose()
                    })
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.blueprint.datatypes.FilterByRange")?
            }
        })
    }
}

impl ::re_byte_size::SizeBytes for FilterByRange {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.start.heap_size_bytes() + self.end.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::TimeInt>::is_pod() && <crate::datatypes::TimeInt>::is_pod()
    }
}
