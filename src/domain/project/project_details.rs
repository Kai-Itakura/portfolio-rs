use std::collections::HashSet;

use crate::domain::project::period::Period;
use crate::domain::project::project_id::ProjectId;
use crate::domain::project::publication::Publication;
use crate::domain::technology::technology_id::TechnologyId;

/// 実務の案件か個人開発かによらず、案件が持つ項目。
pub struct ProjectDetails {
    pub id: ProjectId,
    pub title: String,
    pub description: String,
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

impl ProjectDetails {
    pub fn new(
        id: ProjectId,
        title: String,
        description: String,
        period: Period,
        tech_stack: HashSet<TechnologyId>,
        publication: Publication,
    ) -> Self {
        Self {
            id,
            title,
            description,
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
}
