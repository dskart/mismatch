mod api;
mod app;
mod cmd;
mod store;
mod ui;

#[tokio::main]
async fn main() {
    std::process::exit(crate::cmd::execute().await)
}
