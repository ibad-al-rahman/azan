use chrono::DateTime;
use miqat::HijriDate as CoreHijriDate;

pub type IslamicEvent = miqat::IslamicEvent;

#[uniffi::remote(Enum)]
pub enum IslamicEvent {
    IslamicNewYear,
    Ashura,
    MawlidAlNabi,
    BattleOfHattin,
    BattleOfMutah,
    BattleOfTabuk,
    IsraAndMiraj,
    NisfShaban,
    FirstOfRamadan,
    BattleOfBadr,
    ConquestOfMecca,
    LaylatAlQadr,
    EidAlFitr,
    BattleOfUhud,
    DayOfArafah,
    EidAlAdha,
}

pub type HijriDate = miqat::HijriDate;

#[uniffi::remote(Record)]
pub struct HijriDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

#[uniffi::export]
pub fn hijri_date_from_timestamp(timestamp_secs: i64) -> HijriDate {
    let date = DateTime::from_timestamp_secs(timestamp_secs)
        .unwrap()
        .date_naive();
    CoreHijriDate::from_gregorian(date)
}

#[uniffi::export]
pub fn hijri_date_events(timestamp_secs: i64) -> Vec<IslamicEvent> {
    let date = DateTime::from_timestamp_secs(timestamp_secs)
        .unwrap()
        .date_naive();
    CoreHijriDate::from_gregorian(date).events()
}
