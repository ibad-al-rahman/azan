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
}
