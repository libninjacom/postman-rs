use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonSchema {
    #[serde(rename = "type")]
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
    #[serde(rename = "apiId")]
    pub api_id: String,
    #[serde(rename = "schemaId")]
    pub schema_id: String,
    pub relations: Vec<serde_json::Value>,
    pub name: String,
    #[serde(rename = "apiVersionId")]
    pub api_version_id: String,
}
impl std::fmt::Display for CreateCollectionFromSchemaRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScimUserResource {
    ///If true, the team member is active.
    pub active: Option<bool>,
    ///The team member's SCIM ID.
    pub id: Option<String>,
    ///Information about the Postman team member.
    pub name: Option<serde_json::Value>,
    pub meta: Option<serde_json::Value>,
    #[serde(rename = "userName")]
    ///The team member's SCIM username.
    pub user_name: Option<String>,
    ///A list of schema resource URIs.
    pub schemas: Option<Vec<String>>,
    #[serde(rename = "externalId")]
    ///The team member's external ID.
    pub external_id: Option<String>,
}
impl std::fmt::Display for ScimUserResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImportExportFile {
    #[serde(rename = "type")]
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
    #[serde(rename = "apiVersionId")]
    pub api_version_id: String,
    #[serde(rename = "relationType")]
    pub relation_type: String,
    #[serde(rename = "apiId")]
    pub api_id: String,
    #[serde(rename = "entityId")]
    pub entity_id: String,
}
impl std::fmt::Display for SyncRelationsWithSchemaRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JsonStringified {
    ///The stringified OpenAPI definition.
    pub input: Option<String>,
    #[serde(rename = "type")]
    ///The OpenAPI definition type.
    pub type_: Option<String>,
}
impl std::fmt::Display for JsonStringified {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
