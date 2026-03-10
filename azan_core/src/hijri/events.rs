use std::fmt;

/// A well-known Islamic holiday or observance tied to a fixed Hijri date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HijriEvent {
    /// 1 Muharram — Islamic New Year
    IslamicNewYear,
    /// 10 Muharram — Day of Ashura
    Ashura,
    /// 12 Rabi al-Awwal — Prophet's Birthday (Mawlid al-Nabi)
    MawlidAlNabi,
    /// 27 Rajab — Night Journey and Ascension (Isra and Mi'raj)
    IsraAndMiraj,
    /// 15 Sha'ban — Middle of Sha'ban (Nisf Sha'ban / Laylat al-Bara'at)
    NisfShaban,
    /// 1 Ramadan — First day of Ramadan
    FirstOfRamadan,
    /// 27 Ramadan — Night of Power (Laylat al-Qadr)
    LaylatAlQadr,
    /// 1 Shawwal — Eid al-Fitr
    EidAlFitr,
    /// 9 Dhul Hijja — Day of Arafah (Waqfat Arafat)
    DayOfArafah,
    /// 10 Dhul Hijja — Eid al-Adha
    EidAlAdha,
}

impl HijriEvent {
    pub(crate) fn for_date(month: u8, day: u8) -> Vec<HijriEvent> {
        EVENTS
            .iter()
            .filter(|(m, d, _)| *m == month && *d == day)
            .map(|(_, _, e)| *e)
            .collect()
    }
}

/// (month, day, event)
const EVENTS: &[(u8, u8, HijriEvent)] = &[
    (1, 1, HijriEvent::IslamicNewYear),
    (1, 10, HijriEvent::Ashura),
    (3, 12, HijriEvent::MawlidAlNabi),
    (7, 27, HijriEvent::IsraAndMiraj),
    (8, 15, HijriEvent::NisfShaban),
    (9, 1, HijriEvent::FirstOfRamadan),
    (9, 27, HijriEvent::LaylatAlQadr),
    (10, 1, HijriEvent::EidAlFitr),
    (12, 9, HijriEvent::DayOfArafah),
    (12, 10, HijriEvent::EidAlAdha),
];

impl fmt::Display for HijriEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            HijriEvent::IslamicNewYear => "Islamic New Year",
            HijriEvent::Ashura => "Ashura",
            HijriEvent::MawlidAlNabi => "Mawlid al-Nabi",
            HijriEvent::IsraAndMiraj => "Isra and Mi'raj",
            HijriEvent::NisfShaban => "Nisf Sha'ban",
            HijriEvent::FirstOfRamadan => "First of Ramadan",
            HijriEvent::LaylatAlQadr => "Laylat al-Qadr",
            HijriEvent::EidAlFitr => "Eid al-Fitr",
            HijriEvent::DayOfArafah => "Day of Arafah",
            HijriEvent::EidAlAdha => "Eid al-Adha",
        };
        write!(f, "{name}")
    }
}
