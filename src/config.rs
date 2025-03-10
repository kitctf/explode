use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExplodeConfig {
    /// The command to open a new terminal with a specified command. Used as pwntools
    /// context.terminal
    terminal: Option<Vec<String>>,
}
