use chrono::DateTime;
use miqat::HijriDate as CoreHijriDate;

pub type HijriEvent = miqat::HijriEvent;

#[uniffi::remote(Enum)]
pub enum HijriEvent {
    IslamicNewYear,
    Ashura,
    MawlidAlNabi,
    IsraAndMiraj,
    NisfShaban,
    FirstOfRamadan,
    LaylatAlQadr,
    EidAlFitr,
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
pub fn hijri_date_events(timestamp_secs: i64) -> Vec<HijriEvent> {
    let date = DateTime::from_timestamp_secs(timestamp_secs)
        .unwrap()
        .date_naive();
    CoreHijriDate::from_gregorian(date).events()
}
