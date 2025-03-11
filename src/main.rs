use clap::Parser;
use config::Config;
use explode::Explode;
use explode_config::ExplodeConfig;
use init::initialize_exploit;
use snafu::{report, ResultExt, Whatever};
use std::env;

pub mod explode;
pub mod explode_config;
pub mod init;

#[report]
fn main() -> Result<(), Whatever> {
    explode()?;
    Ok(())
}

fn explode() -> Result<(), Whatever> {
    let explode_command = Explode::parse();

    let config = Config::builder().add_source(config::Environment::with_prefix("EXPLODE"));
    let config = if explode_command.config.try_exists().unwrap_or(false) {
        config.add_source(config::File::from(explode_command.config))
    } else {
        config
    }
    .build()
    .with_whatever_context(|_| "Could not build config")?;

    let config = config
        .try_deserialize::<ExplodeConfig>()
        .with_whatever_context(|_| "Could not read config")?;
    dbg!(&config);

    initialize_exploit(
        &env::current_dir().with_whatever_context(|_| "Could not get current PWD")?,
        &explode_command.init_args,
        &config,
    )?;

    Ok(())
}
