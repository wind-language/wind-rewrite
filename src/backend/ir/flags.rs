#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FunctionModifer {
    NoMangle = 1 << 0,
}
impl FunctionModifer {
    pub fn from_str(s: &str) -> Option<FunctionModifer> {
        match s {
            "no_mangle" => Some(FunctionModifer::NoMangle),
            _ => None,
        }
    }
}