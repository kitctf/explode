mod pwntools;
mod uv;

use core::str;
use std::{ffi::OsString, fs, path::Path};

use clap::Args;
use is_executable::IsExecutable;
use pwntools::setup_pwntools_script;
use snafu::{report, ResultExt, Whatever};
use uv::uv_setup;

use crate::explode_config::ExplodeConfig;

#[derive(Args, Debug)]
pub struct InitArgs {
    #[arg(short, long, default_value = guess_target())]
    /// The target of your exploit. Can be a shell command (e.g. 'python app.py') or a binary
    target: String,
    #[arg(short = 'r', long = "remote", default_value = "localhost")]
    host: String,
    #[arg(short, long, default_value_t = 1337)]
    port: usize,
    #[arg(long, default_value_t = false)]
    ssl: bool,
}

pub fn initialize_exploit(
    dir: &Path,
    args: &InitArgs,
    config: &ExplodeConfig,
) -> Result<(), Whatever> {
    uv_setup(
        dir,
        &config
            .templates
            .as_ref()
            .and_then(|templates| templates.pyproject.clone()),
    )
    .whatever_context("Could not setup uv environment")?;

    setup_pwntools_script(dir, config, args)
        .whatever_context("Could not create exploit script template")?;

    Ok(())
}

fn guess_target() -> String {
    let executable = fs::read_dir("./")
        .unwrap()
        .filter_map(|res| res.ok())
        .filter(|entry| entry.path().is_file())
        .find(|entry| entry.path().is_executable())
        .map(|entry| entry.path().to_string_lossy().to_string());

    executable.unwrap_or("unknown".to_string())
}
