/// このサイトでの案件の公開状態。別システムから消えた案件は、二度と公開しない。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Publication {
    Published,
    Unpublished,
    RemovedUpstream,
}
