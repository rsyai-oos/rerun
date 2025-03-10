import os
from collections.abc import Iterator, Sequence
from enum import Enum
from typing import Optional

import pyarrow as pa

from .types import (
    AnyColumn,
    AnyComponentColumn,
    ComponentLike,
    IndexValuesLike,
    TableLike,
    VectorDistanceMetricLike,
    VectorLike,
    ViewContentsLike,
)

class IndexColumnDescriptor:
    """
    The descriptor of an index column.

    Index columns contain the index values for when the data was updated. They
    generally correspond to Rerun timelines.

    Column descriptors are used to describe the columns in a
    [`Schema`][rerun.dataframe.Schema]. They are read-only. To select an index
    column, use [`IndexColumnSelector`][rerun.dataframe.IndexColumnSelector].
    """

    def name(self) -> str:
        """
        The name of the index.

        This property is read-only.
        """

    @property
    def is_static(self) -> bool:
        """Part of generic ColumnDescriptor interface: always False for Index."""

class IndexColumnSelector:
    """
    A selector for an index column.

    Index columns contain the index values for when the data was updated. They
    generally correspond to Rerun timelines.
    """

    def __init__(self, index: str) -> None:
        """
        Create a new `IndexColumnSelector`.

        Parameters
        ----------
        index : str
            The name of the index to select. Usually the name of a timeline.

        """

    def name(self) -> str:
        """
        The name of the index.

        This property is read-only.
        """

class ComponentColumnDescriptor:
    """
    The descriptor of a component column.

    Component columns contain the data for a specific component of an entity.

    Column descriptors are used to describe the columns in a
    [`Schema`][rerun.dataframe.Schema]. They are read-only. To select a component
    column, use [`ComponentColumnSelector`][rerun.dataframe.ComponentColumnSelector].
    """

    @property
    def entity_path(self) -> str:
        """
        The entity path.

        This property is read-only.
        """

    @property
    def component_name(self) -> str:
        """
        The component name.

        This property is read-only.
        """

    @property
    def is_static(self) -> bool:
        """
        Whether the column is static.

        This property is read-only.
        """

class ComponentColumnSelector:
    """
    A selector for a component column.

    Component columns contain the data for a specific component of an entity.
    """

    def __init__(self, entity_path: str, component: ComponentLike) -> None:
        """
        Create a new `ComponentColumnSelector`.

        Parameters
        ----------
        entity_path : str
            The entity path to select.
        component : ComponentLike
            The component to select

        """
    @property
    def entity_path(self) -> str:
        """
        The entity path.

        This property is read-only.
        """

    @property
    def component_name(self) -> str:
        """
        The component name.

        This property is read-only.
        """

class VectorDistanceMetric(Enum):
    """Which distance metric for use for vector index."""

    L2: VectorDistanceMetric
    COSINE: VectorDistanceMetric
    DOT: VectorDistanceMetric
    HAMMING: VectorDistanceMetric

class Schema:
    """
    The schema representing a set of available columns.

    Can be returned by [`Recording.schema()`][rerun.dataframe.Recording.schema] or
    [`RecordingView.schema()`][rerun.dataframe.RecordingView.schema].
    """

    def __iter__(self) -> Iterator[IndexColumnDescriptor | ComponentColumnDescriptor]:
        """Iterate over all the column descriptors in the schema."""

    def index_columns(self) -> list[IndexColumnDescriptor]:
        """Return a list of all the index columns in the schema."""

    def component_columns(self) -> list[ComponentColumnDescriptor]:
        """Return a list of all the component columns in the schema."""

    def column_for(self, entity_path: str, component: ComponentLike) -> Optional[ComponentColumnDescriptor]:
        """
        Look up the column descriptor for a specific entity path and component.

        Parameters
        ----------
        entity_path : str
            The entity path to look up.
        component : ComponentLike
            The component to look up.

        Returns
        -------
        Optional[ComponentColumnDescriptor]
            The column descriptor, if it exists.

        """

