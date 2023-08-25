//! Possible IDs for a Section.



pub trait SectionID {
    /// `true` if the type is `usize`.
    const NUMERIC: bool;

    /// Converts the ID into a `String`.
    fn name(self) -> String;

    /// Converts the ID into a `usize`.
    fn index(self) -> usize;
}

impl SectionID for &str {
    const NUMERIC: bool = false;

    fn name(self) -> String {
        String::from( self )
    }

    fn index(self) -> usize {
        0
    }
}

impl SectionID for String {
    const NUMERIC: bool = false;

    fn name(self) -> String {
        self
    }

    fn index(self) -> usize {
        0
    }
}

impl SectionID for usize {
    const NUMERIC: bool = true;

    fn name(self) -> String {
        String::new()
    }

    fn index(self) -> usize {
        self
    }
}
