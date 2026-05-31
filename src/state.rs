use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const STATE_FILENAME: &str = ".anchorlings-state.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub format_version: u32,
    #[serde(default)]
    pub current: Option<String>,
    #[serde(default)]
    pub done: Vec<String>,
}

impl Default for State {
    fn default() -> Self {
        Self { format_version: 1, current: None, done: Vec::new() }
    }
}

impl State {
    pub fn load_or_default(workspace_root: &Path) -> Result<Self> {
        let path = workspace_root.join(STATE_FILENAME);
        if !path.exists() {
            return Ok(State::default());
        }
        let content = std::fs::read_to_string(&path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save(&self, workspace_root: &Path) -> Result<()> {
        let path = workspace_root.join(STATE_FILENAME);
        std::fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn mark_done(&mut self, name: &str) {
        if !self.is_done(name) {
            self.done.push(name.to_string());
        }
    }

    pub fn mark_undone(&mut self, name: &str) {
        self.done.retain(|n| n != name);
    }

    pub fn is_done(&self, name: &str) -> bool {
        self.done.iter().any(|n| n == name)
    }
}
