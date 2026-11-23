use crate::domain::project::year_month::YearMonth;

pub enum Period {
    Past { start: YearMonth, end: YearMonth },
    Ongoing { start: YearMonth },
}

#[derive(Debug)]
pub enum PeriodError {
    EndBeforeStart,
}

impl Period {
    pub fn from_dates(start: YearMonth, end: Option<YearMonth>) -> Result<Self, PeriodError> {
        match end {
            Some(end) if end < start => Err(PeriodError::EndBeforeStart),
            Some(end) => Ok(Self::Past { start, end }),
            None => Ok(Self::Ongoing { start }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn year_month(year: i32, month: u32) -> YearMonth {
        YearMonth::new(year, month).unwrap()
    }

    #[test]
    fn from_dates_returns_ongoing_when_end_is_none() {
        let start = year_month(2026, 1);

        let result = Period::from_dates(start, None);

        assert!(matches!(result, Ok(Period::Ongoing { .. })));
    }

    #[test]
    fn from_dates_returns_past_when_end_is_after_start() {
        let start = year_month(2025, 3);
        let end = year_month(2025, 9);

        let result = Period::from_dates(start, Some(end));

        assert!(matches!(result, Ok(Period::Past { .. })));
    }

    #[test]
    fn from_dates_returns_past_when_end_is_same_month_as_start() {
        let start = year_month(2023, 7);
        let end = year_month(2023, 7);

        let result = Period::from_dates(start, Some(end));

        assert!(matches!(result, Ok(Period::Past { .. })));
    }

    #[test]
    fn from_dates_returns_past_when_end_is_next_year_with_smaller_month() {
        let start = year_month(2023, 11);
        let end = year_month(2024, 2);

        let result = Period::from_dates(start, Some(end));

        assert!(matches!(result, Ok(Period::Past { .. })));
    }

    #[test]
    fn from_dates_fails_when_end_is_before_start() {
        let start = year_month(2025, 9);
        let end = year_month(2025, 3);

        let result = Period::from_dates(start, Some(end));

        assert!(matches!(result, Err(PeriodError::EndBeforeStart)));
    }
}
