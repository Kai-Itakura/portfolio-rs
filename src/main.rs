mod app;
mod components;
mod domain;
mod infrastructure;
mod site;

#[tokio::main]
async fn main() {
    topcoat::start(app::router()).await.unwrap();
}
