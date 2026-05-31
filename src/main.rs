mod cli;
mod info;
mod runner;
mod state;

fn main() -> anyhow::Result<()> {
    use clap::Parser;
    cli::Cli::parse().run()
}
