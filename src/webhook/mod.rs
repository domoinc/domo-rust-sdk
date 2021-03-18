pub mod buzz;
pub mod dataset;
pub mod integration;

pub struct Client {}

/// Client initialization and helper methods
impl Client {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
