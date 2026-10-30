pub mod category;
pub mod technology_id;

use category::Category;
use technology_id::TechnologyId;

pub struct Technology {
    pub id: TechnologyId,
    pub name: String,
    pub category: Category,
}
