use chrono::Datelike;
use chrono::Utc;
use chrono::Weekday;
use std::fmt::Debug;

/// Names of all obligatory prayers,
/// sunrise, and Qiyam.
#[derive(PartialEq, Copy, Clone)]
pub enum Prayer {
    Fajr,
    Sunrise,
    Dhuhr,
    Asr,
    Maghrib,
    Ishaa,
    Qiyam,
    FajrTomorrow,
}

impl Debug for Prayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prayer::Fajr | Prayer::FajrTomorrow => write!(f, "Fajr"),
            Prayer::Sunrise => write!(f, "Sunrise"),
            Prayer::Dhuhr => {
                if Utc::now().weekday() == Weekday::Fri {
                    write!(f, "Jumua")
                } else {
                    write!(f, "Dhuhr")
                }
            }
            Prayer::Asr => write!(f, "Asr"),
            Prayer::Maghrib => write!(f, "Maghrib"),
            Prayer::Ishaa => write!(f, "Ishaa"),
            Prayer::Qiyam => write!(f, "Qiyam"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prayer_name_for_fajr_en_transliteration() {
        assert_eq!(format!("{:?}", Prayer::Fajr), "Fajr");
        assert_eq!(format!("{:?}", Prayer::Sunrise), "Sunrise");

        if Utc::now().weekday() == Weekday::Fri {
            assert_eq!(format!("{:?}", Prayer::Dhuhr), "Jumua");
        } else {
            assert_eq!(format!("{:?}", Prayer::Dhuhr), "Dhuhr");
        }

        assert_eq!(format!("{:?}", Prayer::Asr), "Asr");
        assert_eq!(format!("{:?}", Prayer::Maghrib), "Maghrib");
        assert_eq!(format!("{:?}", Prayer::Ishaa), "Ishaa");
        assert_eq!(format!("{:?}", Prayer::Qiyam), "Qiyam");
    }
}
