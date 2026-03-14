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

/// An occurrence of an Islamic event with its associated Hijri date and Gregorian timestamp.
#[derive(uniffi::Record)]
pub struct IslamicEventOccurrence {
    pub event: IslamicEvent,
    pub hijri_date: HijriDate,
    pub gregorian_timestamp_secs: i64,
}

/// Returns all recurring Islamic event occurrences that fall within the given Gregorian year,
/// sorted chronologically by Gregorian date.
#[uniffi::export]
pub fn events_for_gregorian_year(gregorian_year: i32) -> Vec<IslamicEventOccurrence> {
    miqat::hijri::events::events_for_gregorian_year(gregorian_year)
        .into_iter()
        .map(|o| IslamicEventOccurrence {
            event: o.event,
            hijri_date: o.hijri_date,
            gregorian_timestamp_secs: o.gregorian_date.timestamp(),
        })
        .collect()
}

#[derive(uniffi::Object)]
pub struct HijriDateInfo {
    date: CoreHijriDate,
}

#[uniffi::export]
impl HijriDateInfo {
    #[uniffi::constructor]
    pub fn from_timestamp(timestamp_secs: i64) -> Self {
        let date = DateTime::from_timestamp_secs(timestamp_secs)
            .unwrap()
            .date_naive();
        Self {
            date: CoreHijriDate::from_gregorian(date),
        }
    }

    pub fn date(&self) -> HijriDate {
        self.date
    }

    pub fn events(&self) -> Vec<IslamicEvent> {
        self.date.events()
    }

    /// Converts this Hijri date back to a Unix timestamp (seconds) at midnight UTC,
    /// or `None` if the date cannot be represented.
    pub fn to_gregorian(&self) -> Option<i64> {
        self.date.to_gregorian().map(|dt| dt.timestamp())
    }
}
