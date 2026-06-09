use crate::field::Field;
use crate::{Cron, Error};

/// Parses a standard 5-field cron expression into a validated structured representation.
pub fn parse(expression: &str) -> Result<Cron, Error> {
    let parts: Vec<&str> = expression.split_whitespace().collect();

    if parts.len() != 5 {
        return Err(Error::InvalidExpression(format!(
            "Expected 5 fields, got {}",
            parts.len()
        )));
    }

    let minute = parse_field(parts[0], 0, 59, "minute")?;
    let hour = parse_field(parts[1], 0, 23, "hour")?;
    let day_of_month = parse_field(parts[2], 1, 31, "day of month")?;
    let month = parse_field(parts[3], 1, 12, "month")?;
    let day_of_week = parse_field(parts[4], 0, 6, "day of week")?;

    Ok(Cron::new(minute, hour, day_of_month, month, day_of_week))
}

/// Parses a single cron field according to the supported syntax for this implementation.
fn parse_field(input: &str, min: u32, max: u32, field_name: &str) -> Result<Field, Error> {
    if input == "*" {
        return Ok(Field::Any);
    }

    if input.contains('/') {
        return parse_step(input, min, max, field_name);
    }

    if input.contains(',') {
        let values = input
            .split(',')
            .map(|part| parse_literal(part, min, max, field_name))
            .collect::<Result<Vec<u32>, Error>>()?;

        return Ok(Field::List(values));
    }

    if input.contains('-') {
        return parse_range(input, min, max, field_name);
    }

    let value = parse_literal(input, min, max, field_name)?;
    Ok(Field::Exact(value))
}

/// Parses a step expression such as `*/15` or `1-30/5`.
fn parse_step(input: &str, min: u32, max: u32, field_name: &str) -> Result<Field, Error> {
    let parts: Vec<&str> = input.split('/').collect();

    if parts.len() != 2 {
        return Err(Error::InvalidExpression(format!(
            "Invalid {field_name} step: {input}"
        )));
    }

    let base_part = parts[0];
    let step_part = parts[1];

    let step = step_part.parse::<u32>().map_err(|_| {
        Error::InvalidExpression(format!("Invalid {field_name} step value: {step_part}"))
    })?;

    if step == 0 {
        return Err(Error::InvalidExpression(format!(
            "{field_name} step must be greater than 0: {input}"
        )));
    }

    let base = parse_step_base(base_part, min, max, field_name)?;

    Ok(Field::Step {
        base: Box::new(base),
        step,
    })
}

/// Parses the base expression for a step field.
///
/// In the current implementation, the supported step bases are:
///
/// - `*`
/// - a range such as `1-30`
fn parse_step_base(base: &str, min: u32, max: u32, field_name: &str) -> Result<Field, Error> {
    if base == "*" {
        return Ok(Field::Any);
    }

    if base.contains('-') {
        return parse_range(base, min, max, field_name);
    }

    Err(Error::InvalidExpression(format!(
        "Invalid {field_name} step base: {base}"
    )))
}


/// Parses a range expression such as `1-5`.
///
/// Ranges are inclusive and may use the same start and end value,
/// so expressions such as `5-5` are treated as valid ranges
fn parse_range(input: &str, min: u32, max: u32, field_name: &str) -> Result<Field, Error> {
    let parts: Vec<&str> = input.split('-').collect();

    if parts.len() != 2 {
        return Err(Error::InvalidExpression(format!(
            "Invalid {field_name} range: {input}"
        )));
    }

    let start = parse_literal(parts[0], min, max, field_name)?;
    let end = parse_literal(parts[1], min, max, field_name)?;

    if start > end {
        return Err(Error::InvalidExpression(format!(
            "Invalid {field_name} range: start {start} is greater than end {end}"
        )));
    }

    Ok(Field::Range { start, end })
}

