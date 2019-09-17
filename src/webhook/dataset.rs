use serde_json::Value;
use std::error::Error;

impl super::Client {
    /// A webhook dataset can be created via the product.
    /// https://{customer}.domo.com/connectors/com.domo.connector.jsonwh
    ///
    /// Json data can then be sent into the dataset
    pub fn post_dataset_json(&self, url: &str, obj: Value) -> Result<(), Box<dyn Error>> {
        self.client
            .post(&format!("{}", url))
            .json(&obj)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}
