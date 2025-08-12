use super::adjustments::TimeAdjustment;
use super::high_altitude_rule::HighLatitudeRule;
use super::mazhab::Mazhab;
use super::method::Method;
use super::prayer::Prayer;
use super::rounding::Rounding;
use super::twilight::Twilight;

/// Settings that are used for determining the
/// the correct prayer time.
///
/// It is recommended to use [Configuration](struct.Configuration.html) to build
/// the parameters that are need.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Parameters {
    pub method: Method,
    pub fajr_angle: f64,
    pub maghrib_angle: f64,
    pub ishaa_angle: f64,
    pub ishaa_interval: i32,
    pub madhab: Mazhab,
    pub high_latitude_rule: HighLatitudeRule,
    pub adjustments: TimeAdjustment,
    pub method_adjustments: TimeAdjustment,
    pub rounding: Rounding,
    pub twilight: Twilight,
}

impl Parameters {
    pub fn new(fajr_angle: f64, ishaa_angle: f64) -> Parameters {
        Parameters {
            fajr_angle: fajr_angle,
            maghrib_angle: 0.0,
            ishaa_angle: ishaa_angle,
            method: Method::Other,
            ishaa_interval: 0,
            madhab: Mazhab::Shafi,
            high_latitude_rule: HighLatitudeRule::MiddleOfTheNight,
            adjustments: TimeAdjustment::default(),
            method_adjustments: TimeAdjustment::default(),
            rounding: Rounding::Nearest,
            twilight: Twilight::General,
        }
    }

    pub fn night_portions(&self) -> (f64, f64) {
        match self.high_latitude_rule {
            HighLatitudeRule::MiddleOfTheNight => (1.0 / 2.0, 1.0 / 2.0),
            HighLatitudeRule::SeventhOfTheNight => (1.0 / 7.0, 1.0 / 7.0),
            HighLatitudeRule::TwilightAngle => (self.fajr_angle / 60.0, self.ishaa_angle / 60.0),
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
}

/// A builder for the the [Parameters](struct.Parameters.html).
///
/// It is recommended that this is used for setting
/// all parameters that are needed.
pub struct Configuration {
    method: Method,
    fajr_angle: f64,
    maghrib_angle: f64,
    ishaa_angle: f64,
    ishaa_interval: i32,
    madhab: Mazhab,
    high_latitude_rule: HighLatitudeRule,
    adjustments: TimeAdjustment,
    method_adjustments: TimeAdjustment,
    rounding: Rounding,
    twilight: Twilight,
}

impl Configuration {
    pub fn new(fajr_angle: f64, ishaa_angle: f64) -> Configuration {
        Configuration {
            fajr_angle: fajr_angle,
            maghrib_angle: 0.0,
            ishaa_angle: ishaa_angle,
            method: Method::Other,
            ishaa_interval: 0,
            madhab: Mazhab::Shafi,
            high_latitude_rule: HighLatitudeRule::MiddleOfTheNight,
            adjustments: TimeAdjustment::default(),
            method_adjustments: TimeAdjustment::default(),
            rounding: Rounding::Nearest,
            twilight: Twilight::General,
        }
    }

    pub fn with(method: Method, madhab: Mazhab) -> Parameters {
        let mut params = method.parameters();
        params.madhab = madhab;

        params
    }

    pub fn method<'a>(&'a mut self, method: Method) -> &'a mut Configuration {
        self.method = method;
        self
    }

    pub fn method_adjustments<'a>(
        &'a mut self,
        method_adjustments: TimeAdjustment,
    ) -> &'a mut Configuration {
        self.method_adjustments = method_adjustments;
        self
    }

    pub fn high_latitude_rule<'a>(
        &'a mut self,
        high_latitude_rule: HighLatitudeRule,
    ) -> &'a mut Configuration {
        self.high_latitude_rule = high_latitude_rule;
        self
    }

    pub fn madhab<'a>(&'a mut self, madhab: Mazhab) -> &'a mut Configuration {
        self.madhab = madhab;
        self
    }

    pub fn ishaa_interval<'a>(&'a mut self, ishaa_interval: i32) -> &'a mut Configuration {
        self.ishaa_angle = 0.0;
        self.ishaa_interval = ishaa_interval;
        self
    }

    pub fn maghrib_angle<'a>(&'a mut self, angle: f64) -> &'a mut Configuration {
        self.maghrib_angle = angle;
        self
    }

    pub fn rounding<'a>(&'a mut self, value: Rounding) -> &'a mut Configuration {
        self.rounding = value;
        self
    }

    pub fn twilight<'a>(&'a mut self, value: Twilight) -> &'a mut Configuration {
        self.twilight = value;
        self
    }

    pub fn done(&self) -> Parameters {
        Parameters {
            fajr_angle: self.fajr_angle,
            maghrib_angle: self.maghrib_angle,
            ishaa_angle: self.ishaa_angle,
            method: self.method,
            ishaa_interval: self.ishaa_interval,
            madhab: self.madhab,
            high_latitude_rule: self.high_latitude_rule,
            adjustments: self.adjustments,
            method_adjustments: self.method_adjustments,
            rounding: self.rounding,
            twilight: self.twilight,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_parameters_with_fajr_and_ishaa_angles() {
        let params = Parameters::new(18.0, 18.0);

        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.ishaa_angle, 18.0);
        assert_eq!(params.ishaa_interval, 0);
    }

    #[test]
    fn calculated_night_portions_middle_of_the_night() {
        let params = Parameters::new(18.0, 18.0);

        assert_eq!(params.night_portions().0, 1.0 / 2.0);
        assert_eq!(params.night_portions().1, 1.0 / 2.0);
    }

    #[test]
    fn calculated_night_portions_seventh_of_the_night() {
        let params = Configuration::new(18.0, 18.0)
            .high_latitude_rule(HighLatitudeRule::SeventhOfTheNight)
            .done();

        assert_eq!(params.night_portions().0, 1.0 / 7.0);
        assert_eq!(params.night_portions().1, 1.0 / 7.0);
    }

    #[test]
    fn calculated_night_portions_twilight_angle() {
        let params = Configuration::new(10.0, 15.0)
            .high_latitude_rule(HighLatitudeRule::TwilightAngle)
            .done();

        assert_eq!(params.night_portions().0, 10.0 / 60.0);
        assert_eq!(params.night_portions().1, 15.0 / 60.0);
    }

    #[test]
    fn parameters_using_method_and_madhab() {
        let params = Configuration::with(Method::NorthAmerica, Mazhab::Hanafi);

        assert_eq!(params.method, Method::NorthAmerica);
        assert_eq!(params.fajr_angle, 15.0);
        assert_eq!(params.ishaa_angle, 15.0);
        assert_eq!(params.ishaa_interval, 0);
        assert_eq!(params.madhab, Mazhab::Hanafi);
    }
}
