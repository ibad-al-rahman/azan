/// Different mazaheb define the appearance of twilight differently.
/// These values are used by the MoonsightingComittee method
/// for the different ways to calculate Ishaa.
#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum Twilight {
    /// General is a combination of Ahmer and Abyad.
    #[default]
    General,

    /// Ahmer means the twilight is the red glow in the sky.
    /// Used by the Shafi, Maliki, and Hanbali mazaheb.
    Red,

    /// Abyad means the twilight is the white glow in the sky.
    /// Used by the Hanafi mazhab.
    White,
}
