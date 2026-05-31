use crate::info::{Exercise, Mode};
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub fn verify(workspace_root: &Path, exercise: &Exercise) -> Result<bool> {
    if !step(workspace_root, "cargo check", &["check", "-p", &exercise.name])? {
        return Ok(false);
    }

    if exercise.mode == Mode::Test
        && !step(workspace_root, "cargo test", &["test", "-p", &exercise.name])?
    {
        return Ok(false);
    }

    let mut clippy_args: Vec<&str> = vec!["clippy", "-p", &exercise.name];
    if exercise.strict_clippy {
        clippy_args.extend_from_slice(&["--", "-D", "warnings"]);
    }
    if !step(workspace_root, "cargo clippy", &clippy_args)? {
        return Ok(false);
    }

    Ok(true)
}

fn step(workspace_root: &Path, label: &str, args: &[&str]) -> Result<bool> {
    println!("\n→ {label} {}", args[1..].join(" "));
    let status = Command::new("cargo")
        .args(args)
        .current_dir(workspace_root)
        .status()?;
    Ok(status.success())
}
