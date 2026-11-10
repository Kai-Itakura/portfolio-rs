use ulid::Ulid;

/// 作品の ID。案件や技術の ID と違い、このサイトが発行する。スラッグと並び順にも使う。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorkId(Ulid);

impl From<Ulid> for WorkId {
    fn from(ulid: Ulid) -> Self {
        Self(ulid)
    }
}
