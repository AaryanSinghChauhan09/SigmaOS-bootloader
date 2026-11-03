use clap::Parser;
use bootloader::bootloader;

#[derive(Parser)]
struct Cli;

fn main() -> anyhow::Result<()> {
    let config = bootloader::Config::default();
    let mut component = bootloader::new(config);
    component.initialize()?;
    println!("SigmaOS-bootloader ready");
    Ok(())
}
