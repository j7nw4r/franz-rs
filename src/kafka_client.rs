use anyhow::Context;
use derive_builder::Builder;
use derive_getters::Getters;
use crate::error::Result;

#[derive(Debug, Getters, Builder)]
pub struct KafkaClient {
    client_id: String,
    hosts: Vec<String>,
    topics: Option<Vec<String>>,
}

impl KafkaClient {
    pub async fn load_metadata(&mut self) -> Result<()> {
        let topics = self.topics.unwrap_or_default();
        self.fetch_metadata(&topics).await.context("could not fetch metadata")?;
        Ok(())
    }

    async fn fetch_metadata<T: AsRef<str>>(
        &mut self,
        topics: &[T],
    ) -> Result<()> {
        for host in self.hosts {
            tracing::debug!(host = host, "fetching metadata");
        }

        Ok(())
    }
}