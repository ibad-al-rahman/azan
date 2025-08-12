/// Shafaq is the twilight in the sky. Different madhabs define the appearance of
/// twilight differently. These values are used by the MoonsightingComittee method
/// for the different ways to calculate Ishaa.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Twilight {
    /// General is a combination of Ahmer and Abyad.
    General,

    /// Ahmer means the twilight is the red glow in the sky.
    /// Used by the Shafi, Maliki, and Hanbali madhabs.
    Red,

    /// Abyad means the twilight is the white glow in the sky.
    /// Used by the Hanafi madhab.
    White,
}
