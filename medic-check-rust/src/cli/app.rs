use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic-check-rust")]
/// Checks for ensuring that Rust dependencies are
/// properly installed.
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Runs cargo audit.
    Audit(NoopArgs),
    /// Checks that a crate is installed locally.
    CrateInstalled(CrateArgs),
    /// Checks that a crate is installed locally.
    FormatCheck(NoopArgs),
    /// Checks that a release target is installed locally.
    TargetInstalled(RustupArgs),
}

#[derive(Args, Debug)]
pub struct CrateArgs {
    /// Name of a crate.
    #[clap(value_parser)]
    #[arg(short, long, value_hint = clap::ValueHint::CommandString)]
    pub name: Vec<String>,
}

#[derive(Args, Debug)]
pub struct RustupArgs {
    /// Path to a release target
    #[clap(value_parser)]
    #[arg(short, long, value_hint = clap::ValueHint::CommandString)]
    pub target: Vec<String>,
}

#[derive(Args, Debug)]
pub struct NoopArgs {}

impl Default for CliArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl CliArgs {
    pub fn new() -> Self {
        CliArgs::parse()
    }
}
