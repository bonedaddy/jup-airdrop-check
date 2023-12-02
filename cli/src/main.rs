use tokio::io::{BufReader, AsyncBufReadExt};

use crate::types::fetch_allocation;

use {
    anyhow::{anyhow, Result},
    clap::{Arg, ArgMatches, Command},
    config::Configuration,
};

mod types;

#[tokio::main]
pub async fn main() -> Result<()> {
    let matches = Command::new("jup-airdrop-check")
        .arg(config_flag())
        .arg(debug_flag())
        .subcommands(vec![Command::new("config")
            .about("configuration management commands")
            .subcommands(vec![Command::new("new")
                .aliases(["gen", "generate"])
                .about("create and save a new configuration file")
                .arg(keypair_type_flag())])])
        .subcommand(
            Command::new("check-airdrop")
            .arg(
                Arg::new("input-file")
                .long("input-file")
                .help("file containing addresses to check")
            )
        )
        .get_matches();

    let conf_path = matches.get_one::<String>("config").unwrap();
    let debug_log = matches.get_flag("debug");

    utils::init_logger(debug_log);

    process_matches(&matches, conf_path).await?;

    Ok(())
}

async fn process_matches(matches: &ArgMatches, conf_path: &str) -> Result<()> {
    match matches.subcommand() {
        Some(("config", c)) => match c.subcommand() {
            Some(("new", n)) => {
                let cfg = Configuration::new(n.get_one::<String>("keypair-type").unwrap());
                Ok(cfg.save(conf_path)?)
            }
            _ => Err(anyhow!("{INVALID_COMMAND}")),
        },
        Some(("check-airdrop", c)) => {
            let file = tokio::fs::File::open(c.get_one::<String>("input-file").unwrap()).await?;
            // create reader using file
            let reader = BufReader::new(file);
            // get iterator over lines
            let mut lines = reader.lines();
            let mut addrs = Vec::with_capacity(128);
            while let Some(line) = lines.next_line().await.expect("Failed to read file") {
                addrs.push(line);
            }
            addrs.sort_unstable();
            addrs.dedup();
            let mut total_value = 0_u64;
            for addr in addrs {
                let addr = addr.to_ascii_lowercase();
                if let Ok(alloc) = fetch_allocation(&addr).await {
                    println!("{addr} receive {}", alloc.tokens_final);
                    let tkns: u64 = alloc.tokens_final.parse()?;
                    total_value += tkns;
                }
            }
            println!("total tokens received {total_value}");
            Ok(())
        }
        _ => Err(anyhow!("{INVALID_COMMAND}")),
    }
}

fn config_flag() -> Arg {
    Arg::new("config")
        .long("config")
        .help("path to the configuration file")
        .default_value("config.yaml")
}

fn keypair_type_flag() -> Arg {
    Arg::new("keypair-type")
        .long("keypair-type")
        .help("type of keypair we are using")
        .required(true)
}

fn debug_flag() -> Arg {
    Arg::new("debug")
        .long("debug")
        .help("enable debug logging")
        .action(clap::ArgAction::SetTrue)
        .required(false)
}

const INVALID_COMMAND: &str = "invalid command, try running --help";
