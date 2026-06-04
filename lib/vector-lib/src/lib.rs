pub use vector_buffers as buffers;
#[cfg(feature = "test")]
pub use vector_common::event_test_util;
pub use vector_common::{
    Error, NamedInternalEvent, Result, TimeZone, assert_event_data_eq, atomic, btreemap,
    byte_size_of, byte_size_of::ByteSizeOf, conversion, encode_logfmt, finalization, finalizer, id,
    impl_event_data_eq, internal_event, json_size, registered_event, request_metadata,
    sensitive_string, shutdown, stats, trigger,
};
pub use vector_config as configurable;
pub use vector_config::impl_generate_config_from_default;
#[cfg(feature = "vrl")]
pub use vector_core::compile_vrl;
pub use vector_core::{
    EstimatedJsonEncodedSizeOf, buckets, default_data_dir, emit, event, fanout, ipallowlist,
    metric_tags, metrics, partition, quantiles, register, samples, schema, serde, sink, source,
    source_sender, tcp, tls, transform,
};
pub use vector_lookup as lookup;
#[cfg(feature = "vrl")]
pub use vrl;

// Re-export the sub-crates as modules so downstream consumers can reach
// anything via `vector_lib::vector_core::*` / `vector_lib::vector_common::*`
// without taking direct dependencies on them.
pub use vector_common;
pub use vector_config;
pub use vector_core;

pub mod config {
    pub use vector_common::config::ComponentKey;
    pub use vector_core::config::{
        AcknowledgementsConfig, DataType, GlobalOptions, Input, LegacyKey, LogNamespace, LogSchema,
        MEMORY_BUFFER_DEFAULT_MAX_EVENTS, OutputId, SourceAcknowledgementsConfig, SourceOutput,
        Tags, Telemetry, TransformOutput, WildcardMatching, clone_input_definitions,
        init_log_schema, init_telemetry, log_schema, proxy, telemetry,
    };
}
