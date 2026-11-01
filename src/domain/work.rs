use crate::domain::project::Project;
use crate::domain::project::project_id::ProjectId;
use crate::domain::project::project_kind::ProjectKind;
use crate::domain::published_projects::PublishedProjects;

/// 個人開発を見せるための作品。技術スタックは持たず、参照する個人開発のものを見せる。
pub struct Work {
    pub project: ProjectId,
}

#[derive(Debug)]
pub enum WorkError {
    NotPersonal,
    NotPublished,
}

impl Work {
    pub fn from_project(
        project: &Project,
        published: &PublishedProjects,
    ) -> Result<Self, WorkError> {
        if project.kind != ProjectKind::Personal {
            return Err(WorkError::NotPersonal);
        }
        if !published.contains(project.id) {
            return Err(WorkError::NotPublished);
        }
        Ok(Self {
            project: project.id,
        })
    }
}
