use crate::domain::project::work::Work;

pub enum ProjectKind {
    Professional,
    Personal { work: Option<Work> },
}
