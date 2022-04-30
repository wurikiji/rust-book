use core::fmt;
use std::ops::Deref;

pub struct Wrapper(pub Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

impl Deref for Wrapper {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
