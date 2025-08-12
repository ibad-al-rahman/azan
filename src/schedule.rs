//! # Prayer Schedule
//!
//! This module provides the main objects that are used for calculating
//! the prayer times.

use crate::astronomy::ops;
use crate::astronomy::solar::SolarTime;
use crate::astronomy::unit::Angle;
use crate::astronomy::unit::Coordinates;
use crate::astronomy::unit::Stride;
use crate::models::ishaa_parameter::IshaaParameter;
use crate::models::method::Method;
use crate::models::parameters::Parameters;
use crate::models::prayer::Prayer;
use crate::models::rounding::Rounding;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Duration;
use chrono::NaiveDate;
use chrono::Utc;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct PrayerTimes {
    fajr: DateTime<Utc>,
    sunrise: DateTime<Utc>,
    dhuhr: DateTime<Utc>,
    asr: DateTime<Utc>,
    maghrib: DateTime<Utc>,
    ishaa: DateTime<Utc>,
    middle_of_the_night: DateTime<Utc>,
    qiyam: DateTime<Utc>,
    fajr_tomorrow: DateTime<Utc>,
    coordinates: Coordinates,
    date: DateTime<Utc>,
    parameters: Parameters,
}

impl PrayerTimes {
    pub fn new(date: NaiveDate, coordinates: Coordinates, parameters: Parameters) -> PrayerTimes {
        let prayer_date = date
            .and_hms_opt(0, 0, 0)
            .expect("Invalid date provided")
            .and_utc();
        let tomorrow = prayer_date.tomorrow();
        let solar_time = SolarTime::new(prayer_date, coordinates);
        let solar_time_tomorrow = SolarTime::new(tomorrow, coordinates);

        let asr = solar_time.afternoon(parameters.madhab.shadow().into());
        let night = solar_time_tomorrow
            .sunrise
            .signed_duration_since(solar_time.sunset);

        let final_fajr =
            PrayerTimes::calculate_fajr(parameters, solar_time, night, coordinates, prayer_date)
                .rounded_minute(parameters.rounding);
        let final_sunrise = solar_time
            .sunrise
            .adjust_time(parameters.time_adjustments(Prayer::Sunrise))
            .rounded_minute(parameters.rounding);
        let final_dhuhr = solar_time
            .transit
            .adjust_time(parameters.time_adjustments(Prayer::Dhuhr))
            .rounded_minute(parameters.rounding);
        let final_asr = asr
            .adjust_time(parameters.time_adjustments(Prayer::Asr))
            .rounded_minute(parameters.rounding);
        let final_maghrib = ops::adjust_time(
            &solar_time.sunset,
            parameters.time_adjustments(Prayer::Maghrib),
        )
        .rounded_minute(parameters.rounding);
        let final_isha =
            PrayerTimes::calculate_isha(parameters, solar_time, night, coordinates, prayer_date)
                .rounded_minute(parameters.rounding);

        // Calculate the middle of the night and qiyam times
        let (final_middle_of_night, final_qiyam, final_fajr_tomorrow) =
            PrayerTimes::calculate_qiyam(
                final_maghrib,
                parameters,
                solar_time_tomorrow,
                coordinates,
                tomorrow,
            );

        PrayerTimes {
            fajr: final_fajr,
            sunrise: final_sunrise,
            dhuhr: final_dhuhr,
            asr: final_asr,
            maghrib: final_maghrib,
            ishaa: final_isha,
            middle_of_the_night: final_middle_of_night,
            qiyam: final_qiyam,
            fajr_tomorrow: final_fajr_tomorrow,
            coordinates: coordinates,
            date: prayer_date,
            parameters: parameters,
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
            Prayer::Qiyam => self.qiyam,
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
            Prayer::Ishaa => Prayer::Qiyam,
            Prayer::Qiyam => Prayer::FajrTomorrow,
            _ => Prayer::FajrTomorrow,
        }
    }

