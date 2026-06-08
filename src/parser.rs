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