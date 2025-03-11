use std::env;

use anyhow::Result;
use clap::Parser;
use config::Config;
use explode::Explode;
use explode_config::ExplodeConfig;
use init::initialize_exploit;

pub mod explode;
pub mod explode_config;
pub mod init;

fn main() -> Result<()> {
    println!("Hello, world from explode!");

    let explode_command = Explode::parse();

    let config = Config::builder().add_source(config::Environment::with_prefix("EXPLODE"));
    let config = if explode_command.config.try_exists().unwrap_or(false) {
        config.add_source(config::File::from(explode_command.config))
    } else {
        config
    }
    .build()?;

    let config = config.try_deserialize::<ExplodeConfig>()?;
    dbg!(&config);

    initialize_exploit(&env::current_dir()?, &explode_command.init_args, &config)?;

    Ok(())
}
