use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateCollectionRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub collection_uid: String,
    pub collection: Option<serde_json::Value>,
}
impl<'a> UpdateCollectionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/collections/{collection_uid}", collection_uid = self.collection_uid
                ),
            );
        if let Some(ref unwrapped) = self.collection {
            r = r.push_json(json!({ "collection" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn collection(mut self, collection: serde_json::Value) -> Self {
        self.collection = Some(collection);
        self
    }
}
