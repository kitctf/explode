use core::str;
use std::{ffi::OsString, fs::{self, remove_file, OpenOptions}, io::Write, path::{Path, PathBuf}};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use tinytemplate::TinyTemplate;

const PYPROJECT_TEMPLATE: &str = include_str!("../../templates/pyproject.toml");
const PYPROJECT_TEMPLATE_ID: &'static str = "pyproject";

#[derive(Serialize, Deserialize, Debug)]
struct PyprojectTemplateContext {
    project_dir: String
}

pub fn uv_setup(dir: &Path, pyproject_template_dir: &Option<PathBuf>) -> Result<()> {
    uv_command(dir, "init")?;
    uv_cleanup(dir)?;
    replace_pyproject(dir, pyproject_template_dir)?;
    uv_command(dir, "sync")?;

    Ok(())
}

fn uv_command(dir: &Path, command: &str) -> Result<()> {
    let uv_command = std::process::Command::new("uv")
        .arg(command)
        .current_dir(dir)
        .output()?;

    if !uv_command.status.success() {
        return Err(anyhow!("uv exited with exit status {} and output {}", uv_command.status, str::from_utf8(&uv_command.stderr[..])?));
    }

    Ok(())
}

fn uv_cleanup(dir: &Path) -> Result<()> {
    remove_file(dir.join("README.md"))?;
    remove_file(dir.join("main.py"))?;

    Ok(())
}

fn replace_pyproject(dir: &Path, pyproject_template_dir: &Option<PathBuf>) -> Result<()> {
    let mut tt = TinyTemplate::new();

    let content = match pyproject_template_dir {
        Some(path) => {
            fs::read_to_string(path)?
        },
        None => {
            PYPROJECT_TEMPLATE.to_string()
        }
    };
    tt.add_template(&PYPROJECT_TEMPLATE_ID, &content)?;

    let context = PyprojectTemplateContext {
        project_dir: dir.to_path_buf().file_name().context("Could not get context from path")?.to_string_lossy().to_string()
    };
    let rendered = tt.render(PYPROJECT_TEMPLATE_ID, &context)?;

    let mut pyproject = OpenOptions::new().write(true).truncate(true).open(dir.join("pyproject.toml"))?;
    pyproject.write_all(rendered.as_bytes())?;

    Ok(())
}
