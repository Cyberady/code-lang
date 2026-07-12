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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_source_file() {
        let source = SourceFile::new(
            "main.code".to_string(),
            "const PI = 3.14".to_string(),
        );

        assert_eq!(source.path, "main.code");
        assert_eq!(source.text, "const PI = 3.14");
    }

    #[test]
    fn reports_length() {
        let source = SourceFile::new(
            "main.code".to_string(),
            "hello".to_string(),
        );

        assert_eq!(source.len(), 5);
    }

    #[test]
    fn detects_empty_source() {
        let source = SourceFile::new(
            "empty.code".to_string(),
            String::new(),
        );

        assert!(source.is_empty());
    }
}
