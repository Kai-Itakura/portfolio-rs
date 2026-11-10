use crate::domain::project::work_id::WorkId;

/// 個人開発を見せるための作品。技術スタックは持たず、個人開発のものをそのまま見せる。
/// 期間と言語も個人開発から導くので持たない。
/// カード画像とモックアップの 2 枚の画像を持つが、画像を表す型は未決。
pub struct Work {
    pub id: WorkId,
    pub title: String,
    pub description: String,
    /// 作品が実際に動いている外部のサイト。手元でしか動かない作品は持たない。
    pub live_site: Option<String>,
    /// 作品詳細の本文。Markdown。
    pub body: String,
}

impl Work {
    pub fn new(
        id: WorkId,
        title: String,
        description: String,
        live_site: Option<String>,
        body: String,
    ) -> Self {
        Self {
            id,
            title,
            description,
            live_site,
            body,
        }
    }
}
