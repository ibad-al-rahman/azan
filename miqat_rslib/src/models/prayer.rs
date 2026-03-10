pub type Prayer = miqat::Prayer;

#[uniffi::remote(Enum)]
pub enum Prayer {
    Fajr,
    Sunrise,
    Dhuhr,
    Asr,
    Maghrib,
    Ishaa,
    FajrTomorrow,
}
