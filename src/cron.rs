use crate::field::Field;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cron {
    minute: Field,
    hour: Field,
    day_of_month: Field,
    month: Field,
    day_of_week: Field,
}

impl Cron {
    pub(crate) fn new(
        minute: Field,
        hour: Field,
        day_of_month: Field,
        month: Field,
        day_of_week: Field,
    ) -> Self {
        Self {
            minute,
            hour,
            day_of_month,
            month,
            day_of_week,
        }
    }

    pub(crate) fn matches(
        &self,
        minute: u32,
        hour: u32,
        day_of_month: u32,
        month: u32,
        day_of_week: u32,
    ) -> bool {
        self.minute.matches(minute, 0)
            && self.hour.matches(hour, 0)
            && self.day_of_month.matches(day_of_month, 1)
            && self.month.matches(month, 1)
            && self.day_of_week.matches(day_of_week, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_all_wildcards() {
        let cron = Cron::new(
            Field::Any,
            Field::Any,
            Field::Any,
            Field::Any,
            Field::Any,
        );

        assert!(cron.matches(0, 0, 1, 1, 0));
        assert!(cron.matches(59, 23, 31, 12, 6));
    }

    #[test]
    fn matches_exact_values() {
        let cron = Cron::new(
            Field::Exact(30),
            Field::Exact(8),
            Field::Any,
            Field::Any,
            Field::Exact(1),
        );

        assert!(cron.matches(30, 8, 15, 6, 1));
        assert!(!cron.matches(29, 8, 15, 6, 1));
        assert!(!cron.matches(30, 9, 15, 6, 1));
        assert!(!cron.matches(30, 8, 15, 6, 2));
    }

    #[test]
    fn matches_list_range_and_step_combination() {
        let cron = Cron::new(
            Field::Step {
                base: Box::new(Field::Any),
                step: 15,
            },
            Field::Range { start: 9, end: 17 },
            Field::Any,
            Field::List(vec![1, 6, 12]),
            Field::Exact(1),
        );

        assert!(cron.matches(30, 9, 10, 6, 1));
        assert!(cron.matches(45, 17, 10, 12, 1));

        assert!(!cron.matches(31, 9, 10, 6, 1));  // minute not on step
        assert!(!cron.matches(30, 18, 10, 6, 1)); // hour out of range
        assert!(!cron.matches(30, 9, 10, 7, 1));  // month not in list
        assert!(!cron.matches(30, 9, 10, 6, 2));  // day of week mismatch
    }

    #[test]
    fn respects_one_based_fields_for_steps() {
        let cron = Cron::new(
            Field::Any,
            Field::Any,
            Field::Step {
                base: Box::new(Field::Any),
                step: 2,
            },
            Field::Step {
                base: Box::new(Field::Any),
                step: 2,
            },
            Field::Any,
        );

        // day_of_month and month are one-based fields
        assert!(cron.matches(0, 0, 1, 1, 0));
        assert!(cron.matches(0, 0, 3, 3, 0));

        assert!(!cron.matches(0, 0, 2, 1, 0));
        assert!(!cron.matches(0, 0, 1, 2, 0));
    }
}
