use topcoat::{
    Result,
    asset::{Asset, asset},
    view::{Child, View, component, view},
};

use crate::site;

const FAVICON: Asset = asset!("assets/favicon.png");
const APPLE_TOUCH_ICON: Asset = asset!("assets/apple-touch-icon.png");

#[component]
pub(crate) async fn base_layout(
    #[default]
    #[into]
    title: Option<&str>,
    #[default(site::DESC)] description: &str,
    child: Child<'_>,
) -> Result<impl View> {
    let title = match title {
        Some(page_title) => format!("{page_title} | {}", site::TITLE),
        None => site::TITLE.to_owned(),
    };

    Ok(view! {
        <!DOCTYPE html>
        <html lang=(site::LANG)>
            <head>
                <title>(title.as_str())</title>
                <meta name="description" content=(description) />
                <meta property="og:title" content=(title.as_str()) />
                <meta property="og:description" content=(description) />
                <meta property="og:site_name" content=(site::TITLE) />
                <meta property="og:type" content=(site::OG_TYPE) />
                <meta property="og:locale" content=(site::LOCALE) />
                <meta name="twitter:card" content="summary_large_image" />
                <link rel="icon" href=(FAVICON) />
                <link rel="apple-touch-icon" href=(APPLE_TOUCH_ICON) />
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                topcoat::dev::script()
            </head>
            <body>(child)</body>
        </html>
    })
}
