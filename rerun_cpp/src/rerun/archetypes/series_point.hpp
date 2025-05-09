// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/series_point.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../component_batch.hpp"
#include "../component_column.hpp"
#include "../components/color.hpp"
#include "../components/marker_shape.hpp"
#include "../components/marker_size.hpp"
#include "../components/name.hpp"
#include "../components/series_visible.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

RR_PUSH_WARNINGS
RR_DISABLE_DEPRECATION_WARNING

namespace rerun::archetypes {
    /// **Archetype**: Define the style properties for a point series in a chart.
    ///
    /// This archetype only provides styling information and should be logged as static
    /// when possible. The underlying data needs to be logged to the same entity-path using
    /// `archetypes::Scalars`.
    ///
    /// ## Example
    ///
    /// ### Point series
    /// ![image](https://static.rerun.io/series_point_style/82207a705da6c086b28ce161db1db9e8b12258b7/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// #include <cmath>
    ///
    /// constexpr float TAU = 6.28318530717958647692528676655900577f;
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_series_point_style");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     // Set up plot styling:
    ///     // They are logged static as they don't change over time and apply to all timelines.
    ///     // Log two point series under a shared root so that they show in the same plot by default.
    ///     rec.log_static(
    ///         "trig/sin",
    ///         rerun::SeriesPoints()
    ///             .with_colors(rerun::Rgba32{255, 0, 0})
    ///             .with_names("sin(0.01t)")
    ///             .with_markers(rerun::components::MarkerShape::Circle)
    ///             .with_marker_sizes(4.0f)
    ///     );
    ///     rec.log_static(
    ///         "trig/cos",
    ///         rerun::SeriesPoints()
    ///             .with_colors(rerun::Rgba32{0, 255, 0})
    ///             .with_names("cos(0.01t)")
    ///             .with_markers(rerun::components::MarkerShape::Cross)
    ///             .with_marker_sizes(2.0f)
    ///     );
    ///
    ///     // Log the data on a timeline called "step".
    ///     for (int t = 0; t <static_cast<int>(TAU * 2.0 * 10.0); ++t) {
    ///         rec.set_time_sequence("step", t);
    ///
    ///         rec.log("trig/sin", rerun::Scalars(sin(static_cast<double>(t) / 10.0)));
    ///         rec.log("trig/cos", rerun::Scalars(cos(static_cast<double>(t) / 10.0)));
    ///     }
    /// }
    /// ```
    ///
    /// ⚠ **Deprecated since 0.23.0**: Use `SeriesPoints` instead.
    ///
    struct [[deprecated("since 0.23.0: Use `SeriesPoints` instead.")]] SeriesPoint {
        /// Color for the corresponding series.
        std::optional<ComponentBatch> color;

        /// What shape to use to represent the point
        std::optional<ComponentBatch> marker;

        /// Display name of the series.
        ///
        /// Used in the legend.
        std::optional<ComponentBatch> name;

        /// Which point series are visible.
        ///
        /// If not set, all point series on this entity are visible.
        /// Unlike with the regular visibility property of the entire entity, any series that is hidden
        /// via this property will still be visible in the legend.
        std::optional<ComponentBatch> visible_series;

        /// Size of the marker.
        std::optional<ComponentBatch> marker_size;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.SeriesPointIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;
        /// The name of the archetype as used in `ComponentDescriptor`s.
        static constexpr const char ArchetypeName[] = "rerun.archetypes.SeriesPoint";