class RecordingView:
    """
    A view of a recording restricted to a given index, containing a specific set of entities and components.

    See [`Recording.view(…)`][rerun.dataframe.Recording.view] for details on how to create a `RecordingView`.

    Note: `RecordingView` APIs never mutate the underlying view. Instead, they
    always return new views with the requested modifications applied.

    The view will only contain a single row for each unique value of the index
    that is associated with a component column that was included in the view.
    Component columns that are not included via the view contents will not
    impact the rows that make up the view. If the same entity / component pair
    was logged to a given index multiple times, only the most recent row will be
    included in the view, as determined by the `row_id` column. This will
    generally be the last value logged, as row_ids are guaranteed to be
    monotonically increasing when data is sent from a single process.
    """

    def schema(self) -> Schema:
        """
        The schema describing all the columns available in the view.

        This schema will only contain the columns that are included in the view via
        the view contents.
        """

    def filter_range_sequence(self, start: int, end: int) -> RecordingView:
        """
        Filter the view to only include data between the given index sequence numbers.

        This range is inclusive and will contain both the value at the start and the value at the end.

        The view must be of a sequential index type to use this method.

        Parameters
        ----------
        start : int
            The inclusive start of the range.
        end : int
            The inclusive end of the range.

        Returns
        -------
        RecordingView
            A new view containing only the data within the specified range.

            The original view will not be modified.

        """

    def filter_range_seconds(self, start: float, end: float) -> RecordingView:
        """
        Filter the view to only include data between the given index values expressed as seconds.

        This range is inclusive and will contain both the value at the start and the value at the end.

        The view must be of a temporal index type to use this method.

        Parameters
        ----------
        start : int
            The inclusive start of the range.
        end : int
            The inclusive end of the range.

        Returns
        -------
        RecordingView
            A new view containing only the data within the specified range.

            The original view will not be modified.

        """

    def filter_range_nanos(self, start: int, end: int) -> RecordingView:
        """
        Filter the view to only include data between the given index values expressed as seconds.

        This range is inclusive and will contain both the value at the start and the value at the end.

        The view must be of a temporal index type to use this method.

        Parameters
        ----------
        start : int
            The inclusive start of the range.
        end : int
            The inclusive end of the range.

        Returns
        -------
        RecordingView
            A new view containing only the data within the specified range.

            The original view will not be modified.

        """

    def filter_index_values(self, values: IndexValuesLike) -> RecordingView:
        """
        Filter the view to only include data at the provided index values.

        The index values returned will be the intersection between the provided values and the
        original index values.

        This requires index values to be a precise match. Index values in Rerun are
        represented as i64 sequence counts or nanoseconds. This API does not expose an interface
        in floating point seconds, as the numerical conversion would risk false mismatches.

        Parameters
        ----------
        values : IndexValuesLike
            The index values to filter by.

        Returns
        -------
        RecordingView
            A new view containing only the data at the specified index values.

            The original view will not be modified.

        """

    def filter_is_not_null(self, column: AnyComponentColumn) -> RecordingView:
        """
        Filter the view to only include rows where the given component column is not null.

        This corresponds to rows for index values where this component was provided to Rerun explicitly
        via `.log()` or `.send_columns()`.

        Parameters
        ----------
        column : AnyComponentColumn
            The component column to filter by.

        Returns
        -------
        RecordingView
            A new view containing only the data where the specified component column is not null.

            The original view will not be modified.

        """

    def using_index_values(self, values: IndexValuesLike) -> RecordingView:
        """
        Replace the index in the view with the provided values.

        The output view will always have the same number of rows as the provided values, even if
        those rows are empty. Use with [`.fill_latest_at()`][rerun.dataframe.RecordingView.fill_latest_at]
        to populate these rows with the most recent data.

        This requires index values to be a precise match. Index values in Rerun are
        represented as i64 sequence counts or nanoseconds. This API does not expose an interface
        in floating point seconds, as the numerical conversion would risk false mismatches.

        Parameters
        ----------
        values : IndexValuesLike
            The index values to use.

        Returns
        -------
        RecordingView
            A new view containing the provided index values.

            The original view will not be modified.

        """

    def fill_latest_at(self) -> RecordingView:
        """
        Populate any null values in a row with the latest valid data according to the index.

        Returns
        -------
        RecordingView
            A new view with the null values filled in.

            The original view will not be modified.

        """

    def select(self, *args: AnyColumn, columns: Optional[Sequence[AnyColumn]] = None) -> pa.RecordBatchReader:
        """
        Select the columns from the view.

        If no columns are provided, all available columns will be included in
        the output.

        The selected columns do not change the rows that are included in the
        view. The rows are determined by the index values and the components
        that were included in the view contents, or can be overridden with
        [`.using_index_values()`][rerun.dataframe.RecordingView.using_index_values].

        If a column was not provided with data for a given row, it will be
        `null` in the output.

        The output is a [`pyarrow.RecordBatchReader`][] that can be used to read
        out the data.

        Parameters
        ----------
        *args : AnyColumn
            The columns to select.
        columns : Optional[Sequence[AnyColumn]], optional
            Alternatively the columns to select can be provided as a sequence.

        Returns
        -------
        pa.RecordBatchReader
            A reader that can be used to read out the selected data.

        """

    def select_static(self, *args: AnyColumn, columns: Optional[Sequence[AnyColumn]] = None) -> pa.RecordBatchReader:
        """
        Select only the static columns from the view.

        Because static data has no associated index values it does not cause a
        row to be generated in the output. If your view only contains static data
        this method allows you to select it without needing to provide index values.

        This method will always return a single row.

        Any non-static columns that are included in the selection will generate a warning
        and produce empty columns.


        Parameters
        ----------
        *args : AnyColumn
            The columns to select.
        columns : Optional[Sequence[AnyColumn]], optional
            Alternatively the columns to select can be provided as a sequence.

        Returns
        -------
        pa.RecordBatchReader
            A reader that can be used to read out the selected data.

        """

