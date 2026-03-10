use super::parameters::Parameters;
use crate::TimeAdjustment;
use crate::models::ishaa_parameter::IshaaParameter;
use crate::models::rounding::Rounding;

/// Provides preset configuration for a few authorities
/// for calculating prayer times.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Method {
    /// Muslim World League. Standard Fajr time with an angle of 18°.
    /// Earlier Ishaa time with an angle of 17°.
    MuslimWorldLeague,

    /// Egyptian General Authority of Survey. Early Fajr time using an angle 19.5°
    /// and a slightly earlier Ishaa time using an angle of 17.5°.
    Egyptian,

    /// Umm al-Qura University, Makkah. Uses a fixed interval of 90 minutes
    /// from maghrib to calculate Ishaa. And a slightly earlier Fajr time with
    /// an angle of 18.5°. Note: you should add a +30 minute custom adjustment
    /// for Ishaa during Ramadan.
    UmmAlQura,

    /// Method developed by Khalid Shaukat, founder of Moonsighting Committee Worldwide.
    /// Uses standard 18° angles for Fajr and Ishaa in addition to seasonal adjustment values.
    /// This method automatically applies the 1/7 approximation rule for locations above 55°
    /// latitude. Recommended for North America and the UK.
    MoonsightingCommittee,

    /// Also known as the ISNA method. Can be used for North America,
    /// but the moonsightingCommittee method is preferable. Gives later Fajr times and early.
    /// Ishaa times with angles of 15°.
    NorthAmerica,

    /// Used in Singapore, Malaysia, and Indonesia. Early Fajr time with an angle of 20°
    /// and standard Ishaa time with an angle of 18°.
    Singapore,
}

impl Method {
    pub fn parameters(&self) -> Parameters {
        match self {
            Method::MuslimWorldLeague => Parameters {
                fajr_angle: 18.0,
                ishaa_parameter: IshaaParameter::Angle(17.0),
                method_adjustments: TimeAdjustment {
                    dhuhr: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
            Method::Egyptian => Parameters {
                fajr_angle: 19.5,
                ishaa_parameter: IshaaParameter::Angle(17.5),
                method_adjustments: TimeAdjustment {
                    dhuhr: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
            Method::UmmAlQura => Parameters {
                fajr_angle: 18.5,
                ishaa_parameter: IshaaParameter::Interval(90),
                ..Default::default()
            },
            Method::MoonsightingCommittee => Parameters {
                fajr_angle: 18.0,
                ishaa_parameter: IshaaParameter::Angle(18.0),
                is_moonsighting_committee: true,
                method_adjustments: TimeAdjustment {
                    dhuhr: 5,
                    maghrib: 3,
                    ..Default::default()
                },
                ..Default::default()
            },
            Method::NorthAmerica => Parameters {
                fajr_angle: 15.0,
                ishaa_parameter: IshaaParameter::Angle(15.0),
                method_adjustments: TimeAdjustment {
                    dhuhr: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
            Method::Singapore => Parameters {
                fajr_angle: 20.0,
                ishaa_parameter: IshaaParameter::Angle(18.0),
                rounding: Rounding::Ceil,
                method_adjustments: TimeAdjustment {
                    dhuhr: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameters_for_umm_al_qura() {
        let method = Method::UmmAlQura;
        let params = method.parameters();

        assert_eq!(params.fajr_angle, 18.5);
        assert_eq!(params.ishaa_parameter, IshaaParameter::Interval(90));
    }

    #[test]
    fn parameters_for_moonsighting_committee() {
        let method = Method::MoonsightingCommittee;
        let params = method.parameters();

        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.ishaa_parameter, IshaaParameter::Angle(18.0));
    }
}
