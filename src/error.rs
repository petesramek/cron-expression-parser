#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidExpression(String),
    NoOccurrenceFound,
    InvalidTimestamp(i64),
}
