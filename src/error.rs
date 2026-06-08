#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CronError {
    InvalidExpression(String),
    NoOccurrenceFound,
    InvalidTimestamp(i64),
}