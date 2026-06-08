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

    let minute = parse_field(parts[0])?;
    let hour = parse_field(parts[1])?;
    let day_of_month = parse_field(parts[2])?;
    let month = parse_field(parts[3])?;
    let day_of_week = parse_field(parts[4])?;

    return Ok(Cron::new(minute, hour, day_of_month, month, day_of_week));
}

fn parse_field(input: &str) -> Result<Field, CronError> {
    if input == "*" {
        return Ok(Field::Any)
    }

    let value = input
        .parse::<u32>()
        .map_err(|_| CronError::InvalidExpression(format!("Invalid field value: {input}")))?;

    return Ok(Field::Exact(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_any_field() {
        let result = parse_field("*");
        assert_eq!(result.unwrap(), Field::Any);
    }

    #[test]
    fn parses_exact_field() {
        let result = parse_field("5");
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
        let result = parse_field("");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_field_literal() {
        let result = parse_field("abc");
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
}