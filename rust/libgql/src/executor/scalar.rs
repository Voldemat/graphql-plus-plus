pub trait Scalar: Sized + std::fmt::Debug {
    fn from_string(s: &str) -> Result<Self, String>;
    fn from_u64(n: u64) -> Result<Self, String>;
    fn from_i64(n: i64) -> Result<Self, String>;
    fn from_f64(n: f64) -> Result<Self, String>;
    fn from_bool(b: bool) -> Result<Self, String>;
    fn get_str(self: &Self) -> Option<&str>;
}

