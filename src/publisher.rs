use datadog_client::client::{Client, Error as DDError};
use datadog_client::metrics::Serie;

pub struct Publisher {
    client: Client,
}

impl Publisher {
    pub fn from_env() -> Self {
        let client = Client::new(
            std::env::var("DD_HOST").unwrap_or_else(|_| String::from("https://api.datadoghq.eu")),
            std::env::var("DD_API_KEY").unwrap(),
        );
        Self { client }
    }

    pub async fn send(&self, series: Vec<Serie>) -> Result<(), String> {
        self.client
            .post_metrics(&series)
            .await
            .map_err(|err| match err {
                DDError::Reqwest(req) => format!("error while fetching: {:?}", req),
                DDError::Body(_, body) => format!("error with the data: {:?}", body),
            })?;
        Ok(())
    }
}
