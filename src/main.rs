mod app;
mod components;
mod site;

#[tokio::main]
async fn main() {
    topcoat::start(app::router()).await.unwrap();
}
