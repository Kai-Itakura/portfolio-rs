use crate::domain::project::year_month::YearMonth;

#[expect(dead_code, reason = "ページとクエリサービスから使うまでの間")]
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
    use chrono::Month;

    use super::*;

    #[test]
    fn from_dates_returns_ongoing_when_end_is_none() {
        let start = YearMonth {
            year: 2026,
            month: Month::January,
        };

        let result = Period::from_dates(start, None);

        assert!(matches!(result, Ok(Period::Ongoing { .. })));
    }

    #[test]
    fn from_dates_returns_past_when_end_is_after_start() {
        let start = YearMonth {
            year: 2025,
            month: Month::March,
        };
        let end = YearMonth {
            year: 2025,
            month: Month::September,
        };

        let result = Period::from_dates(start, Some(end));

        assert!(matches!(result, Ok(Period::Past { .. })));
    }

    #[test]
    fn from_dates_returns_past_when_end_is_same_month_as_start() {
        let start = YearMonth {
            year: 2023,
            month: Month::July,
        };
        let end = YearMonth {
            year: 2023,
            month: Month::July,
        };

        let result = Period::from_dates(start, Some(end));

        assert!(matches!(result, Ok(Period::Past { .. })));
    }

    #[test]
    fn from_dates_returns_past_when_end_is_next_year_with_smaller_month() {
        let start = YearMonth {
            year: 2023,
            month: Month::November,
        };
        let end = YearMonth {
            year: 2024,
            month: Month::February,
        };

        let result = Period::from_dates(start, Some(end));

        assert!(matches!(result, Ok(Period::Past { .. })));
    }

    #[test]
    fn from_dates_fails_when_end_is_before_start() {
        let start = YearMonth {
            year: 2025,
            month: Month::September,
        };
        let end = YearMonth {
            year: 2025,
            month: Month::March,
        };

        let result = Period::from_dates(start, Some(end));

        assert!(matches!(result, Err(PeriodError::EndBeforeStart)));
    }
}
