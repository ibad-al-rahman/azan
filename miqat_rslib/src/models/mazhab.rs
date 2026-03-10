pub type Mazhab = miqat::Mazhab;

#[uniffi::remote(Enum)]
pub enum Mazhab {
    Shafi,
    Hanafi,
}
