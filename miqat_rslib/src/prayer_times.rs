use crate::hijri::HijriDate;
use chrono::DateTime;
use miqat::Coordinates;
use miqat::Method;
use miqat::Prayer;
use miqat::Provider;

#[derive(uniffi::Object)]
pub struct PrayerTimes {
    fajr: i64,
    sunrise: i64,
    dhuhr: i64,
    asr: i64,
    maghrib: i64,
    ishaa: i64,
    fajr_tomorrow: i64,
    hijri_date: HijriDate,
    inner: miqat::PrayerTimes,
}

#[uniffi::export]
impl PrayerTimes {
    #[uniffi::constructor]
    pub fn from_method(
        date_utc_timestamp_secs: i64,
        coordinates: Coordinates,
        method: Method,
    ) -> Self {
        let date = DateTime::from_timestamp_secs(date_utc_timestamp_secs)
            .unwrap()
            .date_naive();
        let inner = miqat::PrayerTimes::computed(date, coordinates, method.parameters());
        Self::from_inner(inner, date)
    }

    #[uniffi::constructor]
    pub fn from_precomputed(date_utc_timestamp_secs: i64, provider: Provider) -> Self {
        let date = DateTime::from_timestamp_secs(date_utc_timestamp_secs)
            .unwrap()
            .date_naive();
        let inner = miqat::PrayerTimes::precomputed(date, provider);
        Self::from_inner(inner, date)
    }

    pub fn fajr(&self) -> i64 {
        self.fajr
    }

    pub fn sunrise(&self) -> i64 {
        self.sunrise
    }

    pub fn dhuhr(&self) -> i64 {
        self.dhuhr
    }

    pub fn asr(&self) -> i64 {
        self.asr
    }

    pub fn maghrib(&self) -> i64 {
        self.maghrib
    }

    pub fn ishaa(&self) -> i64 {
        self.ishaa
    }

    pub fn fajr_tomorrow(&self) -> i64 {
        self.fajr_tomorrow
    }

    pub fn current_prayer(&self) -> Prayer {
        self.inner.current()
    }

    pub fn next_prayer(&self) -> Prayer {
        self.inner.next()
    }

    pub fn hijri_date(&self) -> HijriDate {
        self.hijri_date
    }
}

impl PrayerTimes {
    fn from_inner(inner: miqat::PrayerTimes, date: chrono::NaiveDate) -> Self {
        PrayerTimes {
            fajr: inner.time(Prayer::Fajr).timestamp(),
            sunrise: inner.time(Prayer::Sunrise).timestamp(),
            dhuhr: inner.time(Prayer::Dhuhr).timestamp(),
            asr: inner.time(Prayer::Asr).timestamp(),
            maghrib: inner.time(Prayer::Maghrib).timestamp(),
            ishaa: inner.time(Prayer::Ishaa).timestamp(),
            fajr_tomorrow: inner.time(Prayer::FajrTomorrow).timestamp(),
            hijri_date: miqat::HijriDate::from_gregorian(date),
            inner,
        }
    }
}
