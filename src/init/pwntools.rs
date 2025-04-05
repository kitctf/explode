use std::io::Write;
use std::{fs::OpenOptions, path::Path};

use std::fs::{self, File};
use std::os::fd::AsRawFd;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use snafu::{ResultExt, Whatever};
use tinytemplate::TinyTemplate;

use crate::explode_config::ExplodeConfig;

use super::InitArgs;

#[cfg(target_os = "linux")]
fn make_script_executable(file: &File) -> Result<(), Whatever> {
    use nix::sys::stat::{fchmod, Mode};
    fchmod(file.as_raw_fd(), Mode::S_IRWXU)
        .whatever_context("Could not make exploit script executable")?;

    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn make_script_executable(file: &File) -> Result<(), Whatever> {}

pub fn setup_pwntools_script(
    dir: &Path,
    config: &ExplodeConfig,
    args: &InitArgs,
) -> Result<(), Whatever> {
    let mut script = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join("exploit.py"))
        .whatever_context("Could not create exploit script")?;
    make_script_executable(&script)?;

    let content: String = render_exploit_script(config, args)
        .whatever_context("Could not render exploit template")?;
    script
        .write_all(content.as_bytes())
        .whatever_context("Could not write content to exploit script")?;

    Ok(())
}

const EXPLOIT_TEMPLATE: &'static str = include_str!("../../templates/exploit.py");
const EXPLOIT_TEMPLATE_ID: &'static str = "exploit_script";

#[derive(Debug, Serialize, Deserialize)]
struct ExploitTemplateContext {
    shell_target: bool,
    target: String,
    host: String,
    port: usize,
    ssl: bool,
    udp: bool,
    smt: bool,
    terminal: Option<Vec<String>>,
}

fn format_quoted_with_arrays(value: &Value, output: &mut String) -> tinytemplate::error::Result<()> {
    match value {
        serde_json::Value::Array(values) => {
            output.push('[');
            if let Some((last, start)) = values.split_last() {
                for val in start {
                    format_quoted_with_arrays(val, output)?;
                    output.push_str(", ");
                }
                format_quoted_with_arrays(last, output)?;
            }
            output.push(']');

            Ok(())
        }
        serde_json::Value::String(str) => {
            output.push_str(&format!("\"{str}\""));
            Ok(())
        }
        other => tinytemplate::format(other, output),
    }
}

fn render_exploit_script(config: &ExplodeConfig, args: &InitArgs) -> Result<String, Whatever> {
    let context = ExploitTemplateContext {
        shell_target: args.target.contains(' '),
        target: args.target.clone(),
        host: args.host.clone(),
        port: args.port.clone(),
        ssl: args.ssl,
        udp: args.udp,
        smt: args.smt,
        terminal: config.terminal.clone(),
    };

    let mut tt = TinyTemplate::new();

    tt.set_default_formatter(&format_quoted_with_arrays);

    let content = match &config
        .templates
        .as_ref()
        .and_then(|tmpl| tmpl.exploit.clone())
    {
        Some(path) => fs::read_to_string(path).whatever_context(format!(
            "Can't read custom pyproject template path {path:#?}"
        ))?,
        None => EXPLOIT_TEMPLATE.to_string(),
    };
    tt.add_template(&EXPLOIT_TEMPLATE_ID, &content)
        .whatever_context("Could not add exploit script template")?;

    Ok(tt
        .render(EXPLOIT_TEMPLATE_ID, &context)
        .whatever_context("Could not render pyproject.toml")?)
}
