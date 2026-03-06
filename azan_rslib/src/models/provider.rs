pub type ProviderCity = azan::ProviderCity;
pub type Provider = azan::Provider;

#[uniffi::remote(Enum)]
pub enum ProviderCity {
    Beirut,
}

#[uniffi::remote(Enum)]
pub enum Provider {
    DarElFatwa(ProviderCity),
}
