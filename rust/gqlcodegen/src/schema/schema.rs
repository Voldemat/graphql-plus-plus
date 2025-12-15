#[derive(Debug)]
pub struct Schema {
    pub server: super::server::schema::Schema,
    pub client: super::client::schema::Schema
}
