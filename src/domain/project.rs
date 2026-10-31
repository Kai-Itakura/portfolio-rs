pub mod period;
pub mod project_kind;

use std::collections::HashSet;

use project_kind::ProjectKind;

use crate::domain::technology::technology_id::TechnologyId;

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
