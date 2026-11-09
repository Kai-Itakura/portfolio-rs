pub mod period;
pub mod project_id;
pub mod project_kind;
pub mod publication;
pub mod work;

use std::collections::HashSet;

use period::Period;
use project_id::ProjectId;
use project_kind::{Personal, Professional, ProjectKind};
use publication::Publication;
use work::Work;

use crate::domain::technology::technology_id::TechnologyId;

/// 1 件の案件。種類 `K` は `Professional` か `Personal` のどちらか。
pub struct Project<K: ProjectKind> {
    pub id: ProjectId,
    pub title: String,
    pub description: String,
    pub period: Period,
    pub tech_stack: HashSet<TechnologyId>,
    pub publication: Publication,
    pub kind: K,
}

/// 種類がまだ分からない案件。保存先から読んだ案件や、種類の混ざった一覧に使う。
pub enum AnyProject {
    Professional(Project<Professional>),
    Personal(Project<Personal>),
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
    RemovedUpstream,
    AlreadyHasWork,
}

impl<K: ProjectKind> Project<K> {
    pub fn new(
        id: ProjectId,
        title: String,
        description: String,
        period: Period,
        tech_stack: HashSet<TechnologyId>,
        publication: Publication,
        kind: K,
    ) -> Self {
        Self {
            id,
            title,
            description,
            period,
            tech_stack,
            publication,
            kind,
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

impl Project<Personal> {
    /// 作品を足す。公開していない個人開発にも足せる（公開するまで作品は見えない）。
    /// 別システムから消えた案件には足せない。実務の案件にはこのメソッドが無い。
    pub fn add_work(self, work: Work) -> Result<Self, AddWorkError> {
        if self.publication == Publication::RemovedUpstream {
            return Err(AddWorkError::RemovedUpstream);
        }
        if self.kind.work.is_some() {
            return Err(AddWorkError::AlreadyHasWork);
        }
        Ok(Self {
            kind: Personal { work: Some(work) },
            ..self
        })
    }
}
