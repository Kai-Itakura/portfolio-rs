pub mod period;
pub mod project_id;
pub mod project_kind;

use std::collections::HashSet;

use period::Period;
use project_id::ProjectId;
use project_kind::ProjectKind;

use crate::domain::technology::technology_id::TechnologyId;

pub struct Project {
    pub id: ProjectId,
    pub title: String,
    pub description: String,
    pub kind: ProjectKind,
    pub period: Period,
    pub tech_stack: HashSet<TechnologyId>,
}

impl Project {
    pub fn new(
        id: ProjectId,
        title: String,
        description: String,
        kind: ProjectKind,
        period: Period,
        tech_stack: HashSet<TechnologyId>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            kind,
            period,
            tech_stack,
        }
    }
}
