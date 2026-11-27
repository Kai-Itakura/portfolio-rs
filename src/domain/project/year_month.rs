use chrono::Month;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct YearMonth {
    pub year: i32,
    pub month: Month,
}
