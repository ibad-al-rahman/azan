pub mod events;

use calendrical_calculations::gregorian::fixed_from_gregorian;
use calendrical_calculations::gregorian::gregorian_from_fixed;
use calendrical_calculations::islamic::ISLAMIC_EPOCH_FRIDAY;
use calendrical_calculations::islamic::fixed_from_tabular_islamic;
use calendrical_calculations::islamic::tabular_islamic_from_fixed;
use chrono::DateTime;
use chrono::Datelike;
use chrono::NaiveDate;
use chrono::Utc;
use std::fmt;

pub use events::IslamicEvent;

/// A date in the Islamic (Hijri) calendar.
///
/// Computed using the tabular Islamic calendar with the Friday epoch
/// (`ISLAMIC_EPOCH_FRIDAY`), which aligns with the most widely used
/// civil Hijri calendar.
///
/// # Example
///
/// ```
/// use miqat::HijriDate;
/// use chrono::NaiveDate;
///
/// let date = NaiveDate::from_ymd_opt(2024, 3, 10).unwrap();
/// let hijri = HijriDate::from_gregorian(date);
/// println!("{}", hijri); // e.g. "29/8/1445"
///
/// for event in hijri.events() {
///     println!("Today is {event}");
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HijriDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl HijriDate {
    /// Converts a Gregorian [`NaiveDate`] to a [`HijriDate`].
    pub fn from_gregorian(date: NaiveDate) -> Self {
        let fixed = fixed_from_gregorian(date.year() as i32, date.month() as u8, date.day() as u8);
        let (year, month, day) = tabular_islamic_from_fixed(fixed, ISLAMIC_EPOCH_FRIDAY);
        Self { year, month, day }
    }

    /// Converts this [`HijriDate`] back to a [`DateTime<Utc>`] at midnight UTC.
    pub fn to_gregorian(&self) -> Option<DateTime<Utc>> {
        let fixed =
            fixed_from_tabular_islamic(self.year, self.month, self.day, ISLAMIC_EPOCH_FRIDAY);
        let (y, m, d) = gregorian_from_fixed(fixed).ok()?;
        NaiveDate::from_ymd_opt(y, m as u32, d as u32)
            .map(|nd| nd.and_hms_opt(0, 0, 0).unwrap().and_utc())
    }

    /// Returns any Islamic holidays that fall on this date.
    pub fn events(&self) -> Vec<IslamicEvent> {
        IslamicEvent::for_date(self.month, self.day)
    }
}

impl From<NaiveDate> for HijriDate {
    fn from(date: NaiveDate) -> Self {
        Self::from_gregorian(date)
    }
}

impl fmt::Display for HijriDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}/{}", self.day, self.month, self.year)
    }
}
