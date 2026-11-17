use chrono::NaiveDate;

pub enum Period {
    Past { start: NaiveDate, end: NaiveDate },
    Ongoing { start: NaiveDate },
}

#[derive(Debug)]
pub enum PeriodError {
    EndBeforeStart,
}

impl Period {
    pub fn from_dates(start: NaiveDate, end: Option<NaiveDate>) -> Result<Self, PeriodError> {
        match end {
            Some(end) if end < start => Err(PeriodError::EndBeforeStart),
            Some(end) => Ok(Self::Past { start, end }),
            None => Ok(Self::Ongoing { start }),
        }
    }
}