        /// `ComponentDescriptor` for the `color` field.
        static constexpr auto Descriptor_color = ComponentDescriptor(
            ArchetypeName, "color", Loggable<rerun::components::Color>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `marker` field.
        static constexpr auto Descriptor_marker = ComponentDescriptor(
            ArchetypeName, "marker",
            Loggable<rerun::components::MarkerShape>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `name` field.
        static constexpr auto Descriptor_name = ComponentDescriptor(
            ArchetypeName, "name", Loggable<rerun::components::Name>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `visible_series` field.
        static constexpr auto Descriptor_visible_series = ComponentDescriptor(
            ArchetypeName, "visible_series",
            Loggable<rerun::components::SeriesVisible>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `marker_size` field.
        static constexpr auto Descriptor_marker_size = ComponentDescriptor(
            ArchetypeName, "marker_size",
            Loggable<rerun::components::MarkerSize>::Descriptor.component_name
        );

      public:
        SeriesPoint() = default;
        SeriesPoint(SeriesPoint&& other) = default;
        SeriesPoint(const SeriesPoint& other) = default;
        SeriesPoint& operator=(const SeriesPoint& other) = default;
        SeriesPoint& operator=(SeriesPoint&& other) = default;

        /// Update only some specific fields of a `SeriesPoint`.
        static SeriesPoint update_fields() {
            return SeriesPoint();
        }

        /// Clear all the fields of a `SeriesPoint`.
        static SeriesPoint clear_fields();

        /// Color for the corresponding series.
        SeriesPoint with_color(const rerun::components::Color& _color) && {
            color = ComponentBatch::from_loggable(_color, Descriptor_color).value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `color` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_color` should
        /// be used when logging a single row's worth of data.
        SeriesPoint with_many_color(const Collection<rerun::components::Color>& _color) && {
            color = ComponentBatch::from_loggable(_color, Descriptor_color).value_or_throw();
            return std::move(*this);
        }

        /// What shape to use to represent the point
        SeriesPoint with_marker(const rerun::components::MarkerShape& _marker) && {
            marker = ComponentBatch::from_loggable(_marker, Descriptor_marker).value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `marker` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_marker` should
        /// be used when logging a single row's worth of data.
        SeriesPoint with_many_marker(const Collection<rerun::components::MarkerShape>& _marker) && {
            marker = ComponentBatch::from_loggable(_marker, Descriptor_marker).value_or_throw();
            return std::move(*this);
        }

        /// Display name of the series.
        ///
        /// Used in the legend.
        SeriesPoint with_name(const rerun::components::Name& _name) && {
            name = ComponentBatch::from_loggable(_name, Descriptor_name).value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `name` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_name` should
        /// be used when logging a single row's worth of data.
        SeriesPoint with_many_name(const Collection<rerun::components::Name>& _name) && {
            name = ComponentBatch::from_loggable(_name, Descriptor_name).value_or_throw();
            return std::move(*this);
        }

        /// Which point series are visible.
        ///
        /// If not set, all point series on this entity are visible.
        /// Unlike with the regular visibility property of the entire entity, any series that is hidden
        /// via this property will still be visible in the legend.
        SeriesPoint with_visible_series(
            const Collection<rerun::components::SeriesVisible>& _visible_series
        ) && {
            visible_series =
                ComponentBatch::from_loggable(_visible_series, Descriptor_visible_series)
                    .value_or_throw();
            return std::move(*this);
        }

        /// Size of the marker.
        SeriesPoint with_marker_size(const rerun::components::MarkerSize& _marker_size) && {
            marker_size = ComponentBatch::from_loggable(_marker_size, Descriptor_marker_size)
                              .value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `marker_size` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_marker_size` should
        /// be used when logging a single row's worth of data.
        SeriesPoint with_many_marker_size(
            const Collection<rerun::components::MarkerSize>& _marker_size
        ) && {
            marker_size = ComponentBatch::from_loggable(_marker_size, Descriptor_marker_size)
                              .value_or_throw();
            return std::move(*this);
        }

        /// Partitions the component data into multiple sub-batches.
        ///
        /// Specifically, this transforms the existing `ComponentBatch` data into `ComponentColumn`s
        /// instead, via `ComponentBatch::partitioned`.
        ///
        /// This makes it possible to use `RecordingStream::send_columns` to send columnar data directly into Rerun.
        ///
        /// The specified `lengths` must sum to the total length of the component batch.
        Collection<ComponentColumn> columns(const Collection<uint32_t>& lengths_);

        /// Partitions the component data into unit-length sub-batches.
        ///
        /// This is semantically similar to calling `columns` with `std::vector<uint32_t>(n, 1)`,
        /// where `n` is automatically guessed.
        Collection<ComponentColumn> columns();
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::SeriesPoint> {
        /// Serialize all set component batches.
        static Result<Collection<ComponentBatch>> as_batches(
            const archetypes::SeriesPoint& archetype
        );
    };
} // namespace rerun

RR_POP_WARNINGS
