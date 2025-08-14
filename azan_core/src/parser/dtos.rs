use chrono::{DateTime, Datelike, TimeZone, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct RootDto {
    pub coordinates: CoordinatesDto,
    pub method: MethodDto,
    #[serde(default)]
    pub mazhab: MazhabDto,
    #[serde(default)]
    pub date: DateDto,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CoordinatesDto {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum MethodDto {
    MuslimWorldLeague,
    Egyptian,
    UmmAlQura,
    MoonsightingCommittee,
    NorthAmerica,
    Singapore,
}

#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
pub enum MazhabDto {
    Hanafi,
    #[default]
    Shafi,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct DateDto {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Default for DateDto {
    fn default() -> Self {
        let today = Utc::now();
        DateDto {
            year: today.year(),
            month: today.month(),
            day: today.day(),
        }
    }
}

impl DateDto {
    pub fn to_utc_date(&self) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(self.year, self.month, self.day, 0, 0, 0)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_date() {
        let json = r#"
        {
            "coordinates": {
                "latitude": 12.34,
                "longitude": 56.78
            },
            "method": "MuslimWorldLeague",
            "mazhab": "Hanafi",
            "date": {
                "year": 2023,
                "month": 3,
                "day": 15
            }
        }
        "#;

        let root: RootDto = serde_json::from_str(json).unwrap();

        assert_eq!(root.coordinates.latitude, 12.34);
        assert_eq!(root.coordinates.longitude, 56.78);
        assert_eq!(root.method, MethodDto::MuslimWorldLeague);
        assert_eq!(root.mazhab, MazhabDto::Hanafi);
        assert_eq!(root.date.year, 2023);
        assert_eq!(root.date.month, 3);
        assert_eq!(root.date.day, 15);
    }

    #[test]
    fn test_default_fields() {
        let json = r#"
        {
            "coordinates": {
                "latitude": 12.34,
                "longitude": 56.78
            },
            "method": "MuslimWorldLeague"
        }
        "#;

        let root: RootDto = serde_json::from_str(json).unwrap();
        let date = DateDto::default();

        assert_eq!(root.coordinates.latitude, 12.34);
        assert_eq!(root.coordinates.longitude, 56.78);
        assert_eq!(root.method, MethodDto::MuslimWorldLeague);
        assert_eq!(root.mazhab, MazhabDto::Shafi);
        assert_eq!(root.date.year, date.year);
        assert_eq!(root.date.month, date.month);
        assert_eq!(root.date.day, date.day);
    }
}
