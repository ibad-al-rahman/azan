/// A well-known Islamic holiday or observance tied to a fixed Hijri date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IslamicEvent {
    /// 1 Muharram — Islamic New Year
    IslamicNewYear,
    /// 10 Muharram — Day of Ashura
    Ashura,
    /// 12 Rabi' I — Prophet's Birthday (Mawlid al-Nabi)
    MawlidAlNabi,
    /// 23 Rabi' II (583 AH)
    BattleOfHattin,
    /// 15 Jumada I (8 AH)
    BattleOfMutah,
    /// 10 Rajab (9 AH)
    BattleOfTabuk,
    /// 27 Rajab — Night Journey and Ascension (Isra and Mi'raj)
    IsraAndMiraj,
    /// 15 Sha'ban — Middle of Sha'ban (Nisf Sha'ban / Laylat al-Bara'at)
    NisfShaban,
    /// 1 Ramadan — First day of Ramadan
    FirstOfRamadan,
    /// 17 Ramadan (2 AH)
    BattleOfBadr,
    /// 20 Ramadan (8 AH)
    ConquestOfMecca,
    /// 27 Ramadan — Night of Power (Laylat al-Qadr)
    LaylatAlQadr,
    /// 1 Shawwal — Eid al-Fitr
    EidAlFitr,
    /// 4 Shawwal (3 AH)
    BattleOfUhud,
    /// 9 Dhul Hijja — Day of Arafah (Waqfat Arafat)
    DayOfArafah,
    /// 10 Dhul Hijja — Eid al-Adha
    EidAlAdha,
}

impl IslamicEvent {
    pub fn for_date(month: u8, day: u8) -> Vec<IslamicEvent> {
        EVENTS
            .iter()
            .filter(|(m, d, _)| *m == month && *d == day)
            .map(|(_, _, e)| *e)
            .collect()
    }
}

/// (month, day, event)
const EVENTS: &[(u8, u8, IslamicEvent)] = &[
    (1, 1, IslamicEvent::IslamicNewYear),
    (1, 10, IslamicEvent::Ashura),
    (3, 12, IslamicEvent::MawlidAlNabi),
    (7, 27, IslamicEvent::IsraAndMiraj),
    (8, 15, IslamicEvent::NisfShaban),
    (9, 1, IslamicEvent::FirstOfRamadan),
    (9, 27, IslamicEvent::LaylatAlQadr),
    (10, 1, IslamicEvent::EidAlFitr),
    (12, 9, IslamicEvent::DayOfArafah),
    (12, 10, IslamicEvent::EidAlAdha),
];
