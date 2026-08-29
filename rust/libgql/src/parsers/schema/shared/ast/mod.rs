pub mod runtime;
pub mod statictime;
pub mod traits;

pub trait AsStr<'s>:
    Ord
    + std::hash::Hash
    + std::borrow::Borrow<str>
    + Clone
    + Send
    + Sync
    + std::fmt::Debug
    + std::fmt::Display
{
    fn to_str(self: &Self) -> &str;
    fn from_str(s: &'s str) -> Self;
}

impl<'s> AsStr<'s> for &'s str {
    fn to_str(self: &Self) -> &str {
        *self
    }

    fn from_str(s: &'s str) -> Self {
        s
    }
}

impl<'s> AsStr<'s> for String {
    fn to_str(self: &Self) -> &str {
        self.as_str()
    }

    fn from_str(s: &'s str) -> Self {
        s.to_string()
    }
}
