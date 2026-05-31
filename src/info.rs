use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Info {
    pub format_version: u32,
    pub welcome_message: String,
    pub final_message: String,
    pub exercises: Vec<Exercise>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Exercise {
    pub name: String,
    pub chapter: String,
    pub mode: Mode,
    pub hint: String,
    #[serde(default)]
    pub strict_clippy: bool,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Check,
    Test,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Check => "check",
            Mode::Test => "test",
        }
    }
}

impl Info {
    pub fn load(workspace_root: &Path) -> Result<Self> {
        let path = workspace_root.join("info.toml");
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        let info: Info = toml::from_str(&content).context("parsing info.toml")?;
        if info.format_version != 1 {
            anyhow::bail!("info.toml format_version {} is not supported", info.format_version);
        }
        Ok(info)
    }

    pub fn find(&self, name: &str) -> Option<&Exercise> {
        self.exercises.iter().find(|e| e.name == name)
    }

    pub fn next_after(&self, name: &str) -> Option<&Exercise> {
        let idx = self.exercises.iter().position(|e| e.name == name)?;
        self.exercises.get(idx + 1)
    }
}
