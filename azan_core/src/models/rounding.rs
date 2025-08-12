#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum Rounding {
    #[default]
    Nearest,
    Ceil,
    None,
}
