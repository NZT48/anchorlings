use crate::info::Info;
use crate::runner;
use crate::state::State;
use anyhow::Result;
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use std::collections::HashSet;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

/// Enter watch mode: run the current exercise once, then re-run it on every
/// debounced file change inside `exercises/`. On pass, advance to the next
/// exercise and watch *that* one.
pub fn watch(workspace_root: &Path, info: &Info, state: &mut State) -> Result<()> {
    if state.done.is_empty() {
        println!("{}", info.welcome_message);
    }
    print_header(info, state);

    if let Some(name) = current_name(info, state) {
        run_one(workspace_root, info, state, &name)?;
    }

    let (tx, rx) = mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_secs(1), tx)?;
    let watch_dir = workspace_root.join("exercises");
    debouncer
        .watcher()
        .watch(&watch_dir, RecursiveMode::Recursive)?;

    println!("\n[watching {} — Ctrl+C to quit]", watch_dir.display());

    while let Ok(msg) = rx.recv() {
        let events = match msg {
            Ok(events) => events,
            Err(err) => {
                eprintln!("watch error: {err:?}");
                continue;
            }
        };

        let touched: HashSet<String> = events
            .iter()
            .filter(|e| matches!(e.kind, DebouncedEventKind::Any))
            .filter_map(|e| identify_exercise(&e.path, info, workspace_root))
            .collect();

        if touched.is_empty() {
            continue;
        }

        // Always re-run the *current* exercise — that's the one the learner
        // is supposed to be on. Touching some other exercise's files
        // shouldn't yank attention away.
        let Some(current) = current_name(info, state) else {
            continue;
        };
        if !touched.contains(&current) {
            continue;
        }

        println!("\n--- change detected in {current} ---");
        run_one(workspace_root, info, state, &current)?;
    }
    Ok(())
}

fn run_one(workspace_root: &Path, info: &Info, state: &mut State, name: &str) -> Result<()> {
    let Some(ex) = info.find(name) else { return Ok(()) };
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
        match &next {
            Some(n) => println!("Next: {n} — edit it to continue."),
            None => println!("\n{}", info.final_message),
        }
        print_header(info, state);
    } else {
        println!(
            "\n✗ {} failed. Edit & save to re-run, or `anchorlings hint`.",
            ex.name
        );
        state.current = Some(ex.name.clone());
        state.save(workspace_root)?;
    }
    Ok(())
}

fn current_name(info: &Info, state: &State) -> Option<String> {
    if let Some(c) = &state.current {
        if info.find(c).is_some() {
            return Some(c.clone());
        }
    }
    info.exercises
        .iter()
        .find(|e| !state.is_done(&e.name))
        .map(|e| e.name.clone())
}

fn identify_exercise(path: &Path, info: &Info, workspace_root: &Path) -> Option<String> {
    let rel = path.strip_prefix(workspace_root).ok()?;
    let mut parts = rel.components();
    if parts.next()?.as_os_str() != "exercises" {
        return None;
    }
    let _chapter = parts.next()?;
    let name = parts.next()?.as_os_str().to_str()?.to_string();
    if info.find(&name).is_some() {
        Some(name)
    } else {
        None
    }
}

fn print_header(info: &Info, state: &State) {
    let total = info.exercises.len();
    let done = state.done.len();
    let current = current_name(info, state).unwrap_or_else(|| "(all done)".to_string());
    println!("\n=== Anchorlings — {done}/{total} done — current: {current} ===");
}

