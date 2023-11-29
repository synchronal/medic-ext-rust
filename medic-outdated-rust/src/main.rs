use medic_outdated_rust::check_outdated;
use medic_outdated_rust::cli::CliArgs;
use medic_outdated_rust::maybe_install_cargo_outdated;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = CliArgs::new();
    maybe_install_cargo_outdated()?;
    check_outdated(cli)
}
