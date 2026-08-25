#[cfg(any(
    feature = "json-serializers-schema-client",
    feature = "json-serializers-schema-server",
    feature = "json-serializers-schema-shared",
))]
pub mod schema;
