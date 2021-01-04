use crate::ruby;
use crate::version::*;
use clap::Clap;
use comfy_table::Cell;
use comfy_table::Table;

#[derive(Clap)]
#[clap(version = "0.1", about = "https://github.com/3wille/rubsty")]
struct Opts {
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(about = "Checks for mismatching versions and returns exitcode 1 if any are found")]
    Check(Check),
    #[clap(about = "Checks for mismatching versions")]
    Print(Print),
}

#[derive(Clap)]
struct Check {
    #[clap(
        long,
        default_value = "1",
        about = "Set the returned code in case of mismatching versions"
    )]
    mismatch_returncode: i32,
}
#[derive(Clap)]
struct Print {}

pub fn cli() {
    let opts: Opts = Opts::parse();
    execute_cli(opts);
}

fn execute_cli(opts: Opts) {
    match opts.subcmd {
        Some(SubCommand::Check(check)) => check_command(check),
        Some(SubCommand::Print(_)) => print_command(),
        None => print_command(),
    };
}

fn print_command() {
    let _ = collect_and_print_mismatches();
}

fn check_command(check: Check) {
    let mismatches = collect_and_print_mismatches();
    if mismatches.len() > 0 {
        std::process::exit(check.mismatch_returncode)
    }
}

fn collect_and_print_mismatches() -> Vec<VersionMismatch> {
    let mismatches = ruby::detect_version_mismatches();
    print_mismatches(&mismatches);
    mismatches
}

fn print_mismatches(mismatches: &Vec<VersionMismatch>) {
    let mut table = Table::new();
    table.load_preset(comfy_table::presets::UTF8_FULL);
    for mismatch in mismatches {
        let mut versions = Vec::new();
        let mut locations = Vec::new();
        for version in &mismatch.versions {
            versions.push(format!("{}", version));
            locations.push(format!("{}", version.found_in_file));
        }
        table
            .add_row(vec![Cell::new(mismatch.level.to_string())
                .add_attribute(comfy_table::Attribute::Bold)
                .fg(comfy_table::Color::Red)])
            .add_row(versions)
            .add_row(locations);
    }
    println!("\n{}", table);
}

#[test]
fn test_default_command() {
    let opts = Opts { subcmd: None };
    execute_cli(opts);
}

#[test]
fn test_print_command() {
    let opts = Opts {
        subcmd: Some(SubCommand::Print(Print {})),
    };
    execute_cli(opts);
}

#[test]
fn test_check_command() {
    let opts = Opts {
        subcmd: Some(SubCommand::Check(Check {
            mismatch_returncode: 0,
        })),
    };
    execute_cli(opts);
}
