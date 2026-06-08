use crate::{Cron, Error};
use chrono::{DateTime, Datelike, Duration, Timelike, Utc};

/// Finds the next UTC Unix timestamp after `reference_timestamp` that matches
/// the provided parsed cron expression.
pub fn next_occurrence(parsed: &Cron, reference_timestamp: i64) -> Result<i64, Error> {
    let mut current = DateTime::<Utc>::from_timestamp(reference_timestamp, 0)
        .ok_or(Error::InvalidTimestamp(reference_timestamp))?;

    current += Duration::minutes(1);
    current = current
        .with_second(0)
        .and_then(|dt| dt.with_nanosecond(0))
        .ok_or(Error::InvalidTimestamp(reference_timestamp))?;

    const MAX_ITERATIONS: i32 = 2635200; // ~5 years in minutes

    for _ in 0..MAX_ITERATIONS {
        let minute = current.minute();
        let hour = current.hour();
        let day_of_month = current.day();
        let month = current.month();
        let day_of_week = current.weekday().num_days_from_sunday();

        if parsed.matches(minute, hour, day_of_month, month, day_of_week) {
            return Ok(current.timestamp());
        }

        current += Duration::minutes(1);
    }

    Err(Error::NoOccurrenceFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;
    use chrono::{TimeZone, Utc};

    #[test]
    fn finds_next_minute_for_all_wildcards() {
        let cron = parse("* * * * *").unwrap();
        let reference = Utc
            .with_ymd_and_hms(2026, 1, 1, 12, 0, 0)
            .unwrap()
            .timestamp();

        let next = next_occurrence(&cron, reference).unwrap();
        let expected = Utc
            .with_ymd_and_hms(2026, 1, 1, 12, 1, 0)
            .unwrap()
            .timestamp();

        assert_eq!(next, expected);
    }

    #[test]
    fn finds_next_midnight() {
        let cron = parse("0 0 * * *").unwrap();
        let reference = Utc
            .with_ymd_and_hms(2026, 1, 1, 12, 30, 0)
            .unwrap()
            .timestamp();

        let next = next_occurrence(&cron, reference).unwrap();
        let expected = Utc
            .with_ymd_and_hms(2026, 1, 2, 0, 0, 0)
            .unwrap()
            .timestamp();

        assert_eq!(next, expected);
    }

    #[test]
    fn finds_next_specific_weekday_time() {
        let cron = parse("30 8 * * 1").unwrap();

        // Tuesday, January 6, 2026, 09:00 UTC
        let reference = Utc
            .with_ymd_and_hms(2026, 1, 6, 9, 0, 0)
            .unwrap()
            .timestamp();

        let next = next_occurrence(&cron, reference).unwrap();

        // Next Monday is January 12, 2026 at 08:30 UTC
        let expected = Utc
            .with_ymd_and_hms(2026, 1, 12, 8, 30, 0)
            .unwrap()
            .timestamp();

        assert_eq!(next, expected);
    }

    #[test]
    fn handles_step_expression() {
        let cron = parse("*/15 * * * *").unwrap();
        let reference = Utc
            .with_ymd_and_hms(2026, 1, 1, 12, 7, 0)
            .unwrap()
            .timestamp();

        let next = next_occurrence(&cron, reference).unwrap();
        let expected = Utc
            .with_ymd_and_hms(2026, 1, 1, 12, 15, 0)
            .unwrap()
            .timestamp();

        assert_eq!(next, expected);
    }

    #[test]
    fn handles_range_step_expression() {
        let cron = parse("0 9-17/2 * * *").unwrap();
        let reference = Utc
            .with_ymd_and_hms(2026, 1, 1, 9, 30, 0)
            .unwrap()
            .timestamp();

        let next = next_occurrence(&cron, reference).unwrap();
        let expected = Utc
            .with_ymd_and_hms(2026, 1, 1, 11, 0, 0)
            .unwrap()
            .timestamp();

        assert_eq!(next, expected);
    }

    #[test]
    fn handles_month_boundary_rollover() {
        let cron = parse("0 0 1 * *").unwrap();
        let reference = Utc
            .with_ymd_and_hms(2026, 1, 31, 23, 59, 0)
            .unwrap()
            .timestamp();

        let next = next_occurrence(&cron, reference).unwrap();
        let expected = Utc
            .with_ymd_and_hms(2026, 2, 1, 0, 0, 0)
            .unwrap()
            .timestamp();

        assert_eq!(next, expected);
    }

    #[test]
    fn handles_day_of_month_overflow() {
        let cron = parse("0 0 31 * *").unwrap();
        let reference = Utc
            .with_ymd_and_hms(2026, 4, 1, 0, 0, 0)
            .unwrap()
            .timestamp();

        let next = next_occurrence(&cron, reference).unwrap();

        // April has no 31st, so next should be May 31
        let expected = Utc
            .with_ymd_and_hms(2026, 5, 31, 0, 0, 0)
            .unwrap()
            .timestamp();

        assert_eq!(next, expected);
    }

    #[test]
    fn handles_year_rollover() {
        let cron = parse("0 0 1 1 *").unwrap();
        let reference = Utc
            .with_ymd_and_hms(2026, 12, 31, 23, 59, 0)
            .unwrap()
            .timestamp();

        let next = next_occurrence(&cron, reference).unwrap();
        let expected = Utc
            .with_ymd_and_hms(2027, 1, 1, 0, 0, 0)
            .unwrap()
            .timestamp();

        assert_eq!(next, expected);
    }

    #[test]
    fn returns_no_occurrence_for_impossible_schedule() {
        let cron = parse("0 0 31 2 *").unwrap();
        let reference = Utc
            .with_ymd_and_hms(2026, 1, 1, 0, 0, 0)
            .unwrap()
            .timestamp();

        let result = next_occurrence(&cron, reference);

        assert!(matches!(result, Err(Error::NoOccurrenceFound)));
    }
}
