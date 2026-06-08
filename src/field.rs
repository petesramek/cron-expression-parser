#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    Any,
    Exact(u32),
    List(Vec<u32>),
    Range { start: u32, end: u32 },
    Step { base: Box<Field>, step: u32 },
}

impl Field {
    pub(crate) fn matches(&self, value: u32, field_min: u32) -> bool {
        match self {
            Field::Any => true,
            Field::Exact(exact) => *exact == value,
            Field::Range { start, end } => start <= &value && &value <= end,
            Field::List(values) => values.contains(&value),
            Field::Step { base, step } => match base.as_ref() {
                Field::Any => value >= field_min && (value - field_min).is_multiple_of(*step),
                Field::Range { start, end } => {
                    value >= *start && value <= *end && (value - *start).is_multiple_of(*step)
                }

                // Should be also applied to list, but is not in the specs defined.
                _ => false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_matches_everything() {
        let field = Field::Any;
        assert!(field.matches(0, 0));
        assert!(field.matches(30, 0));
        assert!(field.matches(59, 0));
    }

    #[test]
    fn exact_matches_only_same_value() {
        let field = Field::Exact(5);
        assert!(field.matches(5, 0));
        assert!(!field.matches(4, 0));
        assert!(!field.matches(6, 0));
    }

    #[test]
    fn list_matches_any_listed_value() {
        let field = Field::List(vec![1, 15, 30]);
        assert!(field.matches(1, 0));
        assert!(field.matches(15, 0));
        assert!(field.matches(30, 0));
        assert!(!field.matches(2, 0));
    }

    #[test]
    fn range_matches_inside_bounds() {
        let field = Field::Range { start: 1, end: 5 };
        assert!(field.matches(1, 0));
        assert!(field.matches(3, 0));
        assert!(field.matches(5, 0));
        assert!(!field.matches(0, 0));
        assert!(!field.matches(6, 0));
    }

    #[test]
    fn wildcard_step_matches_from_field_min() {
        let field = Field::Step {
            base: Box::new(Field::Any),
            step: 15,
        };

        assert!(field.matches(0, 0));
        assert!(field.matches(15, 0));
        assert!(field.matches(30, 0));
        assert!(field.matches(45, 0));
        assert!(!field.matches(1, 0));
        assert!(!field.matches(14, 0));
    }

    #[test]
    fn wildcard_step_respects_non_zero_field_min() {
        let field = Field::Step {
            base: Box::new(Field::Any),
            step: 2,
        };

        // For month/day-of-month style fields, field_min can be 1
        assert!(field.matches(1, 1));
        assert!(field.matches(3, 1));
        assert!(field.matches(5, 1));
        assert!(!field.matches(2, 1));
        assert!(!field.matches(4, 1));
    }

    #[test]
    fn range_step_matches_from_range_start() {
        let field = Field::Step {
            base: Box::new(Field::Range { start: 9, end: 17 }),
            step: 2,
        };

        assert!(field.matches(9, 0));
        assert!(field.matches(11, 0));
        assert!(field.matches(13, 0));
        assert!(field.matches(15, 0));
        assert!(field.matches(17, 0));

        assert!(!field.matches(8, 0));
        assert!(!field.matches(10, 0));
        assert!(!field.matches(18, 0));
    }
}
