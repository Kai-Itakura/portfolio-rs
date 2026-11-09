use crate::domain::project::work::Work;

/// 案件の種類。実務の案件（`Professional`）と個人開発（`Personal`）の 2 つだけで、
/// ほかの型には実装できない（`sealed::Sealed` を外から実装できないため）。
pub trait ProjectKind: sealed::Sealed {}

/// 業務として取り組んだ案件。作品は持たない。
pub struct Professional;

/// 業務の外で取り組んだ案件。作品を 1 つまで持てる。
pub struct Personal {
    pub work: Option<Work>,
}

impl ProjectKind for Professional {}
impl ProjectKind for Personal {}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::Professional {}
    impl Sealed for super::Personal {}
}
