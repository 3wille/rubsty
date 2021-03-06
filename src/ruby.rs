extern crate itertools;
extern crate regex;

use crate::version::{RubyVersion, VersionLevel, VersionMismatch};
use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn detect_version_mismatches(path: String) -> Vec<VersionMismatch> {
    let paths = fs::read_dir(path).unwrap();
    let versions = parse_files_for_versions(paths);
    let mismatches = build_version_mismatches(versions);
    mismatches
}

fn build_version_mismatches(versions: Vec<RubyVersion>) -> Vec<VersionMismatch> {
    let mut mismatches = Vec::new();
    for mut pair in versions.into_iter().combinations(2) {
        let right = pair.pop().expect("Expect pairs of two");
        let left = pair.pop().expect("Expect pairs of two");
        if let Some(mismatch) = compare_two_versions(left, right) {
            mismatches.push(mismatch);
        }
    }
    mismatches
}

fn compare_two_versions(
    left_version: RubyVersion,
    right_version: RubyVersion,
) -> Option<VersionMismatch> {
    for level in RubyVersion::VERSION_LEVELS.iter() {
        if left_version.on_level(level.clone()) != right_version.on_level(level.clone()) {
            let mismatch = VersionMismatch {
                level: level.clone(),
                versions: vec![left_version, right_version],
            };
            return Some(mismatch);
        }
    }
    None
}

fn parse_files_for_versions(paths: fs::ReadDir) -> Vec<RubyVersion> {
    let mut versions = Vec::new();
    for path in paths {
        let path = path.unwrap();
        let filename = path.file_name().into_string().unwrap();
        let filepath = path.path().display().to_string();
        match filename.as_str() {
            ".ruby-version" => {
                println!("Found .ruby-version");
                let version = process_ruby_version_file(filepath);
                versions.push(version)
            }
            ".tool-versions" => {
                println!("Found .tool-versions");
                match process_tool_versions_file(filepath) {
                    Some(version) => versions.push(version),
                    None => println!("No ruby version defined in .tool-versions"),
                }
            }
            _ => println!("Skipping {}", filepath),
        }
    }
    versions
}

fn process_tool_versions_file(filepath: String) -> Option<RubyVersion> {
    if let Ok(lines) = read_lines(&filepath) {
        for line in lines {
            if let Ok(line) = line {
                println!("{}", line);
                if let Some(version) = process_tool_versions_line(line, &filepath) {
                    return Some(version);
                }
            }
        }
    }
    None
}

fn process_tool_versions_line(line: String, filepath: &String) -> Option<RubyVersion> {
    let version_regex =
        Regex::new(r"^ruby (?P<major>\d+)\.(?P<minor>\d+)\.(?P<teeny>\d+)(-p(?P<patch>\d+))?")
            .unwrap();
    let captures = version_regex.captures(&line);
    if let Some(captures) = captures {
        let version = RubyVersion::from_captures(captures, filepath.clone());
        version.print();
        return Some(version);
    }
    None
}

fn process_ruby_version_file(filepath: String) -> RubyVersion {
    let version_regex =
        Regex::new(r"^(?P<major>\d+)\.(?P<minor>\d+)\.(?P<teeny>\d+)(-p(?P<patch>\d+))?").unwrap();
    let file_content = fs::read_to_string(&filepath).unwrap();
    // println!("{}", file_content);
    let captures = version_regex.captures(&file_content).unwrap();

    let version = RubyVersion::from_captures(captures, filepath.clone());
    version.print();
    version
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_same_versions() {
        let paths = fs::read_dir("./fixtures/same_versions").unwrap();
        let versions = parse_files_for_versions(paths);
        let mismatches = build_version_mismatches(versions);
        assert!(mismatches.is_empty());
    }

    #[test]
    fn test_with_different_version() {
        let paths = fs::read_dir("./fixtures/different_versions").unwrap();
        let versions = parse_files_for_versions(paths);
        let mismatches = build_version_mismatches(versions);
        println!("{}", mismatches.len());
        assert!(mismatches.len() == 1);
    }
}
