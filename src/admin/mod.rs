use derive_builder::Builder;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]

pub struct AdminClient {
    testing: u128
}