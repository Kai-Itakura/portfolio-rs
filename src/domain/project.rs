pub mod period;
pub mod project_id;
pub mod project_kind;
pub mod publication;
pub mod work;

use std::collections::HashSet;

use period::Period;
use project_id::ProjectId;
use project_kind::ProjectKind;
use publication::Publication;
use work::Work;

use crate::domain::technology::technology_id::TechnologyId;

pub struct Project {
    pub id: ProjectId,
    pub title: String,
    pub description: String,
    pub kind: ProjectKind,
    pub period: Period,
    pub tech_stack: HashSet<TechnologyId>,
    pub publication: Publication,
}

#[derive(Debug)]
pub enum PublishError {
    RemovedUpstream,
}

#[derive(Debug)]
pub enum DeleteError {
    NotRemovedUpstream,
}

#[derive(Debug)]
pub enum AddWorkError {
    NotPersonal,
    NotPublished,
    AlreadyHasWork,
}

impl Project {
    pub fn new(
        id: ProjectId,
        title: String,
        description: String,
        kind: ProjectKind,
        period: Period,
        tech_stack: HashSet<TechnologyId>,
        publication: Publication,
    ) -> Self {
        Self {
            id,
            title,
            description,
            kind,
            period,
            tech_stack,
            publication,
        }
    }

    pub fn publish(self) -> Result<Self, PublishError> {
        if self.publication == Publication::RemovedUpstream {
            return Err(PublishError::RemovedUpstream);
        }
        Ok(Self {
            publication: Publication::Published,
            ..self
        })
    }

    pub fn unpublish(self) -> Self {
        let publication = match self.publication {
            Publication::Published => Publication::Unpublished,
            other => other,
        };
        Self {
            publication,
            ..self
        }
    }

    /// 別システムから消えた案件に印を付ける。案件と作品は消さずに残し、公開もしない。
    pub fn mark_removed_upstream(self) -> Self {
        Self {
            publication: Publication::RemovedUpstream,
            ..self
        }
    }

    /// このサイトから消してよいかを確かめる。消してよいのは、別システムから消えた案件だけ。
    pub fn ensure_deletable(&self) -> Result<(), DeleteError> {
        if self.publication != Publication::RemovedUpstream {
            return Err(DeleteError::NotRemovedUpstream);
        }
        Ok(())
    }

    pub fn add_work(self, work: Work) -> Result<Self, AddWorkError> {
        let ProjectKind::Personal { work: existing } = &self.kind else {
            return Err(AddWorkError::NotPersonal);
        };
        if self.publication != Publication::Published {
            return Err(AddWorkError::NotPublished);
        }
        if existing.is_some() {
            return Err(AddWorkError::AlreadyHasWork);
        }
        Ok(Self {
            kind: ProjectKind::Personal { work: Some(work) },
            ..self
        })
    }
}
