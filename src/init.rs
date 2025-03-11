mod uv;
mod pwntools;

use core::str;
use std::{ffi::OsString, path::Path};

use clap::Args;
use snafu::{ResultExt, Whatever};
use uv::uv_setup;

use crate::explode_config::ExplodeConfig;

#[derive(Args, Debug)]
pub struct InitArgs {
    #[arg(short, long)]
    /// The target of your exploit. Can be a shell command (e.g. 'python app.py') or a binary
    target: Option<OsString>,
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

    Ok(())
}
