mod app;
mod components;
mod project;
mod site;

#[tokio::main]
async fn main() {
    topcoat::start(app::router()).await.unwrap();
}
