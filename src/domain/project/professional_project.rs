use crate::domain::project::project_details::ProjectDetails;

/// 業務として取り組んだ案件。作品は持たない。
pub struct ProfessionalProject {
    pub details: ProjectDetails,
}
