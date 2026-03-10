#[derive(PartialEq, Debug, Copy, Clone)]
pub enum IshaaParameter {
    Angle(f64),
    Interval(i32),
}

impl Default for IshaaParameter {
    fn default() -> Self {
        IshaaParameter::Angle(0.0)
    }
}
