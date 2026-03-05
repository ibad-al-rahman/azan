mod data;
pub mod provider;

use crate::models::prayer::Prayer;
use crate::precomputed::data::dar_el_fatwa_beirut;
use crate::precomputed::provider::Provider;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Days;
use chrono::NaiveDate;
use chrono::Utc;

pub struct PrecomputedPrayerTimes {
    fajr: DateTime<Utc>,
    sunrise: DateTime<Utc>,
    dhuhr: DateTime<Utc>,
    asr: DateTime<Utc>,
    maghrib: DateTime<Utc>,
    ishaa: DateTime<Utc>,
    fajr_tomorrow: DateTime<Utc>,
}

impl PrecomputedPrayerTimes {
    pub fn new(date: NaiveDate, provider: Provider) -> Self {
        let data = match provider {
            Provider::DarElFatwa(_) => &dar_el_fatwa_beirut::DATA,
        };

        let month = date.month0() as usize;
        let day = date.day0() as usize;
        let times = data[month][day];

        let tomorrow_date = date
            .checked_add_days(Days::new(1))
            .expect("failed to get tomorrow's date");
        let tomorrow_day = tomorrow_date.day0() as usize;
        let tomorrow_month = tomorrow_date.month0() as usize;
        let tomorrow_times = data[tomorrow_month][tomorrow_day];

        let make_time = |h: u8, m: u8| -> DateTime<Utc> {
            date.and_hms_opt(h.into(), m.into(), 0)
                .expect("invalid prayer time")
                .and_utc()
        };

        PrecomputedPrayerTimes {
            fajr: make_time(times[0].0, times[0].1),
            sunrise: make_time(times[1].0, times[1].1),
            dhuhr: make_time(times[2].0, times[2].1),
            asr: make_time(times[3].0, times[3].1),
            maghrib: make_time(times[4].0, times[4].1),
            ishaa: make_time(times[5].0, times[5].1),
            fajr_tomorrow: make_time(tomorrow_times[0].0, tomorrow_times[0].1),
        }
    }

    pub fn time(&self, prayer: Prayer) -> DateTime<Utc> {
        match prayer {
            Prayer::Fajr => self.fajr,
            Prayer::Sunrise => self.sunrise,
            Prayer::Dhuhr => self.dhuhr,
            Prayer::Asr => self.asr,
            Prayer::Maghrib => self.maghrib,
            Prayer::Ishaa => self.ishaa,
            Prayer::FajrTomorrow => self.fajr_tomorrow,
        }
    }

    pub fn current(&self) -> Prayer {
        self.current_time(Utc::now()).expect("Out of bounds")
    }

    pub fn next(&self) -> Prayer {
        match self.current() {
            Prayer::Fajr => Prayer::Sunrise,
            Prayer::Sunrise => Prayer::Dhuhr,
            Prayer::Dhuhr => Prayer::Asr,
            Prayer::Asr => Prayer::Maghrib,
            Prayer::Maghrib => Prayer::Ishaa,
            Prayer::Ishaa | Prayer::FajrTomorrow => Prayer::FajrTomorrow,
        }
    }

    pub fn time_remaining(&self) -> (u32, u32) {
        let next_time = self.time(self.next());
        let now = Utc::now();
        let now_to_next = next_time.signed_duration_since(now).num_seconds() as f64;
        let whole = now_to_next / 60.0 / 60.0;
        let fract = whole.fract();
        let hours = whole.trunc() as u32;
        let minutes = (fract * 60.0).round() as u32;
        (hours, minutes)
    }

