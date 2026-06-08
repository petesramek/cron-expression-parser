//! A small library for parsing standard 5-field cron expressions and computing
//! the next matching UTC occurrence after a given Unix timestamp.
//!
//! # Public API
//!
//! - [`parse`] parses a cron expression into a validated [`Cron`] value
//! - [`next_occurrence`] finds the next matching Unix timestamp after a reference time
//!
//! # Supported syntax
//!
//! - exact values
//! - wildcard (`*`)
//! - lists of exact values
//! - ranges
//! - wildcard steps
//! - range steps
//!
//! # Notes
//!
//! - Matching is performed in UTC
//! - Unsupported syntax is intentionally rejected
//! - Impossible schedules return [`Error::NoOccurrenceFound`]
mod cron;
mod error;
mod evaluator;
mod field;
mod parser;

pub use cron::Cron;
pub use error::Error;
pub use evaluator::next_occurrence;
pub use parser::parse;
