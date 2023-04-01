//! CLI crit tool

extern crate getopts;
extern crate lazy_static;
extern crate pad;
extern crate regex;
extern crate toml;

use pad::PadStr;
use std::cmp;
use std::collections;
use std::env;
use std::fs;
use std::path;
use std::process;

pub static CRIT_ARTIFACT_ROOT : &str = ".crit";

lazy_static::lazy_static! {
    static ref RUSTUP_TARGET_PATTERN : regex::Regex = regex::Regex::new(r"(\S+)").unwrap();

    /// DEFAULT_TARGET_EXCLUSION_PATTERNS collects patterns for problematic target triples,
    /// such as bare metal targets that may lack support for the `std` package,
    /// or targets without community supported cross images.
    static ref DEFAULT_TARGET_EXCLUSION_PATTERNS : regex::Regex = regex::Regex::new(
        &[
            "android",
            "cuda",
            "emscripten",
            "fortanix",
            "fuchsia",
            "gnux32",
            "ios",
            "msvc",
            "none-eabi",
            "pc-solaris",
            "redox",
            "uefi",
            "unknown-none",
            "wasm",
        ].join("|")
    ).unwrap();

    static ref BUILD_MODES : Vec<String> = vec![
        "debug",
        "release",
    ]
        .iter()
        .map(|e| e.to_string())
        .collect();

    static ref BINARY_FILE_EXTENSIONS : Vec<String> = vec![
        "",
        "exe",
        "js",
        "wasm",
    ]
        .iter()
        .map(|e| e.to_string())
        .collect();
}

// Show short CLI spec
fn usage(brief : &str, opts : &getopts::Options) {
    println!("{}", (*opts).usage(brief));
}

/// Show version information
pub fn banner() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

