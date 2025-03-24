#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;
mod outdated;

use cli::CliArgs;
use fancy_regex::Regex;
use medic_lib::std_to_string;
use outdated::OutdatedInfo;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use which::which;

pub fn check_outdated(_args: CliArgs) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("cargo");
    command.args(["outdated", "--format=json"]);

    match command.output() {
        Ok(output) => {
            if output.status.success() {
                let stdout = std_to_string(output.stdout);
                for line in stdout.lines() {
                    let outdated: OutdatedInfo = serde_json::from_str(line)?;
                    for d in &outdated.dependencies {
                        let name_re = Regex::new(r"^((?<parent>.*)->|)(?<name>.+)$").unwrap();

                        let result = name_re.captures(&d.name);
                        let captures = result
                            .expect("Error running regex")
                            .expect("No match found");
                        let name = captures.name("name").unwrap().as_str();

                        if let Some(parent) = captures.name("parent") {
                            println!(
                                "::outdated::name={}::version={}::latest={}::parent={}",
                                name,
                                d.project,
                                d.latest,
                                parent.as_str()
                            );
                        } else {
                            println!(
                                "::outdated::name={}::version={}::latest={}",
                                name, d.project, d.latest
                            );
                        }
                    }
                    if !outdated.dependencies.is_empty() {
                        println!("::remedy::cargo update --verbose")
                    }
                }
            } else {
                return Err("::failure::Unable to get outdated".into());
            }
        }
        Err(_) => return Err("::failure::Unable to get outdated".into()),
    }

    Ok(())
}

pub fn maybe_install_cargo_outdated() -> Result<(), Box<dyn Error>> {
    if which("cargo-outdated").is_err() {
        let mut command = Command::new("cargo");
        command.args(["install", "cargo-outdated", "--color=always"]);
        command.stderr(Stdio::piped()).stdout(Stdio::piped());

        eprintln!("::action::cargo-outdated-install::Installing cargo-outdated");

        let mut child = command.spawn().unwrap();
        let stderr = child.stderr.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        let out_thr = thread::spawn(move || {
            let reader = BufReader::new(stdout);
            reader
                .lines()
                .map_while(Result::ok)
                .for_each(|line| eprintln!("::info::cargo-outdated-install::{}", &line));
        });
        let err_thr = thread::spawn(move || {
            let reader = BufReader::new(stderr);
            reader
                .lines()
                .map_while(Result::ok)
                .for_each(|line| eprintln!("::info::cargo-outdated-install::{}", &line));
        });

        let output = child.wait_with_output();
        out_thr.join().unwrap();
        err_thr.join().unwrap();

        match output {
            Ok(_) => eprintln!("::success::cargo-outdated-install::"),
            Err(_) => {
                eprintln!("::failure::cargo-outdated-install::");
                return Err("Unable to install cargo outdated".into());
            }
        }
    }
    Ok(())
}
