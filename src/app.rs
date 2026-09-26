pub(crate) mod about;

use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    router::{Router, RouterBuilderDiscoverExt, href, page},
    view::{View, view},
};

use crate::components::base_layout::base_layout;

pub fn router() -> Router {
    topcoat::router::module_router!()
        .discover()
        .assets(AssetBundle::load().unwrap())
        .build()
}

#[page]
pub(crate) async fn home() -> Result<impl View> {
    Ok(view! {
        base_layout(
            <h1>"Home"</h1>
            <a href=(href!(about::about))>"about page"</a>
        )
    })
}
