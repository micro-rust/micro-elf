//! Returns an `Arc` subslice.



use std::sync::Arc;



#[derive(Clone, Debug)]
pub struct SubSlice {
    /// Reference counted raw data.
    arc: Arc<[u8]>,

    /// Subslice range start.
    start: usize,

    /// Subslice range end.
    end: usize,
}

impl SubSlice {
    /// Creates a new subslice.
    pub fn new(arc: Arc<[u8]>, start: usize, end: usize) -> Self {
        Self { arc, start, end, }
    }

    /// Creates an iterator for the data.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a u8> {
        self.arc[self.start..self.end].iter()
    }
}

impl<I: core::slice::SliceIndex<[u8]>> core::ops::Index<I> for SubSlice {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &(&self.arc[self.start..self.end])[index]
    }
}
