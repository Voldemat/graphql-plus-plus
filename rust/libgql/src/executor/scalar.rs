pub trait Scalar: Sized + std::fmt::Debug {
    fn get_str(self: &Self) -> Option<&str>;
    fn from_string(str: &str) -> Result<Self, String>;
}
