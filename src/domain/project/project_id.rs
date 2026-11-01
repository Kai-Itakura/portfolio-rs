use ulid::Ulid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectId(Ulid);

impl From<Ulid> for ProjectId {
    fn from(ulid: Ulid) -> Self {
        Self(ulid)
    }
}
