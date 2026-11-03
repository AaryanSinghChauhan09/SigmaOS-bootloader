use std::env;
use log::{info, warn, error, debug};
use anyhow::Result;
use clap::Parser;
use bootloader::bootloader;

#[derive(Parser)]
struct Cli;

fn main() -> Result<()> { -> anyhow::Result<()> {
    let config = bootloader::Config::default();
    let mut component = bootloader::new(config);
    component.initialize()?;
    println!("SigmaOS-bootloader ready");
    Ok(())
}

#[derive(Debug, Clone)]
struct Config {
    verbose: bool,
    dry_run: bool,
}

impl Config {
    fn from_env() -> Self {
        Config {
            verbose: env::var("VERBOSE").is_ok(),
            dry_run: env::var("DRY_RUN").is_ok(),
        }
    }
}
