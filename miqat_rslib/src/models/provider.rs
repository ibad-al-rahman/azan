pub type ProviderCity = miqat::ProviderCity;
pub type Provider = miqat::Provider;

#[uniffi::remote(Enum)]
pub enum ProviderCity {
    Beirut,
}

#[uniffi::remote(Enum)]
pub enum Provider {
    DarElFatwa(ProviderCity),
}
