use crate::info::{Exercise, Info};
use crate::runner;
use crate::state::State;
use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "anchorlings", version, about = "rustlings for Anchor (Solana)")]
pub struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run an exercise once (defaults to the current exercise)
    Run { name: Option<String> },
    /// Show all exercises and done/pending status
    List,
    /// Print the hint for an exercise (defaults to current)
    Hint { name: Option<String> },
    /// Mark an exercise as not-done (file restore not yet implemented)
    Reset { name: String },
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let workspace_root = find_workspace_root()?;
        let info = Info::load(&workspace_root)?;
        let mut state = State::load_or_default(&workspace_root)?;

        match self.cmd {
            None => {
                let ex = pick_current(&info, &state)?;
                run_one(&workspace_root, &info, &mut state, &ex.name.clone())
            }
            Some(Cmd::Run { name }) => {
                let name = match name {
                    Some(n) => n,
                    None => pick_current(&info, &state)?.name.clone(),
                };
                run_one(&workspace_root, &info, &mut state, &name)
            }
            Some(Cmd::List) => {
                let current = pick_current(&info, &state).ok().map(|e| e.name.clone());
                let name_width = info.exercises.iter().map(|e| e.name.len()).max().unwrap_or(0);
                let chap_width = info.exercises.iter().map(|e| e.chapter.len()).max().unwrap_or(0);
                for ex in &info.exercises {
                    let mark = if state.is_done(&ex.name) { "✓" } else { " " };
                    let is_current = current.as_deref() == Some(ex.name.as_str());
                    let suffix = if is_current { "  <- current" } else { "" };
                    println!(
                        "[{mark}] {:<cw$}/{:<nw$} {:<6}{suffix}",
                        ex.chapter,
                        ex.name,
                        ex.mode.as_str(),
                        cw = chap_width,
                        nw = name_width,
                    );
                }
                Ok(())
            }
            Some(Cmd::Hint { name }) => {
                let ex = resolve(&info, &state, name.as_deref())?;
                println!("{}", ex.hint.trim());
                Ok(())
            }
            Some(Cmd::Reset { name }) => {
                let ex = info.find(&name).ok_or_else(|| unknown(&name))?;
                state.mark_undone(&ex.name);
                state.current = Some(ex.name.clone());
                state.save(&workspace_root)?;
                println!(
                    "Marked {} as not-done. (File restore from embedded originals \
                     is not implemented yet; revert the source manually for now.)",
                    ex.name
                );
                Ok(())
            }
        }
    }
}

fn run_one(workspace_root: &Path, info: &Info, state: &mut State, name: &str) -> Result<()> {
    let ex = info.find(name).ok_or_else(|| unknown(name))?;
    println!("▶ Running {} ({})", ex.name, ex.chapter);

    let passed = runner::verify(workspace_root, ex)?;
    if passed {
        println!("\n✓ {} passed!", ex.name);
        state.mark_done(&ex.name);

        let next = info.next_after(&ex.name).map(|e| e.name.clone());
        state.current = next.clone();
        state.save(workspace_root)?;

        println!(
            "Solution for comparison: solutions/{}/{}/src/lib.rs",
            ex.chapter, ex.name
        );

        match next {
            Some(n) => println!("Next: {n}"),
            None => println!("\n{}", info.final_message),
        }
    } else {
        println!(
            "\n✗ {} failed. Run `anchorlings hint {}` for a hint.",
            ex.name, ex.name
        );
        state.current = Some(ex.name.clone());
        state.save(workspace_root)?;
    }
    Ok(())
}

fn pick_current<'a>(info: &'a Info, state: &State) -> Result<&'a Exercise> {
    if let Some(c) = &state.current {
        if let Some(ex) = info.find(c) {
            return Ok(ex);
        }
    }
    for ex in &info.exercises {
        if !state.is_done(&ex.name) {
            return Ok(ex);
        }
    }
    info.exercises
        .last()
        .ok_or_else(|| anyhow!("info.toml has no exercises"))
}

fn resolve<'a>(info: &'a Info, state: &State, name: Option<&str>) -> Result<&'a Exercise> {
    match name {
        Some(n) => info.find(n).ok_or_else(|| unknown(n)),
        None => pick_current(info, state),
    }
}

fn unknown(name: &str) -> anyhow::Error {
    anyhow!("no exercise named `{name}` in info.toml")
}

fn find_workspace_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join("info.toml").is_file() && dir.join("Cargo.toml").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            anyhow::bail!(
                "could not find an anchorlings workspace (looked for info.toml + Cargo.toml \
                 walking up from the current directory)"
            );
        }
    }
}
