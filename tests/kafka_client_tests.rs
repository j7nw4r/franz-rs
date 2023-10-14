use anyhow::Context;
use kafka_client_rs::KafkaClientBuilder;
#[test]
fn kafka_client_builds() -> anyhow::Result<()> {
    let testing_client_id = "testing-id".to_string();
    let host = "test-hosts".to_string();

    let kafka_client = KafkaClientBuilder::default()
        .client_id(testing_client_id.clone())
        .hosts(vec![host.clone()])
        .build().context("could not build using KafkaClientBuilder")?;

    assert_eq!(testing_client_id, *kafka_client.client_id());
    assert_eq!(kafka_client.hosts().len(), 1);
    assert_eq!(host, kafka_client.hosts()[0]);
    Ok(())
}