class Recording:
    """
    A single Rerun recording.

    This can be loaded from an RRD file using [`load_recording()`][rerun.dataframe.load_recording].

    A recording is a collection of data that was logged to Rerun. This data is organized
    as a column for each index (timeline) and each entity/component pair that was logged.

    You can examine the [`.schema()`][rerun.dataframe.Recording.schema] of the recording to see
    what data is available, or create a [`RecordingView`][rerun.dataframe.RecordingView] to
    to retrieve the data.
    """

    def schema(self) -> Schema:
        """The schema describing all the columns available in the recording."""

    def view(
        self,
        *,
        index: str,
        contents: ViewContentsLike,
        include_semantically_empty_columns: bool = False,
        include_indicator_columns: bool = False,
        include_tombstone_columns: bool = False,
    ) -> RecordingView:
        """
        Create a [`RecordingView`][rerun.dataframe.RecordingView] of the recording according to a particular index and content specification.

        The only type of index currently supported is the name of a timeline.

        The view will only contain a single row for each unique value of the index
        that is associated with a component column that was included in the view.
        Component columns that are not included via the view contents will not
        impact the rows that make up the view. If the same entity / component pair
        was logged to a given index multiple times, only the most recent row will be
        included in the view, as determined by the `row_id` column. This will
        generally be the last value logged, as row_ids are guaranteed to be
        monotonically increasing when data is sent from a single process.

        Parameters
        ----------
        index : str
            The index to use for the view. This is typically a timeline name.
        contents : ViewContentsLike
            The content specification for the view.

            This can be a single string content-expression such as: `"world/cameras/**"`, or a dictionary
            specifying multiple content-expressions and a respective list of components to select within
            that expression such as `{"world/cameras/**": ["ImageBuffer", "PinholeProjection"]}`.
        include_semantically_empty_columns : bool, optional
            Whether to include columns that are semantically empty, by default `False`.

            Semantically empty columns are components that are `null` or empty `[]` for every row in the recording.
        include_indicator_columns : bool, optional
            Whether to include indicator columns, by default `False`.

            Indicator columns are components used to represent the presence of an archetype within an entity.
        include_tombstone_columns : bool, optional
            Whether to include tombstone columns, by default `False`.

            Tombstone columns are components used to represent clears. However, even without the clear
            tombstone columns, the view will still apply the clear semantics when resolving row contents.

        Returns
        -------
        RecordingView
            The view of the recording.

        Examples
        --------
        All the data in the recording on the timeline "my_index":
        ```python
        recording.view(index="my_index", contents="/**")
        ```

        Just the Position3D components in the "points" entity:
        ```python
        recording.view(index="my_index", contents={"points": "Position3D"})
        ```

        """

    def recording_id(self) -> str:
        """The recording ID of the recording."""

    def application_id(self) -> str:
        """The application ID of the recording."""

class RRDArchive:
    """
    An archive loaded from an RRD.

    RRD archives may include 1 or more recordings or blueprints.
    """

    def num_recordings(self) -> int:
        """The number of recordings in the archive."""
    def all_recordings(self) -> list[Recording]:
        """All the recordings in the archive."""

def load_recording(path_to_rrd: str | os.PathLike) -> Recording:
    """
    Load a single recording from an RRD file.

    Will raise a `ValueError` if the file does not contain exactly one recording.

    Parameters
    ----------
    path_to_rrd : str | os.PathLike
        The path to the file to load.

    Returns
    -------
    Recording
        The loaded recording.

    """

def load_archive(path_to_rrd: str | os.PathLike) -> RRDArchive:
    """
    Load a rerun archive from an RRD file.

    Parameters
    ----------
    path_to_rrd : str | os.PathLike
        The path to the file to load.

    Returns
    -------
    RRDArchive
        The loaded archive.

    """