/// Parses a single numeric literal and validates it against the allowed field range.
fn parse_literal(input: &str, min: u32, max: u32, field_name: &str) -> Result<u32, Error> {
    let value = input
        .parse::<u32>()
        .map_err(|_| Error::InvalidExpression(format!("Invalid {field_name}: {input}")))?;

    if value < min || value > max {
        return Err(Error::InvalidExpression(format!(
            "{field_name} value out of range: {value} (expected {min}-{max})"
        )));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_any_field() {
        let result = parse_field("*", 0, 59, "minute");
        assert_eq!(result.unwrap(), Field::Any);
    }

    #[test]
    fn parses_exact_field() {
        let result = parse_field("5", 0, 59, "minute");
        assert_eq!(result.unwrap(), Field::Exact(5));
    }

    #[test]
    fn parses_list_field() {
        let result = parse_field("1,15,30", 0, 59, "minute");
        assert_eq!(result.unwrap(), Field::List(vec![1, 15, 30]));
    }

    #[test]
    fn parses_valid_range_field() {
        let result = parse_field("1-5", 0, 59, "minute");
        assert_eq!(result.unwrap(), Field::Range { start: 1, end: 5 });
    }

    #[test]
    fn parses_equal_range_field() {
        let result = parse_field("5-5", 0, 59, "minute");
        assert_eq!(result.unwrap(), Field::Range { start: 5, end: 5 });
    }


    #[test]
    fn parses_wildcard_step_field() {
        let result = parse_field("*/15", 0, 59, "minute");
        assert_eq!(
            result.unwrap(),
            Field::Step {
                base: Box::new(Field::Any),
                step: 15,
            }
        );
    }

    #[test]
    fn parses_range_step_field() {
        let result = parse_field("1-30/5", 0, 59, "minute");
        assert_eq!(
            result.unwrap(),
            Field::Step {
                base: Box::new(Field::Range { start: 1, end: 30 }),
                step: 5,
            }
        );
    }

    #[test]
    fn parses_exact_cron() {
        let result = parse("1 1 1 1 1");
        assert_eq!(
            result.unwrap(),
            Cron::new(
                Field::Exact(1),
                Field::Exact(1),
                Field::Exact(1),
                Field::Exact(1),
                Field::Exact(1)
            )
        );
    }

    #[test]
    fn parses_wildcard_cron() {
        let result = parse("* * * * *");
        assert_eq!(
            result.unwrap(),
            Cron::new(Field::Any, Field::Any, Field::Any, Field::Any, Field::Any)
        );
    }

    #[test]
    fn parses_mixed_exact_and_wildcard_cron() {
        let cron = parse("0 12 * * 1");
        assert!(cron.is_ok());
    }

    #[test]
    fn parses_list_cron() {
        let result = parse("1,15,30 * * * *");
        assert_eq!(
            result.unwrap(),
            Cron::new(
                Field::List(vec![1, 15, 30]),
                Field::Any,
                Field::Any,
                Field::Any,
                Field::Any
            )
        );
    }

    #[test]
    fn parses_range_cron() {
        let result = parse("1-5 * 5-5 * *");
        assert_eq!(
            result.unwrap(),
            Cron::new(
                Field::Range { start: 1, end: 5 },
                Field::Any,
                Field::Range { start: 5, end: 5 },
                Field::Any,
                Field::Any
            )
        );
    }

    #[test]
    fn parses_step_cron() {
        let result = parse("*/15 * * * *");
        assert_eq!(
            result.unwrap(),
            Cron::new(
                Field::Step {
                    base: Box::new(Field::Any),
                    step: 15,
                },
                Field::Any,
                Field::Any,
                Field::Any,
                Field::Any
            )
        );
    }

    #[test]
    fn rejects_empty_field_literal() {
        let result = parse_field("", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_field_literal() {
        let result = parse_field("abc", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_out_of_range_field() {
        let result = parse_field("70", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_value_in_list() {
        let result = parse_field("1,abc,30", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_out_of_range_value_in_list() {
        let result = parse_field("1,70,30", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_range_with_invalid_start() {
        let result = parse_field("a-5", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_range_with_invalid_end() {
        let result = parse_field("1-b", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_range_with_out_of_range_value() {
        let result = parse_field("1-70", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_reversed_range() {
        let result = parse_field("10-5", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_malformed_range() {
        let result = parse_field("1-5-9", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_zero_step() {
        let result = parse_field("*/0", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_step_value() {
        let result = parse_field("*/abc", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_step_base() {
        let result = parse_field("5/10", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_range_step_base() {
        let result = parse_field("70-80/5", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_malformed_step() {
        let result = parse_field("*/15/2", 0, 59, "minute");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_cron_values() {
        let result = parse("* * * * a");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_cron_length() {
        let result = parse("* * * *");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_minute() {
        let result = parse("70 * * * *");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_hour() {
        let result = parse("* 24 * * *");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_day() {
        let result = parse("* * 32 * *");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_month() {
        let result = parse("* * * 13 *");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_day_of_week() {
        let result = parse("* * * * 7");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_empty_range() {
        let result = parse("* * * * -");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_empty_list() {
        let result = parse("* * * * ,");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_empty_step() {
        let result = parse("* * * * /");
        assert!(result.is_err());
    }
}