    pub fn time_remaining(&self) -> (u32, u32) {
        let next_time = self.time(self.next());
        let now = Utc::now();
        let now_to_next = next_time.signed_duration_since(now).num_seconds() as f64;
        let whole: f64 = now_to_next / 60.0 / 60.0;
        let fract = whole.fract();
        let hours = whole.trunc() as u32;
        let minutes = (fract * 60.0).round() as u32;

        (hours, minutes)
    }

    fn current_time(&self, time: DateTime<Utc>) -> Option<Prayer> {
        let current_prayer: Option<Prayer>;

        if self.fajr_tomorrow.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::FajrTomorrow)
        } else if self.qiyam.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Qiyam)
        } else if self.ishaa.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Ishaa);
        } else if self.maghrib.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Maghrib);
        } else if self.asr.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Asr);
        } else if self.dhuhr.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Dhuhr);
        } else if self.sunrise.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Sunrise);
        } else if self.fajr.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Fajr);
        } else {
            current_prayer = None;
        }

        current_prayer
    }

    fn calculate_fajr(
        parameters: Parameters,
        solar_time: SolarTime,
        night: Duration,
        coordinates: Coordinates,
        prayer_date: DateTime<Utc>,
    ) -> DateTime<Utc> {
        let mut fajr = solar_time.time_for_solar_angle(Angle::new(-parameters.fajr_angle), false);

        // special case for moonsighting committee above latitude 55
        if parameters.is_moonsighting_committee && coordinates.latitude >= 55.0 {
            let night_fraction = night.num_seconds() / 7;
            fajr = solar_time
                .sunrise
                .checked_add_signed(Duration::seconds(-night_fraction))
                .unwrap();
        } else {
            // Nothing to do.
        }

        let safe_fajr = if parameters.is_moonsighting_committee {
            let day_of_year = prayer_date.ordinal();
            ops::season_adjusted_morning_twilight(
                coordinates.latitude,
                day_of_year,
                prayer_date.year() as u32,
                solar_time.sunrise,
            )
        } else {
            let portion = parameters.night_portions().0;
            let night_fraction = portion * (night.num_seconds() as f64);

            solar_time
                .sunrise
                .checked_add_signed(Duration::seconds(-night_fraction as i64))
                .unwrap()
        };

        if fajr < safe_fajr {
            fajr = safe_fajr;
        } else {
            // Nothing to do.
        }

        fajr.adjust_time(parameters.time_adjustments(Prayer::Fajr))
    }

    fn calculate_isha(
        parameters: Parameters,
        solar_time: SolarTime,
        night: Duration,
        coordinates: Coordinates,
        prayer_date: DateTime<Utc>,
    ) -> DateTime<Utc> {
        let mut ishaa: DateTime<Utc>;

        match parameters.ishaa_parameter {
            IshaaParameter::Interval(interval) => {
                ishaa = solar_time
                    .sunset
                    .checked_add_signed(Duration::seconds((interval * 60) as i64))
                    .unwrap();
            }
            IshaaParameter::Angle(angle) => {
                ishaa = solar_time.time_for_solar_angle(Angle::new(-angle), true);

                // special case for moonsighting committee above latitude 55
                if parameters.is_moonsighting_committee && coordinates.latitude >= 55.0 {
                    let night_fraction = night.num_seconds() / 7;
                    ishaa = solar_time
                        .sunset
                        .checked_add_signed(Duration::seconds(night_fraction))
                        .unwrap();
                } else {
                    // Nothing to do.
                }

                let safe_isha = if parameters.is_moonsighting_committee {
                    let day_of_year = prayer_date.ordinal();

                    ops::season_adjusted_evening_twilight(
                        coordinates.latitude,
                        day_of_year,
                        prayer_date.year() as u32,
                        solar_time.sunset,
                        parameters.twilight,
                    )
                } else {
                    let portion = parameters.night_portions().1;
                    let night_fraction = portion * (night.num_seconds() as f64);

                    solar_time
                        .sunset
                        .checked_add_signed(Duration::seconds(night_fraction as i64))
                        .unwrap()
                };

                if ishaa > safe_isha {
                    ishaa = safe_isha;
                } else {
                    // Nothing to do.
                }
            }
        }

        ishaa.adjust_time(parameters.time_adjustments(Prayer::Ishaa))
    }

    fn calculate_qiyam(
        current_maghrib: DateTime<Utc>,
        parameters: Parameters,
        solar_time: SolarTime,
        coordinates: Coordinates,
        prayer_date: DateTime<Utc>,
    ) -> (DateTime<Utc>, DateTime<Utc>, DateTime<Utc>) {
        let tomorrow = prayer_date.tomorrow();
        let solar_time_tomorrow = SolarTime::new(tomorrow, coordinates);
        let night = solar_time_tomorrow
            .sunrise
            .signed_duration_since(solar_time.sunset);

        let tomorrow_fajr =
            PrayerTimes::calculate_fajr(parameters, solar_time, night, coordinates, prayer_date);
        let night_duration = tomorrow_fajr
            .signed_duration_since(current_maghrib)
            .num_seconds() as f64;
        let middle_night_portion = (night_duration / 2.0) as i64;
        let last_third_portion = (night_duration * (2.0 / 3.0)) as i64;
        let middle_of_night = current_maghrib
            .checked_add_signed(Duration::seconds(middle_night_portion))
            .unwrap()
            .rounded_minute(Rounding::Nearest);
        let last_third_of_night = current_maghrib
            .checked_add_signed(Duration::seconds(last_third_portion))
            .unwrap()
            .rounded_minute(Rounding::Nearest);

        (middle_of_night, last_third_of_night, tomorrow_fajr)
    }
}

