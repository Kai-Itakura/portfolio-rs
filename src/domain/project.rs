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

    pub fn publish(&mut self) -> Result<(), PublishError> {
        if self.publication == Publication::RemovedUpstream {
            return Err(PublishError::RemovedUpstream);
        }
        self.publication = Publication::Published;
        Ok(())
    }

    pub fn unpublish(&mut self) {
        if self.publication == Publication::Published {
            self.publication = Publication::Unpublished;
        }
    }

    /// 別システムから消えた案件に印を付ける。案件と作品は消さずに残し、公開もしない。
    pub fn mark_removed_upstream(&mut self) {
        self.publication = Publication::RemovedUpstream;
    }

    /// このサイトから消してよいかを確かめる。消してよいのは、別システムから消えた案件だけ。
    pub fn ensure_deletable(&self) -> Result<(), DeleteError> {
        if self.publication != Publication::RemovedUpstream {
            return Err(DeleteError::NotRemovedUpstream);
        }
        Ok(())
    }

    pub fn add_work(&mut self, work: Work) -> Result<(), AddWorkError> {
        let ProjectKind::Personal { work: slot } = &mut self.kind else {
            return Err(AddWorkError::NotPersonal);
        };
        if self.publication != Publication::Published {
            return Err(AddWorkError::NotPublished);
        }
        if slot.is_some() {
            return Err(AddWorkError::AlreadyHasWork);
        }
        *slot = Some(work);
        Ok(())
    }
}
