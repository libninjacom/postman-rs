use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ScimUserResource {
    ///A list of schema resource URIs.
    pub schemas: Option<Vec<String>>,
    ///The team member's SCIM ID.
    pub id: Option<String>,
    #[serde(rename = "userName")]
    ///The team member's SCIM username.
    pub user_name: Option<String>,
    ///Information about the Postman team member.
    pub name: Option<serde_json::Value>,
    #[serde(rename = "externalId")]
    ///The team member's external ID.
    pub external_id: Option<String>,
    ///If true, the team member is active.
    pub active: Option<bool>,
    pub meta: Option<serde_json::Value>,
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
pub struct JsonStringified {
    #[serde(rename = "type")]
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
