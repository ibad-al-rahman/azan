use super::adjustments::TimeAdjustment;
use super::high_altitude_rule::HighLatitudeRule;
use super::mazhab::Mazhab;
use super::prayer::Prayer;
use super::rounding::Rounding;
use super::twilight::Twilight;
use crate::models::ishaa_parameter::IshaaParameter;

/// Settings that are used for determining the
/// the correct prayer time.
///
/// It is recommended to use [Configuration](struct.Configuration.html) to build
/// the parameters that are need.
#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct Parameters {
    pub is_moonsighting_committee: bool,
    pub fajr_angle: f64,
    pub maghrib_angle: f64,
    pub ishaa_parameter: IshaaParameter,
    pub mazhab: Mazhab,
    pub high_latitude_rule: HighLatitudeRule,
    pub adjustments: TimeAdjustment,
    pub method_adjustments: TimeAdjustment,
    pub rounding: Rounding,
    pub twilight: Twilight,
}

impl Parameters {
    pub fn night_portions(&self) -> (f64, f64) {
        let ishaa_angle = match self.ishaa_parameter {
            IshaaParameter::Angle(angle) => angle,
            IshaaParameter::Interval(_) => 0.0,
        };
        match self.high_latitude_rule {
            HighLatitudeRule::MiddleOfTheNight => (1.0 / 2.0, 1.0 / 2.0),
            HighLatitudeRule::SeventhOfTheNight => (1.0 / 7.0, 1.0 / 7.0),
            HighLatitudeRule::TwilightAngle => (self.fajr_angle / 60.0, ishaa_angle / 60.0),
        }
    }

    pub fn time_adjustments(&self, prayer: Prayer) -> i64 {
        match prayer {
            Prayer::Fajr => self.adjustments.fajr + self.method_adjustments.fajr,
            Prayer::Sunrise => self.adjustments.sunrise + self.method_adjustments.sunrise,
            Prayer::Dhuhr => self.adjustments.dhuhr + self.method_adjustments.dhuhr,
            Prayer::Asr => self.adjustments.asr + self.method_adjustments.asr,
            Prayer::Maghrib => self.adjustments.maghrib + self.method_adjustments.maghrib,
            Prayer::Ishaa => self.adjustments.ishaa + self.method_adjustments.ishaa,
            _ => 0,
        }
    }

    pub fn mazhab(mut self, mazhab: Mazhab) -> Self {
        self.mazhab = mazhab;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_parameters_with_fajr_and_ishaa_angles() {
        let params = Parameters {
            fajr_angle: 18.0,
            ishaa_parameter: IshaaParameter::Angle(18.0),
            ..Default::default()
        };

        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.ishaa_parameter, IshaaParameter::Angle(18.0));
    }

    #[test]
    fn calculated_night_portions_middle_of_the_night() {
        let params = Parameters {
            fajr_angle: 18.0,
            ishaa_parameter: IshaaParameter::Angle(18.0),
            ..Default::default()
        };

        assert_eq!(params.night_portions().0, 1.0 / 2.0);
        assert_eq!(params.night_portions().1, 1.0 / 2.0);
    }

    #[test]
    fn calculated_night_portions_seventh_of_the_night() {
        let params = Parameters {
            fajr_angle: 18.0,
            ishaa_parameter: IshaaParameter::Angle(18.0),
            high_latitude_rule: HighLatitudeRule::SeventhOfTheNight,
            ..Default::default()
        };

        assert_eq!(params.night_portions().0, 1.0 / 7.0);
        assert_eq!(params.night_portions().1, 1.0 / 7.0);
    }

    #[test]
    fn calculated_night_portions_twilight_angle() {
        let params = Parameters {
            fajr_angle: 10.0,
            ishaa_parameter: IshaaParameter::Angle(15.0),
            high_latitude_rule: HighLatitudeRule::TwilightAngle,
            ..Default::default()
        };

        assert_eq!(params.night_portions().0, 10.0 / 60.0);
        assert_eq!(params.night_portions().1, 15.0 / 60.0);
    }

    #[test]
    fn parameters_using_method_and_mazhab() {
        let params = Parameters {
            fajr_angle: 15.0,
            ishaa_parameter: IshaaParameter::Angle(15.0),
            high_latitude_rule: HighLatitudeRule::SeventhOfTheNight,
            mazhab: Mazhab::Hanafi,
            ..Default::default()
        };

        assert_eq!(params.fajr_angle, 15.0);
        assert_eq!(params.ishaa_parameter, IshaaParameter::Angle(15.0));
        assert_eq!(params.mazhab, Mazhab::Hanafi);
    }
}
