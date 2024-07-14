use std::fmt;

#[derive(Debug)]
pub struct OutOfBoundsError(pub usize);

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tried to access out of bounds memory @ {}", self.0)
    }
}
