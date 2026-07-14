//! Parser errors.

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken,
    UnexpectedEOF,
}
