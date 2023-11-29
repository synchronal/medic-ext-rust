use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic-outdated-rust")]
/// Check for outdated crates.
pub struct CliArgs {}

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
