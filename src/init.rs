use std::ffi::OsString;

use clap::Args;

#[derive(Args,Debug)]
pub struct InitArgs {
    #[arg(short, long)]
    /// The target of your exploit. Can be a shell command (e.g. 'python app.py') or a binary
    target: Option<OsString>,
    #[arg(short = 'r', long = "remote", default_value = "localhost")]
    host: String,
    #[arg(short, long, default_value_t = 1337)]
    port: usize,
    #[arg(long, default_value_t = false)]
    ssl: bool
}
