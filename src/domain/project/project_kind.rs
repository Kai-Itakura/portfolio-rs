use crate::domain::project::period::Period;

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
