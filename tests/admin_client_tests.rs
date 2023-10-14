use anyhow::Context;
use kafka_client_rs::AdminClientBuilder;

#[test]
fn building_client() -> anyhow::Result<()> {
    let l = AdminClientBuilder::default()
        .testing(1u128)
        .build()
        .context("could not build AdminClient")?;

    Ok(())
}
