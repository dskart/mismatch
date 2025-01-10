use std::sync::Arc;

use crate::cmd::Config;
use crate::{api::Api, app::App};
use anyhow::{Ok, Result};
use clap::{self, value_parser, Arg};
use listenfd::ListenFd;
use tokio::net::TcpListener;
use tracing::info;

pub const CMD_NAME: &str = "serve";

pub fn cmd() -> clap::Command {
    let port_arg = Arg::new("port")
        .long("port")
        .short('p')
        .default_value("8080")
        .value_parser(value_parser!(usize))
        .help("the port for the http api to listen on");

    clap::Command::new(CMD_NAME).arg(port_arg)
}

pub async fn run(config: Config, args: &clap::ArgMatches) -> Result<()> {
    let app = Arc::new(App::new(config.app)?);
    let api = Api::new(config.api, app);

    let router = api.get_router()?;
    let port = args.get_one::<usize>("port").expect("port is required");

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind(format!("127.0.0.1:{}", port)).await?,
    };

    info!(
        "listening on {}",
        listener.local_addr().expect("failed to get local addr")
    );
    axum::serve(listener, router).await?;

    Ok(())
}
