pub mod category;
pub mod technology_id;

use category::Category;
use technology_id::TechnologyId;

#[expect(dead_code, reason = "ページとクエリサービスから使うまでの間")]
pub struct Technology {
    pub id: TechnologyId,
    pub name: String,
    pub category: Category,
}
