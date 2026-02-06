#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_lib::CheckResult::{self, CheckError, CheckOk};
use medic_lib::std_to_string;

use itertools::join;
use regex::Regex;
use std::process::Command;
use which::which;

fn find_missing_crates(stdout: &str, names: &[String]) -> Vec<String> {
    let mut missing = vec![];
    for name in names {
        let pattern = Regex::new(&format!("(?m)^{} v", regex::escape(name))).unwrap();
        if !pattern.is_match(stdout) {
            missing.push(name.clone());
        }
    }
    missing
}

fn find_missing_targets(stdout: &str, targets: &[String]) -> Vec<String> {
    let mut missing = vec![];
    for target in targets {
        let pattern =
            Regex::new(&format!("(?m)^{} \\(installed\\)", regex::escape(target))).unwrap();
        if !pattern.is_match(stdout) {
            missing.push(target.clone());
        }
    }
    missing
}

pub fn cargo_audit() -> CheckResult {
    cargo_exists()?;
    maybe_install_cargo_audit()?;

    match Command::new("cargo")
        .args(["audit", "--color=always"])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                CheckOk
            } else {
                CheckError(
                    "Vulnerable crates detected".into(),
                    Some(std_to_string(output.stdout)),
                    Some(std_to_string(output.stderr)),
                    None,
                )
            }
        }
        Err(err) => CheckError(err.to_string(), None, None, None),
    }
}

pub fn cargo_exists() -> CheckResult {
    if which("cargo").is_err() {
        CheckError("Unable to find cargo in PATH.".into(), None, None, None)
    } else {
        CheckOk
    }
}

pub fn check_formatting() -> CheckResult {
    cargo_exists()?;
    match Command::new("cargo").args(["fmt", "--check"]).output() {
        Ok(command) => match command.status.success() {
            true => CheckOk,
            false => CheckError(
                "Rust project is not correctly formatted".into(),
                Some(std_to_string(command.stdout)),
                Some(std_to_string(command.stderr)),
                Some("cargo fmt".into()),
            ),
        },
        Err(_err) => CheckError(
            "Unable to check for rust formatting. Is `cargo` in PATH?".into(),
            None,
            None,
            None,
        ),
    }
}

pub fn crate_installed(names: Vec<String>) -> CheckResult {
    cargo_exists()?;
    match Command::new("cargo").args(["install", "--list"]).output() {
        Ok(command) => match command.status.success() {
            true => {
                let stdout = std_to_string(command.stdout);
                let missing_crates = find_missing_crates(&stdout, &names);

                if missing_crates.is_empty() {
                    CheckOk
                } else {
                    CheckError(
                        format!(
                            "Rust crates {} do not appear to be installed",
                            join(
                                missing_crates
                                    .iter()
                                    .map(|c| format!("`{c}`"))
                                    .collect::<Vec<_>>(),
                                ", "
                            )
                        ),
                        Some(stdout),
                        Some(std_to_string(command.stderr)),
                        Some(format!(
                            "cargo install --locked {}",
                            join(missing_crates, " ")
                        )),
                    )
                }
            }
            false => CheckError(
                "Unable to check for installed crates. Is cargo in PATH?".into(),
                Some(std_to_string(command.stdout)),
                Some(std_to_string(command.stderr)),
                None,
            ),
        },
        Err(_err) => CheckError(
            "Unable to check for installed crates. Is cargo in PATH?".into(),
            None,
            None,
            None,
        ),
    }
}

pub fn maybe_install_cargo_audit() -> CheckResult {
    if which("cargo-audit").is_err() {
        let mut command = Command::new("cargo");
        command.args(["install", "cargo-audit", "--color=always"]);
        let output = command.output().unwrap();

        if !output.status.success() {
            return CheckError(
                "Error installing cargo-audit".into(),
                Some(std_to_string(output.stdout)),
                Some(std_to_string(output.stderr)),
                None,
            );
        }
    }

    CheckOk
}

pub fn rustup_exists() -> CheckResult {
    if which("rustup").is_err() {
        CheckError("Unable to find rustup in PATH.".into(), None, None, None)
    } else {
        CheckOk
    }
}

pub fn target_installed(targets: Vec<String>) -> CheckResult {
    rustup_exists()?;
    match Command::new("rustup").args(["target", "list"]).output() {
        Ok(command) => match command.status.success() {
            true => {
                let stdout = std_to_string(command.stdout);
                let missing_targets = find_missing_targets(&stdout, &targets);

                if missing_targets.is_empty() {
                    CheckOk
                } else {
                    CheckError(
                        format!(
                            "Rust target {} does not appear to be installed",
                            join(
                                missing_targets
                                    .iter()
                                    .map(|c| format!("`{c}`"))
                                    .collect::<Vec<_>>(),
                                ", "
                            )
                        ),
                        Some(stdout),
                        Some(std_to_string(command.stderr)),
                        Some(format!(
                            "rustup target install {}",
                            join(missing_targets, " ")
                        )),
                    )
                }
            }
            false => CheckError(
                "Unable to check for installed crates. Is cargo in PATH?".into(),
                Some(std_to_string(command.stdout)),
                Some(std_to_string(command.stderr)),
                None,
            ),
        },
        Err(_err) => CheckError(
            "Unable to check for installed crates. Is cargo in PATH?".into(),
            None,
            None,
            None,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::{find_missing_crates, find_missing_targets};

    #[test]
    fn find_missing_crates_returns_empty_when_all_installed() {
        let stdout =
            "cargo-audit v0.18.1:\n    cargo-audit\ncargo-outdated v0.13.1:\n    cargo-outdated\n";
        let names = vec!["cargo-audit".to_string(), "cargo-outdated".to_string()];
        assert_eq!(find_missing_crates(stdout, &names), Vec::<String>::new());
    }

    #[test]
    fn find_missing_crates_returns_missing_crates() {
        let stdout = "cargo-audit v0.18.1:\n    cargo-audit\n";
        let names = vec!["cargo-audit".to_string(), "cargo-outdated".to_string()];
        assert_eq!(find_missing_crates(stdout, &names), vec!["cargo-outdated"]);
    }

    #[test]
    fn find_missing_crates_handles_crate_with_hyphen() {
        let stdout = "my-crate v1.0.0:\n    my-crate\n";
        let names = vec!["my-crate".to_string()];
        assert_eq!(find_missing_crates(stdout, &names), Vec::<String>::new());
    }

    #[test]
    fn find_missing_targets_returns_empty_when_all_installed() {
        let stdout = "aarch64-apple-darwin (installed)\nx86_64-apple-darwin (installed)\n";
        let targets = vec!["aarch64-apple-darwin".to_string()];
        assert_eq!(find_missing_targets(stdout, &targets), Vec::<String>::new());
    }

    #[test]
    fn find_missing_targets_returns_missing_targets() {
        let stdout = "aarch64-apple-darwin (installed)\nx86_64-apple-darwin\n";
        let targets = vec![
            "aarch64-apple-darwin".to_string(),
            "x86_64-apple-darwin".to_string(),
        ];
        assert_eq!(
            find_missing_targets(stdout, &targets),
            vec!["x86_64-apple-darwin"]
        );
    }

    #[test]
    fn find_missing_crates_not_fooled_by_prefix_match() {
        let stdout = "cargo-audit-extended v1.0.0:\n    cargo-audit-extended\n";
        let names = vec!["cargo-audit".to_string()];
        assert_eq!(find_missing_crates(stdout, &names), vec!["cargo-audit"]);
    }
}