/// CLI entrypoint
fn main() {
    let brief = format!("Usage: {} [OPTIONS] [-- <CROSS OPTIONS>]", env!("CARGO_PKG_NAME"));
    let artifact_root : &path::Path = path::Path::new(CRIT_ARTIFACT_ROOT);
    let mut target_exclusion_pattern : regex::Regex = DEFAULT_TARGET_EXCLUSION_PATTERNS.clone();
    let list_targets : bool;

    let mut rest : Vec<String> = vec!["-r"]
        .iter()
        .map(|e| e.to_string())
        .collect();

    let mut opts : getopts::Options = getopts::Options::new();
    opts.optflag("c", "clean", "delete crit artifacts directory tree");
    opts.optopt("e", "exclude-targets", "exclude targets", "<rust regex>");
    opts.optflag("l", "list-targets", "list enabled targets");
    opts.optflag("h", "help", "print usage info");
    opts.optflag("v", "version", "print version info");

    let arguments : Vec<String> = env::args().collect();

    match opts.parse(&arguments[1..]) {
        Err(_) => {
            usage(&brief, &opts);
            process::exit(1);
        },
        Ok(optmatches) => {
            list_targets = optmatches.opt_present("l");

            if optmatches.opt_present("h") {
                usage(&brief, &opts);
                process::exit(0);
            } else if optmatches.opt_present("v") {
                banner();
                process::exit(0);
            } else if optmatches.opt_present("c") {
                if artifact_root.exists() {
                    _ = fs::remove_dir_all(CRIT_ARTIFACT_ROOT)
                        .expect("error: unable to delete crit artifact root directory");
                }

                process::exit(0);
            } else if optmatches.opt_present("e") {
                let ep = optmatches.opt_str("e")
                    .expect("error: missing exclusion pattern flag value");

                target_exclusion_pattern = regex::Regex::new(&ep)
                    .expect("error: unable to compile Rust regular expression");
            }

            if arguments.contains(&"--".to_string()) {
                rest = optmatches.free;
            }
        }
    }

    let cargo_str : String = fs::read_to_string("Cargo.toml")
        .expect("error: unable to read Cargo.toml");

    let cargo_table : toml::Table = cargo_str.parse::<toml::Table>()
        .expect("error: unable to parse Cargo.toml");

    let bin_tables : &Vec<toml::Value> = cargo_table["bin"]
        .as_array()
        .expect("error: unable to retrieve bin sections from Cargo.toml");

    let mut applications : Vec<&str> = Vec::new();

    for bin_table in bin_tables {
        let application : &str = bin_table["name"]
            .as_str()
            .expect("error: Cargo.toml binary missing name field");

        applications.push(application);
    }

    if applications.is_empty() {
        eprintln!("error: no binaries declared in Cargo.toml")
    }

    let rustup_output : process::Output = process::Command::new("rustup")
        .args(["target", "list"])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .output()
        .expect("error: unable to query rustup for available target triples");

    if !rustup_output.status.success() {
        let rustup_stderr : String = String::from_utf8(rustup_output.stderr)
            .expect("error: unable to read stderr stream from rustup");

        eprintln!("{}", rustup_stderr);
        process::exit(1);
    }

    let rustup_target_text : String = String::from_utf8(rustup_output.stdout)
        .expect("error: unable to read stdout stream from rustup");

    let mut targets : collections::BTreeMap<&str, bool> = collections::BTreeMap::new();

    for line in rustup_target_text.lines() {
        if !RUSTUP_TARGET_PATTERN.is_match(line) {
            continue
        }

        let target : &str = RUSTUP_TARGET_PATTERN
            .captures(line)
            .expect("error: unable to parse target")
            .get(1)
            .expect("error: line missing a target in rustup output")
            .as_str();

        let enabled : bool = !target_exclusion_pattern.is_match(target);
        targets.insert(target, enabled);
    }

    if list_targets {
        let mut max_target_len : usize = 0;

        for target in targets.keys() {
            let target_len : usize = target.len();
            max_target_len = cmp::max(max_target_len, target_len);
        }

        println!("{} {}\n", "TARGET".pad_to_width(max_target_len), "ENABLED");

        for (target, enabled) in targets {
            println!("{} {}", target.pad_to_width(max_target_len), enabled);
        }

        process::exit(0);
    }

    targets.retain(|_, &mut enabled| enabled);

    if targets.is_empty() {
        eprintln!("error: no targets enabled");
        process::exit(1);
    }

    let bin_dir : &path::PathBuf = &artifact_root.join("bin");

    // cross automatically creates its --target-dir paths
    let cross_dir : &path::PathBuf = &artifact_root.join("cross");

    for target in targets.keys() {
        println!("building {}...", target);

        let target_dir_pathbuf : path::PathBuf = cross_dir
            .join(target);
        let target_dir : &str = &target_dir_pathbuf
            .display()
            .to_string();

        let cross_output : process::Output = process::Command::new("cross")
                .args(&["build", "--target-dir", target_dir, "--target", target])
                .args(rest.clone())
                .stdout(process::Stdio::piped())
                .stderr(process::Stdio::piped())
                .output()
                .expect("error: unable to run cross");

        if !cross_output.status.success() {
            let cross_stderr : String = String::from_utf8(cross_output.stderr)
                .expect("error: unable to read stderr stream from cross");

            eprintln!("{}", cross_stderr);
            process::exit(1);
        }

        for mode in BUILD_MODES.iter() {
            for application in &applications {
                let application_dir_pathbuf : path::PathBuf = bin_dir
                    .join(target);

                let application_dir : &str = &application_dir_pathbuf
                    .display()
                    .to_string();

                _ = fs::create_dir_all(application_dir)
                    .expect("error: unable to create bin directory");

                for extension in BINARY_FILE_EXTENSIONS.iter() {
                    let mut application_source_pathbuf : path::PathBuf = target_dir_pathbuf
                        .join(target)
                        .join(mode)
                        .join(application);
                    application_source_pathbuf.set_extension(extension);

                    if application_source_pathbuf.exists() {
                        let application_source_path : &str = &application_source_pathbuf
                            .display()
                            .to_string();

                        let mut application_destination_pathbuf : path::PathBuf = application_dir_pathbuf
                            .join(application);
                        application_destination_pathbuf.set_extension(extension);

                        let application_destination_path : &str = &application_destination_pathbuf
                            .display()
                            .to_string();

                        _ = fs::copy(application_source_path, application_destination_path)
                            .expect("error: unable to copy binary");
                    }
                }
            }
        }
    }
}
