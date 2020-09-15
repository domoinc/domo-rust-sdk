use serde_json::Value;

impl super::Client {
    /// A webhook dataset can be created via the product.
    /// https://{customer}.domo.com/connectors/com.domo.connector.jsonwh
    ///
    /// Json data can then be sent into the dataset
    pub async fn post_dataset_json(&self, url: &str, obj: Value) -> Result<(), surf::Exception> {
        surf::post(&format!("{}", url)).body_json(&obj)?.await?;
        Ok(())
    }
}
