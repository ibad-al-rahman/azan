/// Setting for the Asr prayer time.
/// For Hanafi mazhab, the Asr is bit later
/// than that of the Shafi mazhab.
#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum Mazhab {
    #[default]
    Shafi,
    Hanafi,
}

impl Mazhab {
    pub fn shadow(&self) -> i32 {
        match self {
            Mazhab::Shafi => 1,
            Mazhab::Hanafi => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shafi_shadow() {
        let shafi = Mazhab::Shafi;

        assert_eq!(shafi.shadow(), 1);
    }

    #[test]
    fn hanafi_shadow() {
        let hanafi = Mazhab::Hanafi;

        assert_eq!(hanafi.shadow(), 2);
    }
}
