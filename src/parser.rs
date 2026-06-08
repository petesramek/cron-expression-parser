use crate::{Cron, CronError};
use crate::field::Field;

pub fn parse(_expression: &str) -> Result<Cron, CronError> {
    unimplemented!()
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
    fn rejects_empty_literal() {
        let result = parse_field("");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_literal() {
        let result = parse_field("abc");
        assert!(result.is_err());
    }
}