/// A builder for the [PrayerTimes](struct.PrayerTimes.html) struct.
pub struct PrayerSchedule {
    date: Option<NaiveDate>,
    coordinates: Option<Coordinates>,
    params: Option<Parameters>,
}

impl PrayerSchedule {
    pub fn new() -> PrayerSchedule {
        PrayerSchedule {
            date: None,
            coordinates: None,
            params: None,
        }
    }

    pub fn on<'a>(&'a mut self, date: NaiveDate) -> &'a mut PrayerSchedule {
        self.date = Some(date);
        self
    }

    pub fn for_location<'a>(&'a mut self, location: Coordinates) -> &'a mut PrayerSchedule {
        self.coordinates = Some(location);
        self
    }

    pub fn with_configuration<'a>(&'a mut self, params: Parameters) -> &'a mut PrayerSchedule {
        self.params = Some(params);
        self
    }

    pub fn calculate(&self) -> Result<PrayerTimes, String> {
        if self.date.is_some() && self.coordinates.is_some() && self.params.is_some() {
            Ok(PrayerTimes::new(
                self.date.unwrap(),
                self.coordinates.unwrap(),
                self.params.unwrap(),
            ))
        } else {
            Err(String::from(
                "Required information is needed in order to calculate the prayer times.",
            ))
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     // use crate::Configuration;
//     use crate::models::mazhab::Mazhab;
//     use chrono::{NaiveDate, TimeZone, Utc};

//     #[test]
//     fn current_prayer_should_be_fajr() {
//         // Given the above DateTime, the Fajr prayer is at 2015-07-12T08:42:00Z
//         let local_date = NaiveDate::from_ymd_opt(2015, 7, 12).expect("Invalid date provided");
//         let params = Configuration::with(Method::NorthAmerica, Mazhab::Hanafi);
//         let coordinates = Coordinates::new(35.7750, -78.6336);
//         let times = PrayerTimes::new(local_date, coordinates, params);
//         let current_prayer_time = local_date.and_hms_opt(9, 0, 0).unwrap().and_utc();

//         assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Fajr));
//     }

//     #[test]
//     fn current_prayer_should_be_sunrise() {
//         // Given the below DateTime, sunrise is at 2015-07-12T10:08:00Z
//         let local_date = NaiveDate::from_ymd_opt(2015, 7, 12).expect("Invalid date provided");
//         let params = Configuration::with(Method::NorthAmerica, Mazhab::Hanafi);
//         let coordinates = Coordinates::new(35.7750, -78.6336);
//         let times = PrayerTimes::new(local_date, coordinates, params);
//         let current_prayer_time = local_date.and_hms_opt(11, 0, 0).unwrap().and_utc();

//         assert_eq!(
//             times.current_time(current_prayer_time),
//             Some(Prayer::Sunrise)
//         );
//     }

//     #[test]
//     fn current_prayer_should_be_dhuhr() {
//         // Given the above DateTime, dhuhr prayer is at 2015-07-12T17:21:00Z
//         let local_date = NaiveDate::from_ymd_opt(2015, 7, 12).expect("Invalid date provided");
//         let params = Configuration::with(Method::NorthAmerica, Mazhab::Hanafi);
//         let coordinates = Coordinates::new(35.7750, -78.6336);
//         let times = PrayerTimes::new(local_date, coordinates, params);
//         let current_prayer_time = local_date.and_hms_opt(19, 0, 0).unwrap().and_utc();

//         assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Dhuhr));
//     }

//     #[test]
//     fn current_prayer_should_be_asr() {
//         // Given the below DateTime, asr is at 2015-07-12T22:22:00Z
//         let local_date = NaiveDate::from_ymd_opt(2015, 7, 12).expect("Invalid date provided");
//         let params = Configuration::with(Method::NorthAmerica, Mazhab::Hanafi);
//         let coordinates = Coordinates::new(35.7750, -78.6336);
//         let times = PrayerTimes::new(local_date, coordinates, params);
//         let current_prayer_time = local_date.and_hms_opt(22, 26, 0).unwrap().and_utc();

//         assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Asr));
//     }

//     #[test]
//     fn current_prayer_should_be_maghrib() {
//         // Given the below DateTime, maghrib is at 2015-07-13T00:32:00Z
//         let local_date = NaiveDate::from_ymd_opt(2015, 7, 12).expect("Invalid data provided");
//         let params = Configuration::with(Method::NorthAmerica, Mazhab::Hanafi);
//         let coordinates = Coordinates::new(35.7750, -78.6336);
//         let times = PrayerTimes::new(local_date, coordinates, params);
//         let current_prayer_time = Utc.with_ymd_and_hms(2015, 7, 13, 01, 0, 0).unwrap();

//         assert_eq!(
//             times.current_time(current_prayer_time),
//             Some(Prayer::Maghrib)
//         );
//     }

//     #[test]
//     fn current_prayer_should_be_ishaa() {
//         // Given the below DateTime, ishaa is at 2015-07-13T01:57:00Z
//         let local_date = NaiveDate::from_ymd_opt(2015, 7, 12).expect("Invalid date provided");
//         let params = Configuration::with(Method::NorthAmerica, Mazhab::Hanafi);
//         let coordinates = Coordinates::new(35.7750, -78.6336);
//         let times = PrayerTimes::new(local_date, coordinates, params);
//         let current_prayer_time = Utc.with_ymd_and_hms(2015, 7, 13, 02, 0, 0).unwrap();

//         assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Ishaa));
//     }

//     #[test]
//     fn current_prayer_should_be_none() {
//         let local_date = NaiveDate::from_ymd_opt(2015, 7, 12).expect("Invalid data provided");
//         let params = Configuration::with(Method::NorthAmerica, Mazhab::Hanafi);
//         let coordinates = Coordinates::new(35.7750, -78.6336);
//         let times = PrayerTimes::new(local_date, coordinates, params);
//         let current_prayer_time = local_date.and_hms_opt(8, 0, 0).unwrap().and_utc();

//         assert_eq!(times.current_time(current_prayer_time), None);
//     }

//     #[test]
//     fn calculate_times_for_moonsighting_method() {
//         let date = NaiveDate::from_ymd_opt(2016, 1, 31).expect("Invalid date provided");
//         let params = Configuration::with(Method::MoonsightingCommittee, Mazhab::Shafi);
//         let coordinates = Coordinates::new(35.7750, -78.6336);
//         let result = PrayerSchedule::new()
//             .on(date)
//             .for_location(coordinates)
//             .with_configuration(params)
//             .calculate();

//         match result {
//             Ok(schedule) => {
//                 // fajr    = 2016-01-31 10:48:00 UTC
//                 // sunrise = 2016-01-31 12:16:00 UTC
//                 // dhuhr   = 2016-01-31 17:33:00 UTC
//                 // asr     = 2016-01-31 20:20:00 UTC
//                 // maghrib = 2016-01-31 22:43:00 UTC
//                 // ishaa    = 2016-02-01 00:05:00 UTC
//                 assert_eq!(
//                     schedule.time(Prayer::Fajr).format("%-l:%M %p").to_string(),
//                     "10:48 AM"
//                 );
//                 assert_eq!(
//                     schedule
//                         .time(Prayer::Sunrise)
//                         .format("%-l:%M %p")
//                         .to_string(),
//                     "12:16 PM"
//                 );
//                 assert_eq!(
//                     schedule.time(Prayer::Dhuhr).format("%-l:%M %p").to_string(),
//                     "5:33 PM"
//                 );
//                 assert_eq!(
//                     schedule.time(Prayer::Asr).format("%-l:%M %p").to_string(),
//                     "8:20 PM"
//                 );
//                 assert_eq!(
//                     schedule
//                         .time(Prayer::Maghrib)
//                         .format("%-l:%M %p")
//                         .to_string(),
//                     "10:43 PM"
//                 );
//                 assert_eq!(
//                     schedule.time(Prayer::Ishaa).format("%-l:%M %p").to_string(),
//                     "12:05 AM"
//                 );
//             }

//             Err(_err) => assert!(false),
//         }
//     }

//     #[test]
//     fn calculate_times_for_moonsighting_method_with_high_latitude() {
//         let date = NaiveDate::from_ymd_opt(2016, 1, 1).expect("Invalid date provided");
//         let params = Configuration::with(Method::MoonsightingCommittee, Mazhab::Hanafi);
//         let coordinates = Coordinates::new(59.9094, 10.7349);
//         let result = PrayerSchedule::new()
//             .on(date)
//             .for_location(coordinates)
//             .with_configuration(params)
//             .calculate();

//         match result {
//             Ok(schedule) => {
//                 // fajr    = 2016-01-01 06:34:00 UTC
//                 // sunrise = 2016-01-01 08:19:00 UTC
//                 // dhuhr   = 2016-01-01 11:25:00 UTC
//                 // asr     = 2016-01-01 12:36:00 UTC
//                 // maghrib = 2016-01-01 14:25:00 UTC
//                 // ishaa    = 2016-01-01 16:02:00 UTC
//                 assert_eq!(
//                     schedule.time(Prayer::Fajr).format("%-l:%M %p").to_string(),
//                     "6:34 AM"
//                 );
//                 assert_eq!(
//                     schedule
//                         .time(Prayer::Sunrise)
//                         .format("%-l:%M %p")
//                         .to_string(),
//                     "8:19 AM"
//                 );
//                 assert_eq!(
//                     schedule.time(Prayer::Dhuhr).format("%-l:%M %p").to_string(),
//                     "11:25 AM"
//                 );
//                 assert_eq!(
//                     schedule.time(Prayer::Asr).format("%-l:%M %p").to_string(),
//                     "12:36 PM"
//                 );
//                 assert_eq!(
//                     schedule
//                         .time(Prayer::Maghrib)
//                         .format("%-l:%M %p")
//                         .to_string(),
//                     "2:25 PM"
//                 );
//                 assert_eq!(
//                     schedule.time(Prayer::Ishaa).format("%-l:%M %p").to_string(),
//                     "4:02 PM"
//                 );
//             }

//             Err(_err) => assert!(false),
//         }
//     }
// }
