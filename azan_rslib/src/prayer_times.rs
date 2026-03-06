use azan::Coordinates;
use azan::Method;
use azan::Prayer;
use azan::Provider;
use chrono::DateTime;

#[derive(uniffi::Object)]
pub struct PrayerTimes {
    fajr: i64,
    sunrise: i64,
    dhuhr: i64,
    asr: i64,
    maghrib: i64,
    ishaa: i64,
    fajr_tomorrow: i64,
    inner: azan::PrayerTimes,
}

#[uniffi::export]
impl PrayerTimes {
    #[uniffi::constructor]
    pub fn from_method(date_utc_timestamp: i64, coordinates: Coordinates, method: Method) -> Self {
        let date = DateTime::from_timestamp_millis(date_utc_timestamp)
            .unwrap()
            .date_naive();
        let inner = azan::PrayerTimes::computed(date, coordinates, method.parameters());
        Self::from_inner(inner)
    }

    #[uniffi::constructor]
    pub fn from_precomputed(date_utc_timestamp: i64, provider: Provider) -> Self {
        let date = DateTime::from_timestamp_millis(date_utc_timestamp)
            .unwrap()
            .date_naive();
        let inner = azan::PrayerTimes::precomputed(date, provider);
        Self::from_inner(inner)
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
}

impl PrayerTimes {
    fn from_inner(inner: azan::PrayerTimes) -> Self {
        PrayerTimes {
            fajr: inner.time(Prayer::Fajr).timestamp_millis(),
            sunrise: inner.time(Prayer::Sunrise).timestamp_millis(),
            dhuhr: inner.time(Prayer::Dhuhr).timestamp_millis(),
            asr: inner.time(Prayer::Asr).timestamp_millis(),
            maghrib: inner.time(Prayer::Maghrib).timestamp_millis(),
            ishaa: inner.time(Prayer::Ishaa).timestamp_millis(),
            fajr_tomorrow: inner.time(Prayer::FajrTomorrow).timestamp_millis(),
            inner,
        }
    }
}
