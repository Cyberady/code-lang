//! Cursor for traversing source code.
//!
//! The cursor provides a simple API for reading characters from a source file.

use crate::source::SourceFile;

/// A cursor for traversing source code.
#[derive(Debug)]
pub struct Cursor<'a> {
    source: &'a SourceFile,
    position: usize,
}

impl<'a> Cursor<'a> {
    /// Creates a new cursor.
    pub fn new(source: &'a SourceFile) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    /// Returns the current position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns true if the cursor reached the end of the file.
    pub fn is_eof(&self) -> bool {
        self.position >= self.source.text.len()
    }

    /// Returns the current character.
    pub fn current(&self) -> Option<char> {
        self.source.text[self.position..].chars().next()
    }

    /// Returns the next character without advancing.
    pub fn peek(&self) -> Option<char> {
        self.source.text[self.position..].chars().nth(1)
    }

    /// Advances the cursor by one character.
    pub fn advance(&mut self) {
        if let Some(ch) = self.current() {
            self.position += ch.len_utf8();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::SourceFile;

    #[test]
    fn reads_current_character() {
        let source = SourceFile::new("main.code".to_string(), "abc".to_string());

        let cursor = Cursor::new(&source);

        assert_eq!(cursor.current(), Some('a'));
    }

    #[test]
    fn peeks_next_character() {
        let source = SourceFile::new("main.code".to_string(), "abc".to_string());

        let cursor = Cursor::new(&source);

        assert_eq!(cursor.peek(), Some('b'));
    }

    #[test]
    fn advances_cursor() {
        let source = SourceFile::new("main.code".to_string(), "abc".to_string());

        let mut cursor = Cursor::new(&source);

        cursor.advance();

        assert_eq!(cursor.current(), Some('b'));
    }

    #[test]
    fn reaches_end_of_file() {
        let source = SourceFile::new("main.code".to_string(), "a".to_string());

        let mut cursor = Cursor::new(&source);

        cursor.advance();

        assert!(cursor.is_eof());
    }
}
