use std::collections::HashSet;

use chrono::NaiveDate;

use crate::domain::technology::technology_id::TechnologyId;

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

pub enum ProjectKind {
    Professional { period: Period },
    Personal,
}

impl ProjectKind {
    pub fn from_period(period: Option<Period>) -> Self {
        match period {
            Some(period) => Self::Professional { period },
            None => Self::Personal,
        }
    }
}

pub struct Project {
    pub title: String,
    pub description: String,
    pub kind: ProjectKind,
    pub tech_stack: HashSet<TechnologyId>,
}

impl Project {
    pub fn new(
        title: String,
        description: String,
        kind: ProjectKind,
        tech_stack: HashSet<TechnologyId>,
    ) -> Self {
        Self {
            title,
            description,
            kind,
            tech_stack,
        }
    }
}
