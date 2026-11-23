#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct YearMonth {
    year: i32,
    month: u32,
}

#[derive(Debug)]
pub enum YearMonthError {
    MonthOutOfRange,
}

impl YearMonth {
    pub fn new(year: i32, month: u32) -> Result<Self, YearMonthError> {
        if !(1..=12).contains(&month) {
            return Err(YearMonthError::MonthOutOfRange);
        }
        Ok(Self { year, month })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_succeeds_when_month_is_january() {
        let month = 1;

        let result = YearMonth::new(2025, month);

        assert!(result.is_ok());
    }

    #[test]
    fn new_succeeds_when_month_is_december() {
        let month = 12;

        let result = YearMonth::new(2025, month);

        assert!(result.is_ok());
    }

    #[test]
    fn new_fails_when_month_is_zero() {
        let month = 0;

        let result = YearMonth::new(2025, month);

        assert!(matches!(result, Err(YearMonthError::MonthOutOfRange)));
    }

    #[test]
    fn new_fails_when_month_is_thirteen() {
        let month = 13;

        let result = YearMonth::new(2025, month);

        assert!(matches!(result, Err(YearMonthError::MonthOutOfRange)));
    }
}
