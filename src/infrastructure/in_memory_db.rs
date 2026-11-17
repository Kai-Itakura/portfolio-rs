use std::collections::HashMap;
use std::sync::RwLock;

use crate::domain::project::AnyProject;
use crate::domain::project::project_id::ProjectId;
use crate::domain::technology::Technology;
use crate::domain::technology::technology_id::TechnologyId;
use crate::infrastructure::v1_data::{self, LoadError};

/// v1 のデータ置き場。本物の DB に替えるまで、案件と技術をメモリ上に持つ。
/// ルーターに `.app_context(...)` で登録して、リポジトリとクエリサービスから使う。
pub struct InMemoryDb {
    pub projects: RwLock<HashMap<ProjectId, AnyProject>>,
    /// 技術は別システムから届くだけで、このサイトは書き換えないのでロックを持たない。
    pub technologies: HashMap<TechnologyId, Technology>,
}

impl InMemoryDb {
    /// `data/v1.jsonc` の v1 のデータを入れた DB を作る。
    pub fn from_v1_data() -> Result<Self, LoadError> {
        let data = v1_data::load()?;
        let projects = data
            .projects
            .into_iter()
            .map(|project| (project_id(&project), project))
            .collect();
        let technologies = data
            .technologies
            .into_iter()
            .map(|technology| (technology.id, technology))
            .collect();
        Ok(Self {
            projects: RwLock::new(projects),
            technologies,
        })
    }
}

fn project_id(project: &AnyProject) -> ProjectId {
    match project {
        AnyProject::Professional(project) => project.id,
        AnyProject::Personal(project) => project.id,
    }
}
