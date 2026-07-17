//! Represents a location inside a source file.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    /// Byte offset where this span begins.
    pub start: usize,

    /// Byte offset where this span ends.
    pub end: usize,
}

impl Span {
    /// Creates a new span.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns the span length.
    pub fn length(&self) -> usize {
        self.end - self.start
    }
}