    fn current_time(&self, time: DateTime<Utc>) -> Option<Prayer> {
        if self.ishaa.signed_duration_since(time).num_seconds() <= 0 {
            Some(Prayer::Ishaa)
        } else if self.maghrib.signed_duration_since(time).num_seconds() <= 0 {
            Some(Prayer::Maghrib)
        } else if self.asr.signed_duration_since(time).num_seconds() <= 0 {
            Some(Prayer::Asr)
        } else if self.dhuhr.signed_duration_since(time).num_seconds() <= 0 {
            Some(Prayer::Dhuhr)
        } else if self.sunrise.signed_duration_since(time).num_seconds() <= 0 {
            Some(Prayer::Sunrise)
        } else if self.fajr.signed_duration_since(time).num_seconds() <= 0 {
            Some(Prayer::Fajr)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precomputed::provider::ProviderCity;
    use chrono::{TimeZone, Utc};

    fn beirut(date: NaiveDate) -> PrecomputedPrayerTimes {
        PrecomputedPrayerTimes::new(date, Provider::DarElFatwa(ProviderCity::Beirut))
    }

    #[test]
    fn jan1_fajr_is_utc_03_07() {
        let pt = beirut(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
        assert_eq!(
            pt.time(Prayer::Fajr),
            Utc.with_ymd_and_hms(2026, 1, 1, 3, 7, 0).unwrap()
        );
    }

    #[test]
    fn jan1_all_prayers() {
        let pt = beirut(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
        // Local: fajr 5:07, sunrise 6:43, dhuhr 11:41, asr 2:21, maghrib 4:45, ishaa 6:07 (UTC+2)
        assert_eq!(
            pt.time(Prayer::Fajr),
            Utc.with_ymd_and_hms(2026, 1, 1, 3, 7, 0).unwrap()
        );
        assert_eq!(
            pt.time(Prayer::Sunrise),
            Utc.with_ymd_and_hms(2026, 1, 1, 4, 43, 0).unwrap()
        );
        assert_eq!(
            pt.time(Prayer::Dhuhr),
            Utc.with_ymd_and_hms(2026, 1, 1, 9, 41, 0).unwrap()
        );
        assert_eq!(
            pt.time(Prayer::Asr),
            Utc.with_ymd_and_hms(2026, 1, 1, 12, 21, 0).unwrap()
        );
        assert_eq!(
            pt.time(Prayer::Maghrib),
            Utc.with_ymd_and_hms(2026, 1, 1, 14, 45, 0).unwrap()
        );
        assert_eq!(
            pt.time(Prayer::Ishaa),
            Utc.with_ymd_and_hms(2026, 1, 1, 16, 7, 0).unwrap()
        );
    }

    #[test]
    fn mar29_dst_transition() {
        // Mar 29: local 4:59 am, UTC+3 → UTC 1:59
        let pt = beirut(NaiveDate::from_ymd_opt(2026, 3, 29).unwrap());
        assert_eq!(
            pt.time(Prayer::Fajr),
            Utc.with_ymd_and_hms(2026, 3, 29, 1, 59, 0).unwrap()
        );
    }

    #[test]
    fn apr1_utc_offset3() {
        // Apr 1: local 4:54 am, UTC+3 → UTC 1:54
        let pt = beirut(NaiveDate::from_ymd_opt(2026, 4, 1).unwrap());
        assert_eq!(
            pt.time(Prayer::Fajr),
            Utc.with_ymd_and_hms(2026, 4, 1, 1, 54, 0).unwrap()
        );
    }

    #[test]
    fn current_prayer_fajr() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let pt = beirut(date);
        // Time between fajr (3:07) and sunrise (4:43) → current = Fajr
        let t = Utc.with_ymd_and_hms(2026, 1, 1, 3, 30, 0).unwrap();
        assert_eq!(pt.current_time(t), Some(Prayer::Fajr));
    }

    #[test]
    fn current_prayer_none_before_fajr() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let pt = beirut(date);
        let t = Utc.with_ymd_and_hms(2026, 1, 1, 2, 0, 0).unwrap();
        assert_eq!(pt.current_time(t), None);
    }

    #[test]
    fn current_prayer_ishaa() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let pt = beirut(date);
        // After ishaa (16:07 UTC) → current = Ishaa
        let t = Utc.with_ymd_and_hms(2026, 1, 1, 20, 0, 0).unwrap();
        assert_eq!(pt.current_time(t), Some(Prayer::Ishaa));
    }
}
