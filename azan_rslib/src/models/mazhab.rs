pub type Mazhab = azan::Mazhab;

#[uniffi::remote(Enum)]
pub enum Mazhab {
    Shafi,
    Hanafi,
}