class StorageNodeClient:
    """
    A client for interfacing with a Rerun storage node.

    Required-feature: `remote`
    """

    def query_catalog(
        self,
        columns: Optional[list[str]] = None,
        recording_ids: Optional[list[str]] = None,
    ) -> pa.RecordBatchReader:
        """
        Get the metadata for recordings in the storage node.

        Parameters
        ----------
        columns : Optional[list[str]]
            The columns to fetch. If `None`, fetch all columns.
        recording_ids : Optional[list[str]]
            Fetch metadata of only specific recordings. If `None`, fetch for all.

        """

    def get_recording_schema(self, id: str) -> Schema:
        """
        Get the schema for a recording in the storage node.

        Parameters
        ----------
        id : str
            The id of the recording to get the schema for.

        Returns
        -------
        Schema
            The schema of the recording.

        """

    def register(self, entry: str, storage_url: str, metadata: Optional[TableLike] = None) -> str:
        """
        Register a recording along with some metadata.

        Parameters
        ----------
        entry : str
            Catalog entry in which to register the recording in.
        storage_url : str
            The URL to the storage location.
        metadata : Optional[Table | RecordBatch]
            A pyarrow Table or RecordBatch containing the metadata to update.
            This Table must contain only a single row.

        """

    def update_catalog(self, metadata: TableLike) -> None:
        """
        Update the catalog metadata for one or more recordings.

        The updates are provided as a pyarrow Table or RecordBatch containing the metadata to update.
        The Table must contain an 'id' column, which is used to specify the recording to update for each row.

        Parameters
        ----------
        metadata : Table | RecordBatch
            A pyarrow Table or RecordBatch containing the metadata to update.

        """

    def open_recording(self, id: str) -> Recording:
        """
        Open a [`Recording`][rerun.dataframe.Recording] by id to use with the dataframe APIs.

        This will run queries against the remote storage node and stream the results. Faster for small
        numbers of queries with small results.

        Parameters
        ----------
        id : str
            The id of the recording to open.

        Returns
        -------
        Recording
            The opened recording.

        """

    def download_recording(self, id: str) -> Recording:
        """
        Download a [`Recording`][rerun.dataframe.Recording] by id to use with the dataframe APIs.

        This will download the full recording to memory and run queries against a local chunk store.

        Parameters
        ----------
        id : str
            The id of the recording to open.

        Returns
        -------
        Recording
            The opened recording.

        """

    def create_vector_index(
        self,
        entry: str,
        column: ComponentColumnSelector,
        time_index: IndexColumnSelector,
        num_partitions: int,
        num_sub_vectors: int,
        distance_metric: VectorDistanceMetricLike,
    ) -> None:
        """
        Create a vector index.

        Parameters
        ----------
        entry : str
            The name of the catalog entry to index.
        column : ComponentColumnSelector
            The component column to index.
        time_index : IndexColumnSelector
            The index column to use for the time index.
        num_partitions : int
            The number of partitions for the index.
        num_sub_vectors : int
            The number of sub-vectors for the index.
        distance_metric : VectorDistanceMetric
            The distance metric to use for the index.

        """

    def create_fts_index(
        self,
        entry: str,
        column: ComponentColumnSelector,
        time_index: IndexColumnSelector,
        store_position: bool,
        base_tokenizer: str,
    ) -> None:
        """
        Create a full-text-search index.

        Parameters
        ----------
        entry : str
            The name of the catalog entry to index.
        column : ComponentColumnSelector
            The component column to index.
        time_index : IndexColumnSelector
            The index column to use for the time index.
        store_position : bool
            Whether to store the position of the token in the document.
        base_tokenizer : str
            The base tokenizer to use.

        """

    def search_vector_index(
        self,
        entry: str,
        query: VectorLike,
        column: ComponentColumnSelector,
        top_k: int,
    ) -> pa.RecordBatchReader:
        """
        Search over a vector index.

        Parameters
        ----------
        entry : str
            The name of the catalog entry to search.
        query : VectorLike
            The input to search for.
        column : ComponentColumnSelector
            The component column to search over.
        top_k : int
            The number of results to return.

        Returns
        -------
        pa.RecordBatchReader
            The results of the query.

        """

    def search_fts_index(
        self,
        entry: str,
        query: str,
        column: ComponentColumnSelector,
        limit: Optional[int] = None,
    ) -> pa.RecordBatchReader:
        """
        Search over a full-text-search index.

        Parameters
        ----------
        entry : str
            The name of the catalog entry to search.
        query : str
            The input to search for.
        column : ComponentColumnSelector
            The component column to search over.
        limit : Optional[int]
            The maximum number of results to return.

        Returns
        -------
        pa.RecordBatchReader
            The results of the query.

        """

def connect(addr: str) -> StorageNodeClient:
    """
    Load a rerun archive from an RRD file.

    Required-feature: `remote`

    Rerun uses it's own custom URI scheme. The following are valid
    addresses:

    * `rerun://<addr>:<port>` Defaults to a secure TLS connection.
    * `rerun+http://localhost:51234` Falls back to using HTTP only.
    * `rerun+https://localhost:51234` Same as `rerun://` but explicit.

    Parameters
    ----------
    addr : str
        The address of the storage node to connect to.

    Returns
    -------
    StorageNodeClient
        The connected client.

    """
