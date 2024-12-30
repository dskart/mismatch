mod api;
mod cmd;

#[tokio::main]
async fn main() {
    std::process::exit(crate::cmd::execute().await)
}
