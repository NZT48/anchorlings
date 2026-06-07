mod cli;
mod info;
mod runner;
mod state;
mod watch;

fn main() -> anyhow::Result<()> {
    use clap::Parser;
    cli::Cli::parse().run()
}
