use std::fmt;

/// CharCount holds a count of a single character
#[derive(PartialEq, Eq, Hash)]
pub struct CharCount {
    pub letter: char,
    pub count: usize,
}

impl fmt::Display for CharCount {
    /// Formatter for CharCount
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} times {})", self.count, self.letter)
    }
}
impl fmt::Debug for CharCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.count)
            .field(&self.letter)
            .finish()
    }
}
impl CharCount {
    /// Returns a new CharCount from a given letter
    pub fn new(letter: char) -> CharCount {
        CharCount {
            letter: letter,
            count: 1,
        }
    }
}
