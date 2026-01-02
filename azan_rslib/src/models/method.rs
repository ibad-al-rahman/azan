pub type Method = azan::Method;

#[uniffi::remote(Enum)]
pub enum Method {
    MuslimWorldLeague,
    Egyptian,
    UmmAlQura,
    MoonsightingCommittee,
    NorthAmerica,
    Singapore,
}
