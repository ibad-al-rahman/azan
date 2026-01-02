use azan::Coordinates;
use azan::Method;
use azan::Prayer;
use chrono::DateTime;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct PrayerTimes {
    fajr: i64,
    sunrise: i64,
    dhuhr: i64,
    asr: i64,
    maghrib: i64,
    ishaa: i64,
    fajr_tomorrow: i64,
}

impl PrayerTimes {
    pub fn from_method(
        date_utc_timestamp: i64,
        coordinates: Coordinates,
        method: Method,
    ) -> PrayerTimes {
        let date = DateTime::from_timestamp_millis(date_utc_timestamp)
            .unwrap()
            .date_naive();

        azan::PrayerTimes::new(date, coordinates, method.parameters()).into()
    }
}

impl From<azan::PrayerTimes> for PrayerTimes {
    fn from(value: azan::PrayerTimes) -> Self {
        PrayerTimes {
            fajr: value.time(Prayer::Fajr).timestamp_millis(),
            sunrise: value.time(Prayer::Sunrise).timestamp_millis(),
            dhuhr: value.time(Prayer::Dhuhr).timestamp_millis(),
            asr: value.time(Prayer::Asr).timestamp_millis(),
            maghrib: value.time(Prayer::Maghrib).timestamp_millis(),
            ishaa: value.time(Prayer::Ishaa).timestamp_millis(),
            fajr_tomorrow: value.time(Prayer::FajrTomorrow).timestamp_millis(),
        }
    }
}
