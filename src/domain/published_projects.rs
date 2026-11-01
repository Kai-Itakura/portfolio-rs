use std::collections::HashSet;

use crate::domain::project::project_id::ProjectId;

/// このサイトで公開する案件の ID。公開しない案件は、年表にも使用件数にも出ない。
pub struct PublishedProjects(HashSet<ProjectId>);

impl PublishedProjects {
    pub fn new(ids: HashSet<ProjectId>) -> Self {
        Self(ids)
    }

    pub fn contains(&self, id: ProjectId) -> bool {
        self.0.contains(&id)
    }
}
