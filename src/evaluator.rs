use crate::{Cron, CronError};

pub fn next_occurrence(_parsed: &Cron, _reference_timestamp: i64) -> Result<i64, CronError> {
    unimplemented!()
}