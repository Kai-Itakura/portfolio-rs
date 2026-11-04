pub mod period;
pub mod personal_project;
pub mod professional_project;
pub mod project_details;
pub mod project_id;
pub mod publication;
pub mod work;

use personal_project::PersonalProject;
use professional_project::ProfessionalProject;
use project_details::{DeleteError, ProjectDetails, PublishError};

/// 1 件の案件。実務の案件か個人開発のどちらか。
pub enum Project {
    Professional(ProfessionalProject),
    Personal(PersonalProject),
}

impl Project {
    pub fn details(&self) -> &ProjectDetails {
        match self {
            Self::Professional(project) => &project.details,
            Self::Personal(project) => &project.details,
        }
    }

    pub fn publish(self) -> Result<Self, PublishError> {
        self.try_map_details(ProjectDetails::publish)
    }

    pub fn unpublish(self) -> Self {
        self.map_details(ProjectDetails::unpublish)
    }

    pub fn mark_removed_upstream(self) -> Self {
        self.map_details(ProjectDetails::mark_removed_upstream)
    }

    pub fn ensure_deletable(&self) -> Result<(), DeleteError> {
        self.details().ensure_deletable()
    }

    /// 中の `ProjectDetails` に `f` を当て、同じ種類の案件に包み直す。
    fn map_details(self, f: impl FnOnce(ProjectDetails) -> ProjectDetails) -> Self {
        match self {
            Self::Professional(project) => Self::Professional(ProfessionalProject {
                details: f(project.details),
            }),
            Self::Personal(project) => Self::Personal(PersonalProject {
                details: f(project.details),
                ..project
            }),
        }
    }

    /// `map_details` の、失敗しうる `f` を受け取る版。
    fn try_map_details<E>(
        self,
        f: impl FnOnce(ProjectDetails) -> Result<ProjectDetails, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::Professional(project) => Self::Professional(ProfessionalProject {
                details: f(project.details)?,
            }),
            Self::Personal(project) => Self::Personal(PersonalProject {
                details: f(project.details)?,
                ..project
            }),
        })
    }
}
