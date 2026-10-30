use ulid::Ulid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TechnologyId(Ulid);

impl From<Ulid> for TechnologyId {
    fn from(ulid: Ulid) -> Self {
        Self(ulid)
    }
}
