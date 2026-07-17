//! Source file definitions for the Code programming language.
//!
//! A source file represents a single `.code` file being compiled.

/// Represents a source file.
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// Name or path of the source file.
    pub path: String,

    /// Entire source code.
    pub text: String,
}

impl SourceFile {
    /// Creates a new source file.
    pub fn new(path: String, text: String) -> Self {
        Self { path, text }
    }

    /// Returns the length of the source code in bytes.
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Returns `true` if the source file is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns the line and column (both 1-based) for a byte position.
    pub fn line_column(&self, position: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;

        for (index, ch) in self.text.char_indices() {
            if index >= position {
                break;
            }

            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        (line, column)
    }

    /// Returns the line number (1-based) for a byte position.
    pub fn line_number(&self, position: usize) -> usize {
        self.line_column(position).0
    }

    /// Returns the column number (1-based) for a byte position.
    pub fn column_number(&self, position: usize) -> usize {
        self.line_column(position).1
    }

    /// Returns the full source line containing the given byte position.
    pub fn line_text_at(&self, position: usize) -> &str {
        let bytes = self.text.as_bytes();

        let mut start = position.min(bytes.len());

        while start > 0 && bytes[start - 1] != b'\n' {
            start -= 1;
        }

        let mut end = position.min(bytes.len());

        while end < bytes.len() && bytes[end] != b'\n' {
            end += 1;
        }

        &self.text[start..end]
    }

    /// Returns the text of a specific line (1-based).
    pub fn line_text(&self, line: usize) -> Option<&str> {
        self.text.lines().nth(line.saturating_sub(1))
    }

    /// Returns line number, column number and line text for a byte position.
    pub fn location(&self, position: usize) -> (usize, usize, &str) {
        let (line, column) = self.line_column(position);
        let text = self.line_text_at(position);

        (line, column, text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_source_file() {
        let source = SourceFile::new("main.code".to_string(), "const PI = 3.14".to_string());

        assert_eq!(source.path, "main.code");
        assert_eq!(source.text, "const PI = 3.14");
    }

    #[test]
    fn reports_length() {
        let source = SourceFile::new("main.code".to_string(), "hello".to_string());

        assert_eq!(source.len(), 5);
    }

    #[test]
    fn detects_empty_source() {
        let source = SourceFile::new("empty.code".to_string(), String::new());

        assert!(source.is_empty());
    }

    #[test]
    fn reports_line_and_column() {
        let source = SourceFile::new("main.code".to_string(), "one\ntwo\nthree".to_string());

        assert_eq!(source.line_column(5), (2, 2));
    }

    #[test]
    fn extracts_line_text() {
        let source = SourceFile::new("main.code".to_string(), "one\ntwo\nthree".to_string());

        assert_eq!(source.line_text_at(5), "two");
    }

    #[test]
    fn reports_location() {
        let source = SourceFile::new("main.code".to_string(), "one\ntwo\nthree".to_string());

        let (line, column, text) = source.location(5);

        assert_eq!(line, 2);
        assert_eq!(column, 2);
        assert_eq!(text, "two");
    }
}
