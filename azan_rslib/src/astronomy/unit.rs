pub type Coordinates = azan::Coordinates;

#[uniffi::remote(Record)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}
