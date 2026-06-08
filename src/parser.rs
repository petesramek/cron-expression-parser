use crate::{Cron, CronError};
use crate::field::Field;

pub fn parse(expression: &str) -> Result<Cron, CronError> {
    let parts: Vec<&str> = expression.split_whitespace().collect();

    if parts.len() != 5 {
        return Err(CronError::InvalidExpression(format!(
            "Expected 5 fields, got {}",
            parts.len()
        )));
    }

    let minute = parse_field(parts[0], 0, 59, "minute")?;
    let hour = parse_field(parts[1], 0, 23, "hour")?;
    let day_of_month = parse_field(parts[2], 1, 31, "day of month")?;
    let month = parse_field(parts[3], 1, 12, "month")?;
    let day_of_week = parse_field(parts[4], 0, 6, "day of week")?;

    return Ok(Cron::new(minute, hour, day_of_month, month, day_of_week));
}

fn parse_field(input: &str, min: u32, max: u32, field_name: &str) -> Result<Field, CronError> {
    if input == "*" {
        return Ok(Field::Any)
    }

    let value = input
        .parse::<u32>()
        .map_err(|_| CronError::InvalidExpression(format!("Invalid field value: {input}")))?;


    if value < min || value > max {
        return Err(CronError::InvalidExpression(format!(
            "{field_name} value out of range: {value} (expected {min}-{max})"
        )));
    }


    return Ok(Field::Exact(value))
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
    fn parses_exact_cron() {
        let result = parse("1 1 1 1 1");
        assert_eq!(result.unwrap(), Cron::new(Field::Exact(1),Field::Exact(1),Field::Exact(1),Field::Exact(1),Field::Exact(1)));
    }

    #[test]
    fn parses_wildcard_cron() {
        let result = parse("* * * * *");
        assert_eq!(result.unwrap(), Cron::new(Field::Any,Field::Any,Field::Any,Field::Any,Field::Any));
    }


    #[test]
    fn parses_mixed_exact_and_wildcard_cron() {
        let cron = parse("0 12 * * 1");
        assert!(cron.is_ok());
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

}