use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JsonStringified {
    ///The OpenAPI definition type.
    pub type_: Option<String>,
    ///The stringified OpenAPI definition.
    pub input: Option<String>,
}
impl std::fmt::Display for JsonStringified {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonSchema {
    ///The OpenAPI definition type.
    pub type_: Option<String>,
    ///An object that contains a valid JSON OpenAPI definition. For more information, read the [OpenAPI documentation](https://swagger.io/docs/specification/basic-structure/).
    pub input: Option<serde_json::Value>,
}
impl std::fmt::Display for JsonSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateCollectionFromSchemaRequired {
    pub api_id: String,
    pub name: String,
    pub relations: Vec<serde_json::Value>,
    pub schema_id: String,
    pub api_version_id: String,
}
impl std::fmt::Display for CreateCollectionFromSchemaRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScimUserResource {
    ///A list of schema resource URIs.
    pub schemas: Option<Vec<String>>,
    ///If true, the team member is active.
    pub active: Option<bool>,
    ///The team member's external ID.
    pub external_id: Option<String>,
    pub meta: Option<serde_json::Value>,
    ///The team member's SCIM ID.
    pub id: Option<String>,
    ///Information about the Postman team member.
    pub name: Option<serde_json::Value>,
    ///The team member's SCIM username.
    pub user_name: Option<String>,
}
impl std::fmt::Display for ScimUserResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImportExportFile {
    ///The `file` type value.
    pub type_: String,
    ///A file containing a valid user's export .zip file.
    pub input: String,
}
impl std::fmt::Display for ImportExportFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SyncRelationsWithSchemaRequired {
    pub api_version_id: String,
    pub relation_type: String,
    pub api_id: String,
    pub entity_id: String,
}
impl std::fmt::Display for SyncRelationsWithSchemaRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
