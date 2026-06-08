#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    Any,
    Exact(u32),
    List(Vec<u32>),
    Range {
        start: u32,
        end: u32
    },
    Step {
        base: Box<Field>,
        step: u32
    }
}