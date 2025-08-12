use super::parameters::Parameters;
use crate::TimeAdjustment;
use crate::models::ishaa_parameter::IshaaParameter;

/// Provides preset configuration for a few authorities
/// for calculating prayer times.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Method {
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
}

impl Method {
    pub fn parameters(&self) -> Parameters {
        match self {
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
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::models::ishaa_parameter::IshaaParameter;

//     #[test]
//     fn parameters_for_muslim_world_league() {
//         let method = Method::MuslimWorldLeague;
//         let params = method.parameters();

//         assert_eq!(params.fajr_angle, 18.0);
//         assert_eq!(params.ishaa_parameter, IshaaParameter::Angle(17.0));
//     }

//     #[test]
//     fn parameters_for_egyptian() {
//         let method = Method::Egyptian;
//         let params = method.parameters();

//         assert_eq!(params.fajr_angle, 19.5);
//         assert_eq!(params.ishaa_parameter, IshaaParameter::Angle(17.5));
//     }

//     #[test]
//     fn parameters_for_karachi() {
//         let method = Method::Karachi;
//         let params = method.parameters();

//         assert_eq!(params.fajr_angle, 18.0);
//         assert_eq!(params.ishaa_parameter, IshaaParameter::Angle(18.0));
//     }

//     #[test]
//     fn parameters_for_umm_al_qura() {
//         let method = Method::UmmAlQura;
//         let params = method.parameters();

//         assert_eq!(params.fajr_angle, 18.5);
//         assert_eq!(params.ishaa_parameter, IshaaParameter::Interval(90));
//     }

//     #[test]
//     fn parameters_for_dubai() {
//         let method = Method::Dubai;
//         let params = method.parameters();

//         assert_eq!(params.fajr_angle, 18.2, "Parameters: {:?}", params);
//         assert_eq!(params.ishaa_parameter, IshaaParameter::Angle(18.2));
//     }

//     #[test]
//     fn parameters_for_moonsighting_committee() {
//         let method = Method::MoonsightingCommittee;
//         let params = method.parameters();

//         assert_eq!(params.fajr_angle, 18.0);
//         assert_eq!(params.ishaa_parameter, IshaaParameter::Angle(18.0));
//     }

//     #[test]
//     fn parameters_for_north_america() {
//         let method = Method::NorthAmerica;
//         let params = method.parameters();

//         assert_eq!(params.fajr_angle, 15.0);
//         assert_eq!(params.ishaa_parameter, IshaaParameter::Angle(15.0));
//     }

//     #[test]
//     fn parameters_for_kuwait() {
//         let method = Method::Kuwait;
//         let params = method.parameters();

//         assert_eq!(params.fajr_angle, 18.0);
//         assert_eq!(params.ishaa_parameter, IshaaParameter::Angle(17.5));
//     }

//     #[test]
//     fn parameters_for_qatar() {
//         let method = Method::Qatar;
//         let params = method.parameters();

//         assert_eq!(params.fajr_angle, 18.0);
//         assert_eq!(params.ishaa_parameter, IshaaParameter::Interval(90));
//     }

//     #[test]
//     fn parameters_for_singapore() {
//         let method = Method::Singapore;
//         let params = method.parameters();

//         assert_eq!(params.fajr_angle, 20.0);
//         assert_eq!(params.ishaa_parameter, IshaaParameter::Angle(18.0));
//     }

//     #[test]
//     fn parameters_for_other() {
//         let method = Method::Other;
//         let params = method.parameters();

//         assert_eq!(params.fajr_angle, 0.0);
//         assert_eq!(params.ishaa_parameter, IshaaParameter::Angle(0.0));
//     }
// }
