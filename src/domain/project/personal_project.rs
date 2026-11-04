use crate::domain::project::project_details::ProjectDetails;
use crate::domain::project::publication::Publication;
use crate::domain::project::work::Work;

/// 業務の外で取り組んだ案件。作品を 1 つまで持てる。
pub struct PersonalProject {
    pub details: ProjectDetails,
    pub work: Option<Work>,
}

#[derive(Debug)]
pub enum AddWorkError {
    RemovedUpstream,
    AlreadyHasWork,
}

impl PersonalProject {
    /// 作品を足す。公開していない個人開発にも足せる（公開するまで作品は見えない）。
    /// 別システムから消えた案件には足せない。
    pub fn add_work(self, work: Work) -> Result<Self, AddWorkError> {
        if self.details.publication == Publication::RemovedUpstream {
            return Err(AddWorkError::RemovedUpstream);
        }
        if self.work.is_some() {
            return Err(AddWorkError::AlreadyHasWork);
        }
        Ok(Self {
            work: Some(work),
            ..self
        })
    }
}
