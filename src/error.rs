/// Errors that can occur while parsing cron expressions or evaluating occurrences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The cron expression is syntactically invalid or contains out-of-range values.
    InvalidExpression(String),
    /// No matching occurrence was found within the evaluator search window.
    NoOccurrenceFound,
    /// The provided Unix timestamp could not be converted into a valid UTC datetime.
    InvalidTimestamp(i64),
}
