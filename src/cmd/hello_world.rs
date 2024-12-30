use crate::cmd::Config;
use anyhow::{Ok, Result};
use tracing::info;

pub const CMD_NAME: &str = "hello-world";

pub fn cmd() -> clap::Command {
    clap::Command::new(CMD_NAME).about("says hello")
}

pub async fn run(_config: Config, _args: &clap::ArgMatches) -> Result<()> {
    info!("Hello, world!");
    info!(test = 1, "HELLO");
    Ok(())
}
