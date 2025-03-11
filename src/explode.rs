use std::{env, path::PathBuf};

use clap::Parser;
use resolve_path::PathResolveExt;

use crate::init::InitArgs;

fn get_default_config_file() -> PathBuf {
    let config_home = env::var("XDG_CONFIG_HOME").map(|str| PathBuf::from(&str)).unwrap_or("~/.config/".resolve().into_owned());
    config_home.join("explode").join("config.toml")
}

#[derive(Parser,Debug)]
pub struct Explode {
    #[arg(short, long, value_name = "CONFIG_FILE", default_value_os_t = get_default_config_file())]
    pub config: PathBuf,

    #[command(flatten)]
    pub init_args: InitArgs
}
