use topcoat::{
    Result,
    router::{href, page},
    view::{View, view},
};

use crate::components::base_layout::base_layout;

#[page]
pub(crate) async fn about() -> Result<impl View> {
    Ok(view! {
        base_layout(
            title: "About",
            <h1>"About"</h1>
            <a href=(href!(super::home))>"home page"</a>
        )
    })
}
