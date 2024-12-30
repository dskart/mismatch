use anyhow::{bail, Result};
use clap::Arg;
use std::{io::IsTerminal, path::Path};
use tracing::error;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod hello_world;
use config::Config;
mod serve;

pub async fn execute() -> i32 {
    let stdin = std::io::stdin();
    let tracing_registry = tracing_subscriber::registry().with(ErrorLayer::default());

    if stdin.is_terminal() {
        tracing_registry
            .with(tracing_subscriber::fmt::layer().pretty())
            .init();
    } else {
        tracing_registry
            .with(tracing_subscriber::fmt::layer().json().flatten_event(true))
            .init();
    }

    // setup logger
    if let Err(e) = set_up_and_exec().await {
        error!("{}", e);
        return 1;
    }
    return 0;
}

pub async fn set_up_and_exec() -> Result<()> {
    let matches = clap::Command::new("my-service")
        .arg_required_else_help(true)
        .about("TODO")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("makes the logs more verbose with infor"),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .help("read configuration from this file"),
        )
        .subcommand(hello_world::cmd())
        .subcommand(serve::cmd())
        .get_matches();

    let config = setup_config(&matches)?;

    return root_cmd(&matches, config).await;
}

fn setup_config(matches: &clap::ArgMatches) -> Result<Config> {
    let config_path = if let Some(config_path) = matches.get_one::<String>("config") {
        if !Path::new(config_path).exists() {
            bail!("config file {} not found", config_path)
        }
        Some(config_path.clone())
    } else {
        const DEFAULT_CONFIG_FILE: &str = "config.yaml";
        if Path::new(DEFAULT_CONFIG_FILE).exists() {
            Some(String::from(DEFAULT_CONFIG_FILE))
        } else {
            None
        }
    };

    let mut config: Config = if let Some(config_path) = config_path {
        let config_yaml = std::fs::read_to_string(config_path)?;
        serde_yaml::from_str(&config_yaml)?
    } else {
        Config::default()
    };

    config.load_from_env("TODO__")?;
    config.validate()?;

    Ok(config)
}

pub async fn root_cmd(matches: &clap::ArgMatches, config: Config) -> Result<()> {
    match matches.subcommand() {
        Some((hello_world::CMD_NAME, sub_match)) => hello_world::run(config, sub_match).await,
        Some((serve::CMD_NAME, sub_match)) => serve::run(config, sub_match).await,
        None => Ok(()),
        _ => unreachable!("match arms should cover all the possible cases"),
    }
}
