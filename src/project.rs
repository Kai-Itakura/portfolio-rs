use chrono::NaiveDate;

pub enum Period {
    Past { start: NaiveDate, end: NaiveDate },
    Ongoing { start: NaiveDate },
}

pub enum ProjectKind {
    Professional { period: Period },
    Personal,
}

pub struct Project {
    pub title: String,
    pub description: String,
    pub kind: ProjectKind,
}
