use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExplodeConfig {
    /// The command to open a new terminal with a specified command. Used as pwntools
    /// context.terminal
    pub terminal: Option<Vec<String>>,

    /// Specify custom template files
    pub templates: Option<Templates>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Templates {
    /// Tiny template template file for pyproject.toml
    pub pyproject: Option<PathBuf>
}
