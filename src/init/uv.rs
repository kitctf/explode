use core::str;
use std::{
    fs::{self, remove_file, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use snafu::{whatever, OptionExt, ResultExt, Whatever};
use tinytemplate::TinyTemplate;

const PYPROJECT_TEMPLATE: &str = include_str!("../../templates/pyproject.toml");
const PYPROJECT_TEMPLATE_ID: &'static str = "pyproject";

#[derive(Serialize, Deserialize, Debug)]
struct PyprojectTemplateContext {
    project_dir: String,
    smt: bool,
}

pub fn uv_setup(dir: &Path, pyproject_template_path: &Option<PathBuf>, smt: bool) -> Result<(), Whatever> {
    uv_command(dir, "init")?;
    uv_cleanup(dir)?;
    replace_pyproject(dir, pyproject_template_path, smt)?;
    uv_command(dir, "sync")?;

    Ok(())
}

fn uv_command(dir: &Path, command: &str) -> Result<(), Whatever> {
    let uv_command = std::process::Command::new("uv")
        .arg(command)
        .current_dir(dir)
        .output()
        .whatever_context(format!("Could not run 'uv {command}"))?;

    if !uv_command.status.success() {
        whatever!(
            "'uv {command}' failed with exit status {} and stderr output {}",
            uv_command.status,
            str::from_utf8(&uv_command.stderr[..])
                .with_whatever_context(|_| "Can't decode uv stderr")?
        );
    }

    Ok(())
}

fn uv_cleanup(dir: &Path) -> Result<(), Whatever> {
    remove_file(dir.join("README.md")).whatever_context("Could not remove README")?;
    remove_file(dir.join("main.py")).whatever_context("Could not remove main.py")?;

    Ok(())
}

fn replace_pyproject(
    dir: &Path,
    pyproject_template_path: &Option<PathBuf>,
    smt: bool,
) -> Result<(), Whatever> {
    let mut tt = TinyTemplate::new();

    let content = match pyproject_template_path {
        Some(path) => fs::read_to_string(path).whatever_context(format!(
            "Can't read custom pyproject template path {path:#?}"
        ))?,
        None => PYPROJECT_TEMPLATE.to_string(),
    };
    tt.add_template(&PYPROJECT_TEMPLATE_ID, &content)
        .whatever_context("Could not add pyproject.toml template")?;

    let context = PyprojectTemplateContext {
        project_dir: dir
            .to_path_buf()
            .file_name()
            .whatever_context("Could not get context from path")?
            .to_string_lossy()
            .to_string(),
        smt
    };
    let rendered = tt
        .render(PYPROJECT_TEMPLATE_ID, &context)
        .whatever_context("Could not render pyproject.toml")?;

    let mut pyproject = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(dir.join("pyproject.toml"))
        .whatever_context("Could not open pyproject.toml")?;
    pyproject
        .write_all(rendered.as_bytes())
        .whatever_context("Could not write pyproject.toml")?;

    Ok(())
}
