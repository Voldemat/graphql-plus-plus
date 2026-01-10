use crate::parsers::schema::{client, server};

pub fn serialize(
    schema: &server::schema::Schema,
    client_schema: Option<&client::schema::ClientSchema>
) -> Result<String, String> {
    let server_uses_map = client_schema.map(|schema| {
        schema.get_server_uses_map()
    });
    return Ok("".into());
}
