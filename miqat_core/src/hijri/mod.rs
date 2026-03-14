pub mod events;

use calendrical_calculations::gregorian::fixed_from_gregorian;
use calendrical_calculations::gregorian::gregorian_from_fixed;
use calendrical_calculations::islamic::Location;
use calendrical_calculations::islamic::fixed_from_saudi_islamic;
use calendrical_calculations::islamic::observational_islamic_from_fixed;
use calendrical_calculations::islamic::saudi_islamic_from_fixed;
use chrono::DateTime;
use chrono::Datelike;
use chrono::NaiveDate;
use chrono::Utc;
use std::fmt;

pub use events::IslamicEvent;

/// A date in the Islamic (Hijri) calendar.
///
/// Computed using the Saudi Islamic calendar, which is based on the
/// astronomical criterion used in Saudi Arabia for crescent moon sighting.
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
    /// Converts a Gregorian [`NaiveDate`] to a [`HijriDate`] using the observational
    /// Islamic calendar based on actual crescent moon visibility at the given location.
    ///
    /// Unlike [`from_gregorian`](Self::from_gregorian), this method accounts for geographic
    /// location, since crescent visibility varies by where you are.
    ///
    /// # Example
    ///
    /// ```
    /// use miqat::HijriDate;
    /// use chrono::NaiveDate;
    /// use calendrical_calculations::islamic::Location;
    ///
    /// let location = Location {
    ///     latitude: 21.4,
    ///     longitude: 39.8,
    ///     elevation: 298.0,
    ///     utc_offset: 3.0 / 24.0, // UTC+3
    /// };
    /// let date = NaiveDate::from_ymd_opt(2024, 3, 10).unwrap();
    /// let hijri = HijriDate::from_gregorian_observational(date, location);
    /// println!("{}", hijri);
    /// ```
    pub fn from_gregorian_observational(date: NaiveDate, location: Location) -> Self {
        let fixed = fixed_from_gregorian(date.year(), date.month() as u8, date.day() as u8);
        let (year, month, day) = observational_islamic_from_fixed(fixed, location);
        Self { year, month, day }
    }

    /// Converts a Gregorian [`NaiveDate`] to a [`HijriDate`] using the Saudi Islamic calendar,
    /// which is based on the astronomical criterion used in Saudi Arabia.
    pub fn from_gregorian(date: NaiveDate) -> Self {
        let fixed = fixed_from_gregorian(date.year(), date.month() as u8, date.day() as u8);
        let (year, month, day) = saudi_islamic_from_fixed(fixed);
        Self { year, month, day }
    }

    /// Converts this [`HijriDate`] back to a [`DateTime<Utc>`] at midnight UTC.
    pub fn to_gregorian(&self) -> Option<DateTime<Utc>> {
        let fixed = fixed_from_saudi_islamic(self.year, self.month, self.day);
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
